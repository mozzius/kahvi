//! Expressions (ECMA-262 §13), including the cover grammars for arrow
//! parameters and destructuring assignment.

use std::rc::Rc;

use super::{PResult, Parser};
use crate::SyntaxError;
use crate::ast::*;
use crate::lexer::{P, Tok};

/// Constructs that are only valid if the expression is later reinterpreted
/// as a pattern (CoverInitializedName, duplicate `__proto__`).
#[derive(Debug, Default)]
pub(crate) struct Cover {
    pub shorthand_init: Option<usize>,
    pub duplicate_proto: Option<usize>,
}

impl Cover {
    pub fn check(&self, p: &Parser) -> PResult<()> {
        if let Some(pos) = self.shorthand_init {
            return p.err("invalid shorthand property initializer", pos);
        }
        if let Some(pos) = self.duplicate_proto {
            return p.err("duplicate __proto__ property in object literal", pos);
        }
        Ok(())
    }

    fn merge(&mut self, other: Cover) {
        self.shorthand_init = self.shorthand_init.or(other.shorthand_init);
        self.duplicate_proto = self.duplicate_proto.or(other.duplicate_proto);
    }
}

fn binary_precedence(tok: &crate::lexer::Token, allow_in: bool) -> Option<(u8, BinOrLogical)> {
    use BinaryOp::*;
    let op = match tok.kind {
        Tok::Punct(p) => match p {
            P::QQ => return Some((1, BinOrLogical::Logical(LogicalOp::Coalesce))),
            P::PipePipe => return Some((2, BinOrLogical::Logical(LogicalOp::Or))),
            P::AmpAmp => return Some((3, BinOrLogical::Logical(LogicalOp::And))),
            P::Pipe => (4, BitOr),
            P::Caret => (5, BitXor),
            P::Amp => (6, BitAnd),
            P::EqEq => (7, Eq),
            P::NotEq => (7, NotEq),
            P::EqEqEq => (7, StrictEq),
            P::NotEqEq => (7, StrictNotEq),
            P::Lt => (8, Lt),
            P::Gt => (8, Gt),
            P::LtEq => (8, LtEq),
            P::GtEq => (8, GtEq),
            P::Shl => (9, Shl),
            P::Shr => (9, Shr),
            P::UShr => (9, UShr),
            P::Plus => (10, Add),
            P::Minus => (10, Sub),
            P::Star => (11, Mul),
            P::Slash => (11, Div),
            P::Percent => (11, Mod),
            P::StarStar => (12, Exp),
            _ => return None,
        },
        Tok::Ident if !tok.escaped && tok.ident == "instanceof" => (8, InstanceOf),
        Tok::Ident if !tok.escaped && tok.ident == "in" && allow_in => (8, In),
        _ => return None,
    };
    Some((op.0, BinOrLogical::Binary(op.1)))
}

#[derive(Clone, Copy)]
enum BinOrLogical {
    Binary(BinaryOp),
    Logical(LogicalOp),
}

fn assign_op(p: P) -> Option<AssignOp> {
    use BinaryOp::*;
    Some(match p {
        P::Eq => AssignOp::Assign,
        P::PlusEq => AssignOp::Binary(Add),
        P::MinusEq => AssignOp::Binary(Sub),
        P::StarEq => AssignOp::Binary(Mul),
        P::SlashEq => AssignOp::Binary(Div),
        P::PercentEq => AssignOp::Binary(Mod),
        P::StarStarEq => AssignOp::Binary(Exp),
        P::ShlEq => AssignOp::Binary(Shl),
        P::ShrEq => AssignOp::Binary(Shr),
        P::UShrEq => AssignOp::Binary(UShr),
        P::AmpEq => AssignOp::Binary(BitAnd),
        P::PipeEq => AssignOp::Binary(BitOr),
        P::CaretEq => AssignOp::Binary(BitXor),
        P::AmpAmpEq => AssignOp::Logical(LogicalOp::And),
        P::PipePipeEq => AssignOp::Logical(LogicalOp::Or),
        P::QQEq => AssignOp::Logical(LogicalOp::Coalesce),
        _ => return None,
    })
}

impl Parser<'_> {
    /// Runs `f` with the [In] grammar parameter set to `allow`.
    pub fn with_in<T>(
        &mut self,
        allow: bool,
        f: impl FnOnce(&mut Self) -> PResult<T>,
    ) -> PResult<T> {
        let saved = std::mem::replace(&mut self.allow_in, allow);
        let r = f(self);
        self.allow_in = saved;
        r
    }

    pub fn parse_expression_allow_in(&mut self) -> PResult<Expr> {
        self.with_in(true, |p| p.parse_expression())
    }

    /// Expression: comma-separated AssignmentExpressions.
    pub fn parse_expression(&mut self) -> PResult<Expr> {
        let mut cover = Cover::default();
        let e = self.parse_expression_cover(&mut cover)?;
        cover.check(self)?;
        Ok(e)
    }

    pub fn parse_expression_cover(&mut self, cover: &mut Cover) -> PResult<Expr> {
        let start = self.start();
        let first = self.parse_assignment_cover(cover)?;
        if !self.is(P::Comma) {
            return Ok(first);
        }
        let mut items = vec![first];
        while self.eat(P::Comma)? {
            items.push(self.parse_assignment_cover(cover)?);
        }
        Ok(Expr::Sequence(items, self.span_from(start)))
    }

    /// AssignmentExpression.
    pub fn parse_assignment(&mut self) -> PResult<Expr> {
        let mut cover = Cover::default();
        let e = self.parse_assignment_cover(&mut cover)?;
        cover.check(self)?;
        Ok(e)
    }

    pub fn parse_assignment_cover(&mut self, cover: &mut Cover) -> PResult<Expr> {
        let start = self.start();
        if self.is_word("yield") && self.in_generator() {
            return self.parse_yield();
        }
        // Arrow functions with a single unparenthesized parameter.
        if self.tok.kind == Tok::Ident && self.peek()?.is(P::Arrow) {
            if self.peek()?.nl_before {
                return self.unexpected();
            }
            let tok = self.next()?;
            let param = Ident {
                name: tok.ident.clone(),
                span: tok.span(),
            };
            return self.parse_arrow_from_params(
                start,
                vec![Param {
                    target: Pattern::Ident(param),
                    default: None,
                }],
                None,
                false,
            );
        }
        if self.is_word("async") && !self.peek()?.nl_before {
            let next = self.peek()?;
            if next.kind == Tok::Ident && !(next.ident == "function" && !next.escaped) {
                // `async x => ...`
                let mut lx = self.lx.clone();
                lx.next_token()?;
                let after = lx.next_token()?;
                if after.is(P::Arrow) && !after.nl_before {
                    self.next()?;
                    let tok = self.next()?;
                    let param = Ident {
                        name: tok.ident.clone(),
                        span: tok.span(),
                    };
                    if tok.ident == "await" {
                        return self.err(
                            "'await' is not a valid parameter name in an async function",
                            tok.start,
                        );
                    }
                    return self.parse_arrow_from_params(
                        start,
                        vec![Param {
                            target: Pattern::Ident(param),
                            default: None,
                        }],
                        None,
                        true,
                    );
                }
            }
        }

        let mut inner = Cover::default();
        let left = self.parse_conditional(&mut inner)?;
        if let Some(op) = self.tok_assign_op()
            && !matches!(left, Expr::Arrow(_))
        {
            let op_pos = self.tok.start;
            self.next()?;
            let target = if op == AssignOp::Assign {
                // Object/array literals become destructuring patterns; their
                // cover-only constructs are fine there.
                self.expr_to_assignment_target(left, true)?
            } else {
                cover.merge(inner);
                let t = self.expr_to_assignment_target(left, false)?;
                if !matches!(t, Pattern::Ident(_) | Pattern::Expr(_)) {
                    return self.err("invalid compound assignment target", op_pos);
                }
                t
            };
            let value = self.parse_assignment_cover(cover)?;
            return Ok(Expr::Assign {
                op,
                target: Box::new(target),
                value: Box::new(value),
                span: self.span_from(start),
            });
        }
        cover.merge(inner);
        Ok(left)
    }

    fn tok_assign_op(&self) -> Option<AssignOp> {
        match self.tok.kind {
            Tok::Punct(p) => assign_op(p),
            _ => None,
        }
    }

    fn parse_yield(&mut self) -> PResult<Expr> {
        let start = self.start();
        if self.fn_ctx().in_params {
            return self.err("yield expression not allowed in formal parameters", start);
        }
        self.fn_ctx_mut().yield_or_await_pos.get_or_insert(start);
        self.next()?;
        if self.tok.nl_before {
            return Ok(Expr::Yield {
                argument: None,
                delegate: false,
                span: self.span_from(start),
            });
        }
        let delegate = self.eat(P::Star)?;
        let ends = matches!(
            self.tok.kind,
            Tok::Eof
                | Tok::Punct(
                    P::RParen
                        | P::RBracket
                        | P::RBrace
                        | P::Comma
                        | P::Semi
                        | P::Colon
                        | P::Question
                )
        ) || self.is_word("in")
            || self.is_word("of");
        let argument = if delegate || !ends {
            Some(Box::new(self.parse_assignment()?))
        } else {
            None
        };
        Ok(Expr::Yield {
            argument,
            delegate,
            span: self.span_from(start),
        })
    }

    fn parse_conditional(&mut self, cover: &mut Cover) -> PResult<Expr> {
        let start = self.start();
        let test = self.parse_binary(0, cover)?;
        if matches!(test, Expr::Arrow(_)) || !self.is(P::Question) {
            return Ok(test);
        }
        self.next()?;
        let consequent = self.with_in(true, |p| p.parse_assignment())?;
        self.expect(P::Colon)?;
        let alternate = self.parse_assignment()?;
        Ok(Expr::Conditional {
            test: Box::new(test),
            consequent: Box::new(consequent),
            alternate: Box::new(alternate),
            span: self.span_from(start),
        })
    }

    /// Precedence climbing over binary and logical operators.
    fn parse_binary(&mut self, min_prec: u8, cover: &mut Cover) -> PResult<Expr> {
        let start = self.start();
        let mut left = if self.tok.kind == Tok::PrivateName {
            // `#x in obj`
            let tok = self.next()?;
            if !self.is_word("in") || !self.allow_in || min_prec > 8 {
                return self.err("unexpected private name", tok.start);
            }
            self.reference_private(&tok.ident, tok.start)?;
            self.next()?;
            let right = self.parse_binary(9, cover)?;
            Expr::PrivateIn(tok.ident, Box::new(right), self.span_from(start))
        } else {
            self.parse_unary(cover)?
        };
        if matches!(left, Expr::Arrow(_)) {
            return Ok(left);
        }
        while let Some((prec, op)) = binary_precedence(&self.tok, self.allow_in) {
            if prec <= min_prec && !(prec == 12 && min_prec == 12) {
                break;
            }
            if prec == 12 && matches!(left, Expr::Unary(..) | Expr::Await(..)) {
                return self.err(
                    "unary expression before '**' must be parenthesized",
                    self.tok.start,
                );
            }
            self.next()?;
            // `**` is right-associative.
            let right = if prec == 12 {
                self.parse_binary(11, cover)?
            } else {
                self.parse_binary(prec, cover)?
            };
            let span = self.span_from(start);
            left = match op {
                BinOrLogical::Binary(b) => Expr::Binary(b, Box::new(left), Box::new(right), span),
                BinOrLogical::Logical(l) => {
                    let mixes = |e: &Expr, other: LogicalOp| matches!(e, Expr::Logical(o, ..) if (*o == LogicalOp::Coalesce) != (other == LogicalOp::Coalesce));
                    if mixes(&left, l) || mixes(&right, l) {
                        return self.err(
                            "cannot mix '??' with '&&' or '||' without parentheses",
                            span.start as usize,
                        );
                    }
                    Expr::Logical(l, Box::new(left), Box::new(right), span)
                }
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self, cover: &mut Cover) -> PResult<Expr> {
        let start = self.start();
        let op = match self.tok.kind {
            Tok::Punct(P::Plus) => Some(UnaryOp::Plus),
            Tok::Punct(P::Minus) => Some(UnaryOp::Minus),
            Tok::Punct(P::Bang) => Some(UnaryOp::Not),
            Tok::Punct(P::Tilde) => Some(UnaryOp::BitNot),
            Tok::Ident if !self.tok.escaped => match self.tok.ident.as_str() {
                "delete" => Some(UnaryOp::Delete),
                "void" => Some(UnaryOp::Void),
                "typeof" => Some(UnaryOp::Typeof),
                _ => None,
            },
            _ => None,
        };
        if let Some(op) = op {
            self.next()?;
            let arg = self.parse_unary(&mut Cover::default())?;
            if op == UnaryOp::Delete {
                if self.strict && matches!(arg.unparen(), Expr::Ident(_)) {
                    return self.err(
                        "cannot delete an unqualified identifier in strict mode",
                        start,
                    );
                }
                if is_private_reference(arg.unparen()) {
                    return self.err("cannot delete a private field", start);
                }
            }
            return Ok(Expr::Unary(op, Box::new(arg), self.span_from(start)));
        }
        if self.is_word("await") && self.await_should_parse_expression() {
            if self.fn_ctx().in_params {
                return self.err("await expression not allowed in formal parameters", start);
            }
            if self.fn_ctx().in_static_block {
                return self.err("await is not allowed in class static blocks", start);
            }
            self.fn_ctx_mut().yield_or_await_pos.get_or_insert(start);
            self.next()?;
            let arg = self.parse_unary(&mut Cover::default())?;
            return Ok(Expr::Await(Box::new(arg), self.span_from(start)));
        }
        if self.is(P::PlusPlus) || self.is(P::MinusMinus) {
            let op = if self.is(P::PlusPlus) {
                UpdateOp::Increment
            } else {
                UpdateOp::Decrement
            };
            self.next()?;
            let target = self.parse_unary(&mut Cover::default())?;
            self.check_simple_target(&target)?;
            return Ok(Expr::Update {
                op,
                prefix: true,
                target: Box::new(target),
                span: self.span_from(start),
            });
        }
        let expr = self.parse_lhs(cover)?;
        if (self.is(P::PlusPlus) || self.is(P::MinusMinus)) && !self.tok.nl_before {
            let op = if self.is(P::PlusPlus) {
                UpdateOp::Increment
            } else {
                UpdateOp::Decrement
            };
            self.check_simple_target(&expr)?;
            self.next()?;
            return Ok(Expr::Update {
                op,
                prefix: false,
                target: Box::new(expr),
                span: self.span_from(start),
            });
        }
        Ok(expr)
    }

    /// `await` starts an AwaitExpression in async code (and is an error
    /// where it is reserved but no expression is allowed).
    fn await_should_parse_expression(&self) -> bool {
        self.in_async() || self.fn_ctx().in_static_block
    }

    /// The target of `++`/`--` (and compound assignment) must be simple.
    pub fn check_simple_target(&self, e: &Expr) -> PResult<()> {
        match e.unparen() {
            Expr::Ident(id) => {
                if self.strict && (id.name == "eval" || id.name == "arguments") {
                    return self.err(
                        format!("cannot assign to '{}' in strict mode", id.name),
                        id.span.start as usize,
                    );
                }
                Ok(())
            }
            Expr::Member { .. } | Expr::SuperMember(..) => Ok(()),
            other => self.err("invalid assignment target", other.span().start as usize),
        }
    }

    /// LeftHandSideExpression: NewExpression or CallExpression, with
    /// optional chains.
    pub(crate) fn parse_lhs(&mut self, cover: &mut Cover) -> PResult<Expr> {
        let start = self.start();
        let base = if self.is_word("new") {
            self.parse_new()?
        } else if self.is_word("super") {
            self.parse_super()?
        } else if self.is_word("import") {
            self.parse_import_expression()?
        } else {
            let e = self.parse_primary(cover)?;
            if matches!(e, Expr::Arrow(_)) {
                return Ok(e);
            }
            e
        };
        self.parse_call_tail(start, base, true)
    }

    /// Member accesses, calls, tagged templates and optional chains after `base`.
    fn parse_call_tail(&mut self, start: usize, mut expr: Expr, allow_call: bool) -> PResult<Expr> {
        let mut in_chain = false;
        loop {
            match self.tok.kind {
                Tok::Punct(P::Dot) => {
                    self.next()?;
                    let property = self.parse_member_name()?;
                    expr = Expr::Member {
                        object: Box::new(expr),
                        property,
                        optional: false,
                        span: self.span_from(start),
                    };
                }
                Tok::Punct(P::QDot) => {
                    if !allow_call {
                        return self.err(
                            "optional chain is not allowed in a new expression",
                            self.tok.start,
                        );
                    }
                    self.next()?;
                    in_chain = true;
                    expr = match self.tok.kind {
                        Tok::Punct(P::LParen) => {
                            let args = self.parse_arguments()?;
                            Expr::Call {
                                callee: Box::new(expr),
                                args,
                                optional: true,
                                span: self.span_from(start),
                            }
                        }
                        Tok::Punct(P::LBracket) => {
                            self.next()?;
                            let prop = self.parse_expression_allow_in()?;
                            self.expect(P::RBracket)?;
                            Expr::Member {
                                object: Box::new(expr),
                                property: MemberProp::Computed(Box::new(prop)),
                                optional: true,
                                span: self.span_from(start),
                            }
                        }
                        Tok::Template => {
                            return self.err("tagged template in optional chain", self.tok.start);
                        }
                        _ => {
                            let property = self.parse_member_name()?;
                            Expr::Member {
                                object: Box::new(expr),
                                property,
                                optional: true,
                                span: self.span_from(start),
                            }
                        }
                    };
                }
                Tok::Punct(P::LBracket) => {
                    self.next()?;
                    let prop = self.parse_expression_allow_in()?;
                    self.expect(P::RBracket)?;
                    expr = Expr::Member {
                        object: Box::new(expr),
                        property: MemberProp::Computed(Box::new(prop)),
                        optional: false,
                        span: self.span_from(start),
                    };
                }
                Tok::Template => {
                    if in_chain {
                        return self.err("tagged template in optional chain", self.tok.start);
                    }
                    let quasi = self.parse_template(true)?;
                    expr = Expr::TaggedTemplate {
                        tag: Box::new(expr),
                        quasi,
                        span: self.span_from(start),
                    };
                }
                Tok::Punct(P::LParen) if allow_call => {
                    let direct_eval = matches!(&expr, Expr::Ident(id) if id.name == "eval");
                    let args = self.parse_arguments()?;
                    if direct_eval {
                        let ctx = self.fn_ctx_mut();
                        ctx.has_direct_eval = true;
                        ctx.uses_arguments = true;
                        ctx.uses_this = true;
                    }
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                        optional: false,
                        span: self.span_from(start),
                    };
                }
                _ => break,
            }
        }
        if in_chain {
            expr = Expr::OptionalChain(Box::new(expr), self.span_from(start));
        }
        Ok(expr)
    }

    fn parse_member_name(&mut self) -> PResult<MemberProp> {
        match self.tok.kind {
            Tok::Ident => {
                let t = self.next()?;
                Ok(MemberProp::Ident(t.ident.clone(), t.span()))
            }
            Tok::PrivateName => {
                let t = self.next()?;
                self.reference_private(&t.ident, t.start)?;
                Ok(MemberProp::Private(t.ident.clone(), t.span()))
            }
            _ => self.unexpected(),
        }
    }

    pub fn parse_arguments(&mut self) -> PResult<Vec<Arg>> {
        self.expect(P::LParen)?;
        let mut args = Vec::new();
        self.with_in(true, |p| {
            while !p.is(P::RParen) {
                if p.eat(P::Ellipsis)? {
                    args.push(Arg::Spread(p.parse_assignment()?));
                } else {
                    args.push(Arg::Expr(p.parse_assignment()?));
                }
                if !p.eat(P::Comma)? {
                    break;
                }
            }
            p.expect(P::RParen)?;
            Ok(())
        })?;
        Ok(args)
    }

    fn parse_new(&mut self) -> PResult<Expr> {
        let start = self.start();
        self.next()?; // new
        if self.eat(P::Dot)? {
            if !self.is_word("target") {
                return self.unexpected();
            }
            self.next()?;
            if !self.fn_ctx().allow_new_target {
                return self.err("new.target is only allowed in functions", start);
            }
            return Ok(Expr::NewTarget(self.span_from(start)));
        }
        let callee_start = self.start();
        let callee = if self.is_word("import") {
            if !self.peek()?.is(P::Dot) {
                return self.err("cannot use new with import()", self.tok.start);
            }
            self.parse_import_expression()?
        } else if self.is_word("new") {
            self.parse_new()?
        } else if self.is_word("super") {
            self.parse_super()?
        } else {
            let mut cover = Cover::default();
            let e = self.parse_primary(&mut cover)?;
            cover.check(self)?;
            if matches!(e, Expr::Arrow(_)) {
                return self.err("cannot use new with an arrow function", callee_start);
            }
            e
        };
        let callee = self.parse_call_tail(callee_start, callee, false)?;
        let args = if self.is(P::LParen) {
            self.parse_arguments()?
        } else {
            Vec::new()
        };
        Ok(Expr::New {
            callee: Box::new(callee),
            args,
            span: self.span_from(start),
        })
    }

    fn parse_super(&mut self) -> PResult<Expr> {
        let start = self.start();
        self.next()?;
        match self.tok.kind {
            Tok::Punct(P::LParen) => {
                if !self.fn_ctx().allow_super_call {
                    return self.err(
                        "'super' call is only allowed in derived class constructors",
                        start,
                    );
                }
                let args = self.parse_arguments()?;
                self.fn_ctx_mut().uses_this = true;
                Ok(Expr::SuperCall(args, self.span_from(start)))
            }
            Tok::Punct(P::Dot) | Tok::Punct(P::LBracket) => {
                if !self.fn_ctx().allow_super_property {
                    return self.err("'super' property access is only allowed in methods", start);
                }
                self.fn_ctx_mut().uses_this = true;
                let prop = if self.eat(P::Dot)? {
                    match self.tok.kind {
                        Tok::Ident => {
                            let t = self.next()?;
                            MemberProp::Ident(t.ident.clone(), t.span())
                        }
                        _ => return self.unexpected(),
                    }
                } else {
                    self.next()?;
                    let e = self.parse_expression_allow_in()?;
                    self.expect(P::RBracket)?;
                    MemberProp::Computed(Box::new(e))
                };
                Ok(Expr::SuperMember(prop, self.span_from(start)))
            }
            _ => self.err(
                "'super' must be followed by a call or property access",
                start,
            ),
        }
    }

    fn parse_import_expression(&mut self) -> PResult<Expr> {
        let start = self.start();
        self.next()?; // import
        if self.eat(P::Dot)? {
            if !self.is_word("meta") {
                return self.unexpected();
            }
            self.next()?;
            if !self.module {
                return self.err("import.meta is only allowed in modules", start);
            }
            return Ok(Expr::ImportMeta(self.span_from(start)));
        }
        if !self.is(P::LParen) {
            return self.unexpected();
        }
        self.next()?;
        let (specifier, options) = self.with_in(true, |p| {
            let specifier = p.parse_assignment()?;
            let mut options = None;
            if p.eat(P::Comma)? && !p.is(P::RParen) {
                options = Some(Box::new(p.parse_assignment()?));
                p.eat(P::Comma)?;
            }
            p.expect(P::RParen)?;
            Ok((specifier, options))
        })?;
        Ok(Expr::Import {
            specifier: Box::new(specifier),
            options,
            span: self.span_from(start),
        })
    }

    /// PrimaryExpression.
    fn parse_primary(&mut self, cover: &mut Cover) -> PResult<Expr> {
        let start = self.start();
        match self.tok.kind {
            Tok::Ident => {
                if !self.tok.escaped {
                    match self.tok.ident.as_str() {
                        "this" => {
                            self.next()?;
                            self.fn_ctx_mut().uses_this = true;
                            return Ok(Expr::This(self.span_from(start)));
                        }
                        "null" => {
                            self.next()?;
                            return Ok(Expr::Null(self.span_from(start)));
                        }
                        "true" | "false" => {
                            let t = self.next()?;
                            return Ok(Expr::Bool(t.ident == "true", self.span_from(start)));
                        }
                        "function" => {
                            return Ok(Expr::Function(Rc::new(
                                self.parse_function(start, false, false, false)?,
                            )));
                        }
                        "class" => return Ok(Expr::Class(self.parse_class(false)?)),
                        "async" => {
                            let next = self.peek()?;
                            if next.kind == Tok::Ident
                                && !next.escaped
                                && next.ident == "function"
                                && !next.nl_before
                            {
                                self.next()?;
                                return Ok(Expr::Function(Rc::new(
                                    self.parse_function(start, true, false, false)?,
                                )));
                            }
                            if next.is(P::LParen) && !next.nl_before {
                                return self.parse_async_call_or_arrow(cover);
                            }
                        }
                        "let" if self.strict => {
                            return self.err("'let' is reserved in strict mode", start);
                        }
                        _ => {}
                    }
                }
                let tok = self.next()?;
                if tok.escaped
                    && crate::RESERVED_WORDS.contains(&tok.ident.as_str())
                    && !matches!(tok.ident.as_str(), "yield" | "await")
                {
                    return self.err("keywords cannot contain escape characters", tok.start);
                }
                self.check_identifier(&tok.ident, tok.start, false)?;
                if tok.ident == "arguments" {
                    if !self.fn_ctx().allow_arguments {
                        return self.err("'arguments' is not allowed in class field initializers or static blocks", tok.start);
                    }
                    self.fn_ctx_mut().uses_arguments = true;
                }
                Ok(Expr::Ident(Ident {
                    span: tok.span(),
                    name: tok.ident,
                }))
            }
            Tok::Number => {
                let t = self.next()?;
                if t.legacy_octal && self.strict {
                    return self.err(
                        "legacy octal literals are not allowed in strict mode",
                        t.start,
                    );
                }
                Ok(Expr::Number(t.num, t.span()))
            }
            Tok::BigInt => {
                let t = self.next()?;
                Ok(Expr::BigInt(t.bigint.clone(), t.span()))
            }
            Tok::String => {
                let t = self.next()?;
                if t.legacy_octal && self.strict {
                    return self.err(
                        "octal escape sequences are not allowed in strict mode",
                        t.start,
                    );
                }
                Ok(Expr::String(t.str_val.clone(), t.span()))
            }
            Tok::Template => Ok(Expr::Template(self.parse_template(false)?)),
            Tok::Punct(P::Slash) | Tok::Punct(P::SlashEq) => {
                self.tok = self.lx.rescan_regexp(self.tok.start, self.tok.nl_before)?;
                let t = self.next()?;
                validate_regexp_flags(&t.regex_flags, t.start)?;
                // It is a Syntax Error if IsValidRegularExpressionLiteral(RegularExpressionLiteral) is false.
                let flags = kahvi_regexp::Flags {
                    ignore_case: t.regex_flags.contains('i'),
                    multiline: t.regex_flags.contains('m'),
                    dot_all: t.regex_flags.contains('s'),
                    unicode: t.regex_flags.contains('u'),
                    unicode_sets: t.regex_flags.contains('v'),
                };
                if let Err(e) = kahvi_regexp::validate(&t.str_val, flags) {
                    return Err(SyntaxError::new(
                        format!(
                            "invalid regular expression /{}/: {}",
                            String::from_utf16_lossy(&t.str_val),
                            e
                        ),
                        t.start,
                    ));
                }
                Ok(Expr::RegExp {
                    span: t.span(),
                    pattern: t.str_val,
                    flags: t.regex_flags,
                })
            }
            Tok::Punct(P::LBracket) => self.parse_array_literal(cover),
            Tok::Punct(P::LBrace) => self.parse_object_literal(cover),
            Tok::Punct(P::LParen) => self.parse_paren_or_arrow(),
            _ => self.unexpected(),
        }
    }

    pub fn parse_template(&mut self, tagged: bool) -> PResult<Template> {
        let start = self.start();
        let mut cooked = Vec::new();
        let mut raw = Vec::new();
        let mut exprs = Vec::new();
        loop {
            let t = self.next()?;
            if t.kind != Tok::Template {
                return self.err("expected template", t.start);
            }
            if !t.cooked_ok && !tagged {
                return self.err("invalid escape sequence in template literal", t.start);
            }
            cooked.push(if t.cooked_ok {
                Some(t.str_val.clone())
            } else {
                None
            });
            raw.push(t.raw.clone());
            if t.template_tail {
                break;
            }
            exprs.push(self.parse_expression_allow_in()?);
            if !self.is(P::RBrace) {
                return self.unexpected();
            }
            self.tok = self.lx.rescan_template_continuation(self.tok.start)?;
        }
        Ok(Template {
            cooked,
            raw,
            exprs,
            span: self.span_from(start),
        })
    }

    fn parse_array_literal(&mut self, cover: &mut Cover) -> PResult<Expr> {
        let start = self.start();
        self.next()?;
        let mut elements = Vec::new();
        self.with_in(true, |p| {
            while !p.is(P::RBracket) {
                if p.eat(P::Comma)? {
                    elements.push(None);
                    continue;
                }
                let is_spread = p.eat(P::Ellipsis)?;
                let e = p.parse_assignment_cover(cover)?;
                elements.push(Some(if is_spread {
                    ArrayElem::Spread(e)
                } else {
                    ArrayElem::Expr(e)
                }));
                if !p.is(P::RBracket) {
                    p.expect(P::Comma)?;
                    if is_spread {
                        p.spread_trailing_comma.insert(start);
                    }
                }
            }
            p.next()?;
            Ok(())
        })?;
        Ok(Expr::Array(elements, self.span_from(start)))
    }

    /// PropertyName.
    pub fn parse_property_key(&mut self) -> PResult<PropKey> {
        let t = &self.tok;
        let key = match t.kind {
            Tok::Ident => PropKey::Ident(t.ident.clone(), t.span()),
            Tok::String => {
                if t.legacy_octal && self.strict {
                    return self.err(
                        "octal escape sequences are not allowed in strict mode",
                        t.start,
                    );
                }
                PropKey::String(t.str_val.clone(), t.span())
            }
            Tok::Number => {
                if t.legacy_octal && self.strict {
                    return self.err(
                        "legacy octal literals are not allowed in strict mode",
                        t.start,
                    );
                }
                PropKey::Number(t.num, t.span())
            }
            Tok::BigInt => PropKey::BigInt(t.bigint.clone(), t.span()),
            Tok::PrivateName => PropKey::Private(t.ident.clone(), t.span()),
            Tok::Punct(P::LBracket) => {
                self.next()?;
                let e = self.with_in(true, |p| p.parse_assignment())?;
                self.expect(P::RBracket)?;
                return Ok(PropKey::Computed(Box::new(e)));
            }
            _ => return self.unexpected(),
        };
        self.next()?;
        Ok(key)
    }

    fn parse_object_literal(&mut self, cover: &mut Cover) -> PResult<Expr> {
        let start = self.start();
        self.next()?;
        let mut props = Vec::new();
        let mut seen_proto = false;
        self.with_in(true, |p| {
            while !p.is(P::RBrace) {
                let prop = p.parse_object_property(cover)?;
                if let ObjectProp::Proto(_, span) = &prop {
                    if seen_proto && cover.duplicate_proto.is_none() {
                        cover.duplicate_proto = Some(span.start as usize);
                    }
                    seen_proto = true;
                }
                props.push(prop);
                if !p.is(P::RBrace) {
                    p.expect(P::Comma)?;
                }
            }
            p.next()?;
            Ok(())
        })?;
        Ok(Expr::Object(props, self.span_from(start)))
    }

    fn parse_object_property(&mut self, cover: &mut Cover) -> PResult<ObjectProp> {
        let start = self.start();
        if self.eat(P::Ellipsis)? {
            let e = self.parse_assignment_cover(cover)?;
            return Ok(ObjectProp::Spread(e, self.span_from(start)));
        }
        // Method modifiers: async, get, set, *.
        let mut is_async = false;
        let mut is_generator = false;
        let mut accessor: Option<MethodKind> = None;
        if self.tok.kind == Tok::Ident
            && !self.tok.escaped
            && matches!(self.tok.ident.as_str(), "async" | "get" | "set")
        {
            let next = self.peek()?;
            let key_start = matches!(
                next.kind,
                Tok::Ident
                    | Tok::PrivateName
                    | Tok::String
                    | Tok::Number
                    | Tok::BigInt
                    | Tok::Punct(P::LBracket)
            );
            let modifies = if self.tok.ident == "async" {
                !next.nl_before && (key_start || next.is(P::Star))
            } else {
                key_start
            };
            if modifies {
                match self.tok.ident.as_str() {
                    "async" => is_async = true,
                    "get" => accessor = Some(MethodKind::Getter),
                    _ => accessor = Some(MethodKind::Setter),
                }
                self.next()?;
            }
        }
        if accessor.is_none() && self.eat(P::Star)? {
            is_generator = true;
        }
        let key_tok = self.tok.clone();
        let key = self.parse_property_key()?;
        if let PropKey::Private(_, span) = &key {
            return self.err(
                "private names are not allowed in object literals",
                span.start as usize,
            );
        }
        if let Some(kind) = accessor {
            let f = self.parse_method(start, kind, false, false, false)?;
            return Ok(match kind {
                MethodKind::Getter => ObjectProp::Getter(key, Rc::new(f)),
                _ => ObjectProp::Setter(key, Rc::new(f)),
            });
        }
        if self.is(P::LParen) || is_async || is_generator {
            let f = self.parse_method(start, MethodKind::Method, is_async, is_generator, false)?;
            return Ok(ObjectProp::Method(key, Rc::new(f)));
        }
        if self.eat(P::Colon)? {
            let value = self.parse_assignment_cover(cover)?;
            let is_proto = matches!(&key, PropKey::Ident(n, _) if n == "__proto__")
                || matches!(&key, PropKey::String(s, _) if *s == "__proto__".encode_utf16().collect::<Vec<_>>());
            if is_proto {
                return Ok(ObjectProp::Proto(value, self.span_from(start)));
            }
            return Ok(ObjectProp::Init(key, value, self.span_from(start)));
        }
        // Shorthand: an IdentifierReference.
        let PropKey::Ident(name, span) = key else {
            return self.unexpected();
        };
        if key_tok.escaped
            && crate::RESERVED_WORDS.contains(&name.as_str())
            && !matches!(name.as_str(), "yield" | "await")
        {
            return self.err(
                "keywords cannot contain escape characters",
                span.start as usize,
            );
        }
        self.check_identifier(&name, span.start as usize, false)?;
        if name == "arguments" {
            if !self.fn_ctx().allow_arguments {
                return self.err(
                    "'arguments' is not allowed in class field initializers or static blocks",
                    span.start as usize,
                );
            }
            self.fn_ctx_mut().uses_arguments = true;
        }
        let ident = Ident { name, span };
        if self.is(P::Eq) {
            let eq_pos = self.tok.start;
            self.next()?;
            let default = self.parse_assignment()?;
            cover.shorthand_init.get_or_insert(eq_pos);
            return Ok(ObjectProp::CoverInit(ident, default, self.span_from(start)));
        }
        Ok(ObjectProp::Shorthand(ident))
    }

    /// `( ... )`: a parenthesized expression or arrow function parameters.
    fn parse_paren_or_arrow(&mut self) -> PResult<Expr> {
        let start = self.start();
        self.next()?;
        let saved_yield = self.fn_ctx_mut().yield_or_await_pos.take();
        let mut inner = Cover::default();
        let mut items: Vec<Expr> = Vec::new();
        let mut rest: Option<Pattern> = None;
        let mut trailing_comma = None;
        self.with_in(true, |p| {
            while !p.is(P::RParen) {
                if p.eat(P::Ellipsis)? {
                    rest = Some(p.parse_binding_target()?);
                    if !p.is(P::RParen) {
                        return p.err("rest parameter must be last", p.tok.start);
                    }
                    break;
                }
                items.push(p.parse_assignment_cover(&mut inner)?);
                if !p.is(P::RParen) {
                    let comma = p.expect(P::Comma)?;
                    if p.is(P::RParen) {
                        trailing_comma = Some(comma.start);
                    }
                }
            }
            p.next()?;
            Ok(())
        })?;
        let inner_yield = self.fn_ctx_mut().yield_or_await_pos.take();
        self.fn_ctx_mut().yield_or_await_pos = saved_yield.or(inner_yield);
        if self.is(P::Arrow) && !self.tok.nl_before {
            if let Some(pos) = inner_yield {
                return self.err(
                    "yield/await expressions are not allowed in arrow parameters",
                    pos,
                );
            }
            let mut params = Vec::new();
            for item in items {
                params.push(self.expr_to_param(item)?);
            }
            return self.parse_arrow_from_params(start, params, rest, false);
        }
        if items.is_empty() || rest.is_some() {
            return self.unexpected();
        }
        if let Some(pos) = trailing_comma {
            return self.err("unexpected trailing comma in parenthesized expression", pos);
        }
        inner.check(self)?;
        let expr = if items.len() == 1 {
            items.pop().expect("one item")
        } else {
            let span = Span::new(
                items[0].span().start as usize,
                items.last().map_or(start, |e| e.span().end as usize),
            );
            Expr::Sequence(items, span)
        };
        Ok(Expr::Paren(Box::new(expr), self.span_from(start)))
    }

    /// `async ( ... )`: a call to something named async, or async arrow parameters.
    fn parse_async_call_or_arrow(&mut self, cover: &mut Cover) -> PResult<Expr> {
        let start = self.start();
        let async_tok = self.next()?;
        self.next()?; // (
        let saved_yield = self.fn_ctx_mut().yield_or_await_pos.take();
        let mut inner = Cover::default();
        let mut items: Vec<Arg> = Vec::new();
        let mut rest: Option<Pattern> = None;
        let mut rest_expr: Option<Expr> = None;
        self.with_in(true, |p| {
            while !p.is(P::RParen) {
                if p.eat(P::Ellipsis)? {
                    let e = p.parse_assignment_cover(&mut inner)?;
                    if p.is(P::RParen) && rest.is_none() {
                        rest_expr = Some(e.clone());
                    }
                    items.push(Arg::Spread(e));
                } else {
                    items.push(Arg::Expr(p.parse_assignment_cover(&mut inner)?));
                }
                if !p.eat(P::Comma)? {
                    break;
                }
            }
            p.expect(P::RParen)?;
            Ok(())
        })?;
        let inner_yield = self.fn_ctx_mut().yield_or_await_pos.take();
        self.fn_ctx_mut().yield_or_await_pos = saved_yield.or(inner_yield);
        if self.is(P::Arrow) && !self.tok.nl_before {
            if let Some(pos) = inner_yield {
                return self.err(
                    "yield/await expressions are not allowed in arrow parameters",
                    pos,
                );
            }
            let mut params = Vec::new();
            let n = items.len();
            for (i, item) in items.into_iter().enumerate() {
                match item {
                    Arg::Expr(e) => params.push(self.expr_to_param(e)?),
                    Arg::Spread(e) if i == n - 1 => {
                        let _ = &rest_expr;
                        rest = Some(self.expr_to_binding_pattern(e)?);
                    }
                    Arg::Spread(e) => {
                        return self.err("rest parameter must be last", e.span().start as usize);
                    }
                }
            }
            for p in params.iter().map(|p| &p.target).chain(rest.iter()) {
                if crate::early::bound_names(p).contains(&"await") {
                    return self.err(
                        "'await' is not a valid parameter name in an async function",
                        start,
                    );
                }
            }
            return self.parse_arrow_from_params(start, params, rest, true);
        }
        cover.merge(inner);
        let callee = Expr::Ident(Ident {
            name: "async".into(),
            span: async_tok.span(),
        });
        Ok(Expr::Call {
            callee: Box::new(callee),
            args: items,
            optional: false,
            span: self.span_from(start),
        })
    }
}

fn is_private_reference(e: &Expr) -> bool {
    match e {
        Expr::Member {
            property: MemberProp::Private(..),
            ..
        } => true,
        Expr::OptionalChain(inner, _) => is_private_reference(inner),
        _ => false,
    }
}

fn validate_regexp_flags(flags: &str, pos: usize) -> PResult<()> {
    let mut seen = String::new();
    for c in flags.chars() {
        if !"dgimsuyv".contains(c) || seen.contains(c) {
            return Err(SyntaxError::new(
                format!("invalid regular expression flags '{flags}'"),
                pos,
            ));
        }
        seen.push(c);
    }
    if seen.contains('u') && seen.contains('v') {
        return Err(SyntaxError::new(
            "regular expression flags 'u' and 'v' are exclusive",
            pos,
        ));
    }
    Ok(())
}
