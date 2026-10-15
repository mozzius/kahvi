//! Statements and declarations (ECMA-262 §14).

use std::rc::Rc;

use super::{Label, PResult, Parser};
use crate::ast::*;
use crate::lexer::{P, Tok};

impl Parser<'_> {
    /// StatementListItem: Statement or Declaration.
    pub fn parse_statement_list_item(&mut self) -> PResult<Stmt> {
        if self.tok.kind == Tok::Ident && !self.tok.escaped {
            match self.tok.ident.as_str() {
                "function" => return self.parse_function_declaration(false, self.start()),
                "class" => return Ok(Stmt::Class(self.parse_class(true)?)),
                "const" => return self.parse_lexical_declaration(VarKind::Const),
                "let" if self.let_starts_declaration()? => {
                    return self.parse_lexical_declaration(VarKind::Let);
                }
                "async" if self.async_function_follows()? => {
                    let start = self.start();
                    self.next()?;
                    return self.parse_function_declaration(true, start);
                }
                "using" if self.using_starts_declaration()? => {
                    return self.parse_lexical_declaration(VarKind::Using);
                }
                "await" if self.in_async() && self.await_using_follows()? => {
                    return self.parse_lexical_declaration(VarKind::AwaitUsing);
                }
                _ => {}
            }
        }
        self.parse_statement()
    }

    /// `let` begins a LexicalDeclaration when followed by an identifier, `[` or `{`.
    fn let_starts_declaration(&self) -> PResult<bool> {
        let next = self.peek()?;
        Ok(match next.kind {
            Tok::Punct(P::LBracket) | Tok::Punct(P::LBrace) => true,
            Tok::Ident => could_bind(&next.ident),
            _ => false,
        })
    }

    fn async_function_follows(&self) -> PResult<bool> {
        let next = self.peek()?;
        Ok(next.kind == Tok::Ident && !next.escaped && next.ident == "function" && !next.nl_before)
    }

    /// `using [no LineTerminator here] BindingIdentifier`
    fn using_starts_declaration(&self) -> PResult<bool> {
        let next = self.peek()?;
        Ok(next.kind == Tok::Ident && !next.nl_before && could_bind(&next.ident))
    }

    /// `await [no LineTerminator here] using [no LineTerminator here] BindingIdentifier`
    fn await_using_follows(&self) -> PResult<bool> {
        let mut lx = self.lx.clone();
        let t1 = lx.next_token()?;
        if !(t1.kind == Tok::Ident && !t1.escaped && t1.ident == "using" && !t1.nl_before) {
            return Ok(false);
        }
        let t2 = lx.next_token()?;
        Ok(t2.kind == Tok::Ident && !t2.nl_before)
    }

    /// Statement (no declarations).
    pub fn parse_statement(&mut self) -> PResult<Stmt> {
        let start = self.start();
        match self.tok.kind {
            Tok::Punct(P::LBrace) => return Ok(Stmt::Block(self.parse_block()?)),
            Tok::Punct(P::Semi) => {
                self.next()?;
                return Ok(Stmt::Empty(self.span_from(start)));
            }
            Tok::Ident if !self.tok.escaped => {
                let word = self.tok.ident.clone();
                match word.as_str() {
                    "var" => {
                        self.next()?;
                        let decl = self.parse_var_declarations(VarKind::Var, start, true)?;
                        self.semicolon()?;
                        let mut decl = decl;
                        decl.span = self.span_from(start);
                        return Ok(Stmt::Var(decl));
                    }
                    "if" => return self.parse_if(),
                    "for" => return self.parse_for(),
                    "while" => {
                        self.next()?;
                        self.expect(P::LParen)?;
                        let test = self.parse_expression_allow_in()?;
                        self.expect(P::RParen)?;
                        let body = self.parse_loop_body()?;
                        return Ok(Stmt::While {
                            test,
                            body: Box::new(body),
                            span: self.span_from(start),
                        });
                    }
                    "do" => {
                        self.next()?;
                        let body = self.parse_loop_body()?;
                        self.expect_word("while")?;
                        self.expect(P::LParen)?;
                        let test = self.parse_expression_allow_in()?;
                        self.expect(P::RParen)?;
                        // A semicolon after do-while is always optional.
                        self.eat(P::Semi)?;
                        return Ok(Stmt::DoWhile {
                            body: Box::new(body),
                            test,
                            span: self.span_from(start),
                        });
                    }
                    "continue" | "break" => return self.parse_break_continue(word == "continue"),
                    "return" => {
                        if !self.fn_ctx().allow_return {
                            return self.err("'return' outside of a function", start);
                        }
                        self.next()?;
                        let arg = if self.can_insert_semicolon() {
                            None
                        } else {
                            Some(self.parse_expression_allow_in()?)
                        };
                        self.semicolon()?;
                        return Ok(Stmt::Return(arg, self.span_from(start)));
                    }
                    "with" => {
                        if self.strict {
                            return self.err("'with' is not allowed in strict mode", start);
                        }
                        self.next()?;
                        self.expect(P::LParen)?;
                        let object = self.parse_expression_allow_in()?;
                        self.expect(P::RParen)?;
                        let body = self.parse_substatement_not_labelled_function()?;
                        return Ok(Stmt::With {
                            object,
                            body: Box::new(body),
                            span: self.span_from(start),
                        });
                    }
                    "switch" => return self.parse_switch(),
                    "throw" => {
                        self.next()?;
                        if self.tok.nl_before {
                            return self.err("illegal newline after 'throw'", self.tok.start);
                        }
                        let arg = self.parse_expression_allow_in()?;
                        self.semicolon()?;
                        return Ok(Stmt::Throw(arg, self.span_from(start)));
                    }
                    "try" => return self.parse_try(),
                    "debugger" => {
                        self.next()?;
                        self.semicolon()?;
                        return Ok(Stmt::Debugger(self.span_from(start)));
                    }
                    "function" => {
                        return self.err("function declarations are not allowed here", start);
                    }
                    "class" => return self.err("class declarations are not allowed here", start),
                    "let" if self.peek()?.is(P::LBracket) => {
                        return self.err("lexical declaration is not allowed here", start);
                    }
                    "async" if self.async_function_follows()? => {
                        return self.err("async function declarations are not allowed here", start);
                    }
                    _ => {}
                }
                // LabelledStatement
                if self.peek()?.is(P::Colon) {
                    return self.parse_labelled();
                }
            }
            Tok::Ident if self.peek()?.is(P::Colon) => return self.parse_labelled(),
            _ => {}
        }
        // ExpressionStatement
        let expr = self.parse_expression_allow_in()?;
        self.semicolon()?;
        Ok(Stmt::Expr(expr, self.span_from(start)))
    }

    /// The body of if/with/labelled statements: a Statement, but lexical
    /// declarations are errors and (Annex B, sloppy) function declarations
    /// are allowed after `if`.
    fn parse_substatement(&mut self) -> PResult<Stmt> {
        if self.is_word("let") && self.peek()?.is(P::LBracket) {
            return self.err("lexical declaration is not allowed here", self.start());
        }
        self.parse_statement()
    }

    fn parse_loop_body(&mut self) -> PResult<Stmt> {
        self.fn_ctx_mut().in_iteration += 1;
        let body = self.parse_substatement_not_labelled_function();
        self.fn_ctx_mut().in_iteration -= 1;
        body
    }

    pub fn parse_block(&mut self) -> PResult<Block> {
        let start = self.start();
        self.expect(P::LBrace)?;
        let mut body = Vec::new();
        while !self.is(P::RBrace) {
            if self.tok.kind == Tok::Eof {
                return self.unexpected();
            }
            body.push(self.parse_statement_list_item()?);
        }
        self.next()?;
        crate::early::check_block_declarations(&body, self.strict)?;
        Ok(Block {
            body,
            span: self.span_from(start),
        })
    }

    fn parse_if(&mut self) -> PResult<Stmt> {
        let start = self.start();
        self.next()?;
        self.expect(P::LParen)?;
        let test = self.parse_expression_allow_in()?;
        self.expect(P::RParen)?;
        let consequent = self.parse_if_body()?;
        let alternate = if self.eat_word("else")? {
            Some(Box::new(self.parse_if_body()?))
        } else {
            None
        };
        Ok(Stmt::If {
            test,
            consequent: Box::new(consequent),
            alternate,
            span: self.span_from(start),
        })
    }

    /// Annex B.3.3: in sloppy mode, `if (x) function f() {}` is allowed.
    fn parse_if_body(&mut self) -> PResult<Stmt> {
        if !self.strict && self.is_word("function") {
            let start = self.start();
            let f = self.parse_function_declaration(false, start)?;
            if let Stmt::Function(func) = &f
                && (func.is_generator || func.is_async)
            {
                return self.err("generator declarations are not allowed here", start);
            }
            // The declaration behaves as if it were the only statement of a Block.
            let span = self.span_from(start);
            return Ok(Stmt::Block(Block {
                body: vec![f],
                span,
            }));
        }
        self.parse_substatement_not_labelled_function()
    }

    /// The Statement of an if, with or iteration statement: It is a Syntax
    /// Error if IsLabelledFunction(Statement) is true.
    fn parse_substatement_not_labelled_function(&mut self) -> PResult<Stmt> {
        let start = self.start();
        let stmt = self.parse_substatement()?;
        if is_labelled_function(&stmt) {
            return self.err("a labelled function declaration is not allowed here", start);
        }
        Ok(stmt)
    }

    fn parse_labelled(&mut self) -> PResult<Stmt> {
        let start = self.start();
        let tok = self.next()?;
        self.check_identifier(&tok.ident, tok.start, false)?;
        self.expect(P::Colon)?;
        let name = tok.ident.clone();
        if self.fn_ctx().labels.iter().any(|l| l.name == name) {
            return self.err(format!("duplicate label '{name}'"), tok.start);
        }
        let is_loop = self.statement_is_iteration()?;
        self.fn_ctx_mut().labels.push(Label {
            name: name.clone(),
            is_loop,
        });
        let body = if self.is_word("function") {
            if self.strict {
                return self.err(
                    "labelled function declarations are not allowed in strict mode",
                    self.start(),
                );
            }
            let fstart = self.start();
            let f = self.parse_function_declaration(false, fstart)?;
            if let Stmt::Function(func) = &f
                && (func.is_generator || func.is_async)
            {
                return self.err("labelled generator declarations are not allowed", fstart);
            }
            f
        } else {
            self.parse_substatement()?
        };
        self.fn_ctx_mut().labels.pop();
        Ok(Stmt::Labeled {
            label: Ident {
                name,
                span: tok.span(),
            },
            body: Box::new(body),
            span: self.span_from(start),
        })
    }

    /// Whether the statement at the current token is an iteration statement,
    /// looking through further labels (`a: b: while (...)`).
    fn statement_is_iteration(&self) -> PResult<bool> {
        let mut lx = self.lx.clone();
        let mut tok = self.tok.clone();
        loop {
            if tok.kind != Tok::Ident {
                return Ok(false);
            }
            if !tok.escaped && matches!(tok.ident.as_str(), "for" | "while" | "do") {
                return Ok(true);
            }
            let next = lx.next_token()?;
            if !next.is(P::Colon) {
                return Ok(false);
            }
            tok = lx.next_token()?;
        }
    }

    fn parse_break_continue(&mut self, is_continue: bool) -> PResult<Stmt> {
        let start = self.start();
        self.next()?;
        let label = if self.tok.kind == Tok::Ident && !self.tok.nl_before {
            let t = self.next()?;
            self.check_identifier(&t.ident, t.start, false)?;
            Some(Ident {
                span: t.span(),
                name: t.ident,
            })
        } else {
            None
        };
        self.semicolon()?;
        let ctx = self.fn_ctx();
        match &label {
            Some(l) => {
                let Some(found) = ctx.labels.iter().rev().find(|x| x.name == l.name) else {
                    return self.err(
                        format!("undefined label '{}'", l.name),
                        l.span.start as usize,
                    );
                };
                if is_continue && !found.is_loop {
                    return self.err(
                        format!("'{}' does not label an iteration statement", l.name),
                        start,
                    );
                }
            }
            None => {
                if is_continue && ctx.in_iteration == 0 {
                    return self.err("'continue' outside of a loop", start);
                }
                if !is_continue && ctx.in_iteration == 0 && ctx.in_switch == 0 {
                    return self.err("'break' outside of a loop or switch", start);
                }
            }
        }
        let span = self.span_from(start);
        Ok(if is_continue {
            Stmt::Continue(label, span)
        } else {
            Stmt::Break(label, span)
        })
    }

    fn parse_switch(&mut self) -> PResult<Stmt> {
        let start = self.start();
        self.next()?;
        self.expect(P::LParen)?;
        let discriminant = self.parse_expression_allow_in()?;
        self.expect(P::RParen)?;
        self.expect(P::LBrace)?;
        let mut cases = Vec::new();
        let mut seen_default = false;
        self.fn_ctx_mut().in_switch += 1;
        while !self.eat(P::RBrace)? {
            let case_start = self.start();
            let test = if self.eat_word("case")? {
                Some(self.parse_expression_allow_in()?)
            } else if self.is_word("default") {
                if seen_default {
                    return self.err("multiple default clauses in switch", case_start);
                }
                seen_default = true;
                self.next()?;
                None
            } else {
                return self.unexpected();
            };
            self.expect(P::Colon)?;
            let mut body = Vec::new();
            while !self.is(P::RBrace) && !self.is_word("case") && !self.is_word("default") {
                if self.tok.kind == Tok::Eof {
                    return self.unexpected();
                }
                body.push(self.parse_statement_list_item()?);
            }
            // CaseClause / DefaultClause: It is a Syntax Error if ContainsUsing of StatementList is true.
            if let Some(at) = contains_using(&body) {
                return self.err(
                    "'using' declarations are not allowed directly in a case clause",
                    at,
                );
            }
            cases.push(SwitchCase {
                test,
                body,
                span: self.span_from(case_start),
            });
        }
        self.fn_ctx_mut().in_switch -= 1;
        let all: Vec<Stmt> = cases.iter().flat_map(|c| c.body.iter().cloned()).collect();
        crate::early::check_block_declarations(&all, self.strict)?;
        Ok(Stmt::Switch {
            discriminant,
            cases,
            span: self.span_from(start),
        })
    }

    fn parse_try(&mut self) -> PResult<Stmt> {
        let start = self.start();
        self.next()?;
        let block = self.parse_block()?;
        let handler = if self.is_word("catch") {
            let cstart = self.start();
            self.next()?;
            let param = if self.eat(P::LParen)? {
                let p = self.parse_binding_target()?;
                self.expect(P::RParen)?;
                Some(p)
            } else {
                None
            };
            let body = self.parse_block()?;
            if let Some(p) = &param {
                crate::early::check_catch(p, &body.body)?;
            }
            Some(CatchClause {
                param,
                body,
                span: self.span_from(cstart),
            })
        } else {
            None
        };
        let finalizer = if self.eat_word("finally")? {
            Some(self.parse_block()?)
        } else {
            None
        };
        if handler.is_none() && finalizer.is_none() {
            return self.err("missing catch or finally after try", self.tok.start);
        }
        Ok(Stmt::Try {
            block,
            handler,
            finalizer,
            span: self.span_from(start),
        })
    }

    /// `let`/`const`/`using` declarations as a StatementListItem.
    fn parse_lexical_declaration(&mut self, kind: VarKind) -> PResult<Stmt> {
        let start = self.start();
        if kind == VarKind::AwaitUsing {
            self.next()?; // await
        }
        self.next()?; // let / const / using
        let mut decl = self.parse_var_declarations(kind, start, true)?;
        self.semicolon()?;
        decl.span = self.span_from(start);
        Ok(Stmt::Var(decl))
    }

    /// VariableDeclarationList / BindingList. `allow_in` is false in for-heads.
    pub fn parse_var_declarations(
        &mut self,
        kind: VarKind,
        start: usize,
        allow_in: bool,
    ) -> PResult<VarDecl> {
        let mut declarations = Vec::new();
        loop {
            let dstart = self.start();
            let target = if matches!(kind, VarKind::Using | VarKind::AwaitUsing) {
                Pattern::Ident(self.binding_identifier()?)
            } else {
                self.parse_binding_target()?
            };
            if kind.is_lexical() {
                for name in crate::early::bound_names(&target) {
                    if name == "let" {
                        return self.err("'let' cannot be a lexically bound name", dstart);
                    }
                }
            }
            let init = if self.eat(P::Eq)? {
                Some(self.with_in(allow_in, |p| p.parse_assignment())?)
            } else {
                None
            };
            declarations.push(VarDeclarator {
                target,
                init,
                span: self.span_from(dstart),
            });
            if !self.eat(P::Comma)? {
                break;
            }
        }
        let decl = VarDecl {
            kind,
            declarations,
            span: self.span_from(start),
        };
        if allow_in {
            self.check_declaration_initializers(&decl)?;
        }
        Ok(decl)
    }

    /// Outside for-in/of heads, `const` and destructuring need initializers.
    fn check_declaration_initializers(&self, decl: &VarDecl) -> PResult<()> {
        for d in &decl.declarations {
            if d.init.is_none() {
                if matches!(
                    decl.kind,
                    VarKind::Const | VarKind::Using | VarKind::AwaitUsing
                ) {
                    return self.err(
                        "missing initializer in const declaration",
                        d.span.start as usize,
                    );
                }
                if !matches!(d.target, Pattern::Ident(_)) {
                    return self.err(
                        "missing initializer in destructuring declaration",
                        d.span.start as usize,
                    );
                }
            }
        }
        Ok(())
    }

    fn parse_for(&mut self) -> PResult<Stmt> {
        let start = self.start();
        self.next()?;
        let is_await = if self.is_word("await") {
            if !self.in_async() {
                return self.unexpected();
            }
            self.next()?;
            true
        } else {
            false
        };
        self.expect(P::LParen)?;

        // Head: a declaration, an expression, or nothing.
        let decl_kind = if self.tok.kind == Tok::Ident && !self.tok.escaped {
            match self.tok.ident.as_str() {
                "var" => Some(VarKind::Var),
                "const" => Some(VarKind::Const),
                "let" => {
                    let next = self.peek()?;
                    // `let of` too: `let` is never the LeftHandSideExpression of a
                    // for-of, so `for (let of of x)` declares `of`.
                    let starts = next.is(P::LBracket)
                        || next.is(P::LBrace)
                        || (next.kind == Tok::Ident && could_bind(&next.ident));
                    starts.then_some(VarKind::Let)
                }
                "using" if self.using_in_for_head()? => Some(VarKind::Using),
                "await" if self.in_async() && self.await_using_follows()? => {
                    Some(VarKind::AwaitUsing)
                }
                _ => None,
            }
        } else {
            None
        };

        if let Some(kind) = decl_kind {
            let dstart = self.start();
            if kind == VarKind::AwaitUsing {
                self.next()?;
            }
            self.next()?;
            let decl = self.parse_var_declarations(kind, dstart, false)?;
            if decl.declarations.len() == 1 && (self.is_word("of") || self.is_word("in")) {
                let is_of = self.is_word("of");
                let d = decl
                    .declarations
                    .into_iter()
                    .next()
                    .expect("one declarator");
                if let Some(init) = &d.init {
                    // Annex B: `for (var x = 1 in o)` in sloppy mode with a simple binding.
                    let annex_b = !is_of
                        && kind == VarKind::Var
                        && !self.strict
                        && matches!(d.target, Pattern::Ident(_));
                    if !annex_b {
                        return self.err(
                            "for-in/of declarations may not have initializers",
                            init.span().start as usize,
                        );
                    }
                }
                if !is_of && matches!(kind, VarKind::Using | VarKind::AwaitUsing) {
                    return self.err("'using' declarations are not allowed in for-in", dstart);
                }
                if is_await && !is_of {
                    return self.unexpected();
                }
                self.next()?;
                let head = ForHead::Var {
                    kind,
                    target: d.target,
                    init: d.init,
                };
                return self.finish_for_in_of(start, head, is_of, is_await);
            }
            self.check_declaration_initializers(&decl)?;
            if is_await {
                return self.unexpected();
            }
            return self.finish_for(start, Some(ForInit::Var(decl)));
        }

        if self.is(P::Semi) {
            if is_await {
                return self.unexpected();
            }
            return self.finish_for(start, None);
        }

        // Expression or LeftHandSideExpression.
        let expr_start = self.start();
        let starts_with_let = self.is_word("let");
        let starts_with_async = self.is_word("async") && !self.peek()?.is(P::Arrow);
        let mut cover = super::expr::Cover::default();
        let expr = self.with_in(false, |p| p.parse_expression_cover(&mut cover))?;
        if self.is_word("of") || self.is_word("in") {
            let is_of = self.is_word("of");
            if is_of && starts_with_let {
                return self.err("for-of head may not start with 'let'", expr_start);
            }
            if is_of && starts_with_async && !is_await && matches!(expr, Expr::Ident(_)) {
                return self.err("for-of head may not start with 'async of'", expr_start);
            }
            if is_await && !is_of {
                return self.unexpected();
            }
            let target = if self.is_web_compat_call(&expr) {
                Pattern::Expr(Box::new(expr))
            } else {
                self.expr_to_assignment_target(expr, true)?
            };
            self.next()?;
            return self.finish_for_in_of(start, ForHead::Assign(target), is_of, is_await);
        }
        cover.check(self)?;
        if is_await {
            return self.unexpected();
        }
        self.finish_for(start, Some(ForInit::Expr(expr)))
    }

    fn using_in_for_head(&self) -> PResult<bool> {
        let next = self.peek()?;
        if next.kind != Tok::Ident || next.nl_before || !could_bind(&next.ident) {
            return Ok(false);
        }
        // `for ( [lookahead ≠ using of] ForDeclaration of ...)`: `using of` starts a
        // declaration only as a ForStatement's LexicalDeclaration (`for (using of = x;;)`).
        if next.ident == "of" && !next.escaped {
            let mut lx = self.lx.clone();
            lx.next_token()?; // of
            let after = lx.next_token()?;
            return Ok(after.is(P::Eq) || after.is(P::Comma) || after.is(P::Semi));
        }
        Ok(true)
    }

    fn finish_for(&mut self, start: usize, init: Option<ForInit>) -> PResult<Stmt> {
        self.expect(P::Semi)?;
        let test = if self.is(P::Semi) {
            None
        } else {
            Some(self.parse_expression_allow_in()?)
        };
        self.expect(P::Semi)?;
        let update = if self.is(P::RParen) {
            None
        } else {
            Some(self.parse_expression_allow_in()?)
        };
        self.expect(P::RParen)?;
        let body = self.parse_loop_body()?;
        if let Some(ForInit::Var(decl)) = &init
            && decl.kind.is_lexical()
        {
            crate::early::check_for_head_vs_body(decl, &body)?;
        }
        Ok(Stmt::For {
            init,
            test,
            update,
            body: Box::new(body),
            span: self.span_from(start),
        })
    }

    fn finish_for_in_of(
        &mut self,
        start: usize,
        left: ForHead,
        is_of: bool,
        is_await: bool,
    ) -> PResult<Stmt> {
        let right = if is_of {
            self.with_in(true, |p| p.parse_assignment())?
        } else {
            self.parse_expression_allow_in()?
        };
        self.expect(P::RParen)?;
        let body = self.parse_loop_body()?;
        if let ForHead::Var { kind, target, .. } = &left
            && kind.is_lexical()
        {
            crate::early::check_for_binding_vs_body(target, &body)?;
        }
        let span = self.span_from(start);
        Ok(if is_of {
            Stmt::ForOf {
                left,
                right,
                body: Box::new(body),
                is_await,
                span,
            }
        } else {
            Stmt::ForIn {
                left,
                right,
                body: Box::new(body),
                span,
            }
        })
    }

    /// `function` / `async function` declaration (the current token is `function`).
    pub fn parse_function_declaration(&mut self, is_async: bool, start: usize) -> PResult<Stmt> {
        let f = self.parse_function(start, is_async, true, false)?;
        Ok(Stmt::Function(Rc::new(f)))
    }
}

/// The word could start a BindingIdentifier (or is one of the contextual
/// words a declaration would then reject with a better error).
fn could_bind(word: &str) -> bool {
    !crate::RESERVED_WORDS.contains(&word) || matches!(word, "yield" | "await" | "let")
}

/// ContainsUsing of a StatementList: the start of its first `using` or
/// `await using` declaration.
pub(crate) fn contains_using(stmts: &[Stmt]) -> Option<usize> {
    stmts.iter().find_map(|s| match s {
        Stmt::Var(d) if matches!(d.kind, VarKind::Using | VarKind::AwaitUsing) => {
            Some(d.span.start as usize)
        }
        _ => None,
    })
}

/// IsLabelledFunction(stmt)
fn is_labelled_function(stmt: &Stmt) -> bool {
    match stmt {
        // 2. Let item be the LabelledItem of stmt.
        Stmt::Labeled { body, .. } => match &**body {
            // 3. If item is LabelledItem : FunctionDeclaration, return true.
            Stmt::Function(_) => true,
            // 4-5. If item is LabelledItem : Statement, return IsLabelledFunction(Statement).
            other => is_labelled_function(other),
        },
        // 1. If stmt is not a LabelledStatement, return false.
        _ => false,
    }
}
