//! Functions, arrow functions, methods and classes (ECMA-262 §15).

use std::rc::Rc;

use super::{ClassScope, FnCtx, PResult, Parser, PrivateKind};
use crate::ast::*;
use crate::early;
use crate::lexer::{P, Tok};

impl Parser<'_> {
    fn child_ctx(&self, is_async: bool, is_generator: bool) -> FnCtx {
        let mut ctx = FnCtx::top(&super::ParseOptions::default());
        ctx.is_async = is_async;
        ctx.is_generator = is_generator;
        ctx.allow_return = true;
        ctx.allow_new_target = true;
        ctx.allow_arguments = true;
        ctx
    }

    /// `function` declarations and expressions; the current token is `function`.
    /// `is_async` means `async` has already been consumed.
    pub fn parse_function(
        &mut self,
        start: usize,
        is_async: bool,
        is_declaration: bool,
        name_optional: bool,
    ) -> PResult<Function> {
        self.expect_word("function")?;
        let is_generator = self.eat(P::Star)?;
        let id = if self.tok.kind == Tok::Ident && !self.is(P::LParen) {
            let tok = self.next()?;
            // A declaration's name is bound in the enclosing scope, so the
            // enclosing context's yield/await rules apply; an expression's
            // name follows the function's own.
            if is_declaration {
                self.check_identifier(&tok.ident, tok.start, true)?;
            } else {
                if (is_generator && tok.ident == "yield") || (is_async && tok.ident == "await") {
                    return self.err(
                        format!("'{}' is not a valid function name here", tok.ident),
                        tok.start,
                    );
                }
                let child = self.child_ctx(is_async, is_generator);
                let saved = std::mem::replace(self.fn_ctx_mut(), child);
                let r = self.check_identifier(&tok.ident, tok.start, true);
                *self.fn_ctx_mut() = saved;
                r?;
            }
            Some(Ident {
                span: tok.span(),
                name: tok.ident,
            })
        } else {
            if is_declaration && !name_optional {
                return self.unexpected();
            }
            None
        };
        let ctx = self.child_ctx(is_async, is_generator);
        self.parse_function_rest(start, id, FunctionKind::Normal, ctx, false)
    }

    /// Parameters and body, shared by functions and methods.
    fn parse_function_rest(
        &mut self,
        start: usize,
        id: Option<Ident>,
        kind: FunctionKind,
        ctx: FnCtx,
        unique_params: bool,
    ) -> PResult<Function> {
        let saved_strict = self.strict;
        let saved_classes_len = self.classes.len();
        self.push_fn(ctx);
        let result = (|| {
            let (params, rest) = self.parse_formal_parameters()?;
            self.expect(P::LBrace)?;
            let body = self.parse_directives_and_statements(true, |p| p.is(P::RBrace))?;
            let use_strict = self.saw_use_strict;
            self.expect(P::RBrace)?;
            Ok((params, rest, body, use_strict))
        })();
        let fctx = self.pop_fn();
        let (params, rest, body, has_use_strict) = result?;
        debug_assert_eq!(saved_classes_len, self.classes.len());
        let strict = self.strict;
        self.strict = saved_strict;
        let f = Function {
            id,
            kind,
            is_async: fctx.is_async,
            is_generator: fctx.is_generator,
            simple_params: is_simple(&params, &rest),
            expected_args: expected_args(&params),
            params,
            rest,
            body: FunctionBody::Block(body),
            strict,
            has_use_strict,
            span: self.span_from(start),
            uses_arguments: fctx.uses_arguments,
            uses_this: fctx.uses_this,
            has_direct_eval: fctx.has_direct_eval,
        };
        self.validate_function(&f, unique_params)?;
        Ok(f)
    }

    /// Early errors that depend on the whole function (strictness is only
    /// known after the body's directive prologue).
    fn validate_function(&self, f: &Function, unique_params: bool) -> PResult<()> {
        if f.has_use_strict && !f.simple_params {
            return self.err(
                "\"use strict\" not allowed in function with non-simple parameters",
                f.span.start as usize,
            );
        }
        if f.strict
            && let Some(id) = &f.id
        {
            self.check_strict_binding(&id.name, id.span.start as usize)?;
        }
        let mut names: Vec<(&str, usize)> = Vec::new();
        for p in f.params.iter().map(|p| &p.target).chain(f.rest.iter()) {
            for name in early::bound_names(p) {
                if f.strict {
                    self.check_strict_binding(name, p.span().start as usize)?;
                }
                names.push((name, p.span().start as usize));
            }
        }
        let no_duplicates =
            f.strict || unique_params || !f.simple_params || f.kind == FunctionKind::Arrow;
        if no_duplicates {
            for (i, (a, pos)) in names.iter().enumerate() {
                if names[..i].iter().any(|(b, _)| a == b) {
                    return self.err(format!("duplicate parameter name '{a}'"), *pos);
                }
            }
        }
        if let FunctionBody::Block(body) = &f.body {
            early::check_function_body(&names, body)?;
        }
        Ok(())
    }

    fn check_strict_binding(&self, name: &str, pos: usize) -> PResult<()> {
        if name == "eval" || name == "arguments" {
            return self.err(format!("cannot bind '{name}' in strict mode"), pos);
        }
        if crate::STRICT_RESERVED_WORDS.contains(&name) {
            return self.err(format!("'{name}' is reserved in strict mode"), pos);
        }
        Ok(())
    }

    /// FormalParameters including the parentheses.
    fn parse_formal_parameters(&mut self) -> PResult<(Vec<Param>, Option<Pattern>)> {
        self.expect(P::LParen)?;
        let list = self.parse_formal_parameter_list(|p| p.is(P::RParen))?;
        self.expect(P::RParen)?;
        Ok(list)
    }

    /// FormalParameters without the parentheses, stopping where `end` holds.
    pub(super) fn parse_formal_parameter_list(
        &mut self,
        end: impl Fn(&Self) -> bool,
    ) -> PResult<(Vec<Param>, Option<Pattern>)> {
        self.fn_ctx_mut().in_params = true;
        let mut params = Vec::new();
        let mut rest = None;
        let r = self.with_in(true, |p| {
            while !end(p) {
                if p.eat(P::Ellipsis)? {
                    rest = Some(p.parse_binding_target()?);
                    if p.is(P::Eq) {
                        return p.err("rest parameter may not have a default", p.tok.start);
                    }
                    break;
                }
                let elem = p.parse_binding_element()?;
                params.push(Param {
                    target: elem.target,
                    default: elem.default,
                });
                if !end(p) {
                    p.expect(P::Comma)?;
                }
            }
            Ok(())
        });
        self.fn_ctx_mut().in_params = false;
        r?;
        Ok((params, rest))
    }

    /// The parse steps of CreateDynamicFunction (§20.2.1.1.1, steps 17-24).
    pub(super) fn parse_dynamic_function_parts(
        params: &str,
        body: &str,
        source: &str,
        is_async: bool,
        is_generator: bool,
    ) -> PResult<Function> {
        let opts = super::ParseOptions::default();
        // 17-18. Let params be ParseText(paramString, paramGrammar); a List of errors is a SyntaxError.
        let mut p = Parser::new(params, false, &opts)?;
        let ctx = p.child_ctx(is_async, is_generator);
        p.push_fn(ctx);
        p.parse_formal_parameter_list(|p| p.tok.kind == Tok::Eof)?;
        if p.tok.kind != Tok::Eof {
            return p.unexpected();
        }
        p.pop_fn();
        p.finish_private_names()?;
        // 19-20. Let body be ParseText(bodyParseString, bodyGrammar); a List of errors is a SyntaxError.
        let mut p = Parser::new(body, false, &opts)?;
        let ctx = p.child_ctx(is_async, is_generator);
        p.push_fn(ctx);
        let statements = p.parse_directives_and_statements(true, |p| p.tok.kind == Tok::Eof)?;
        p.pop_fn();
        early::check_function_body(&[], &statements)?;
        p.finish_private_names()?;
        // 23-24. Let expr be ParseText(sourceText, exprGrammar); a List of errors is a SyntaxError.
        let mut p = Parser::new(source, false, &opts)?;
        if is_async {
            p.expect_word("async")?;
        }
        let f = p.parse_function(0, is_async, false, true)?;
        if p.tok.kind != Tok::Eof || f.is_generator != is_generator {
            return p.unexpected();
        }
        p.finish_private_names()?;
        Ok(f)
    }

    /// The rest of an arrow function after its parameters were recovered from
    /// the cover grammar; the current token is `=>`.
    pub fn parse_arrow_from_params(
        &mut self,
        start: usize,
        params: Vec<Param>,
        rest: Option<Pattern>,
        is_async: bool,
    ) -> PResult<Expr> {
        self.expect(P::Arrow)?;
        let outer = self.fn_ctx().clone();
        let mut ctx = self.child_ctx(is_async, false);
        ctx.is_arrow = true;
        ctx.allow_new_target = outer.allow_new_target;
        ctx.allow_super_property = outer.allow_super_property;
        ctx.allow_super_call = outer.allow_super_call;
        ctx.allow_arguments = outer.allow_arguments;
        for p in params.iter().map(|p| &p.target).chain(rest.iter()) {
            for name in early::bound_names(p) {
                if name == "await" && (is_async || self.await_reserved()) {
                    return self.err(
                        "'await' is not a valid parameter name here",
                        p.span().start as usize,
                    );
                }
                if name == "yield" && (self.strict || self.in_generator()) {
                    return self.err(
                        "'yield' is not a valid parameter name here",
                        p.span().start as usize,
                    );
                }
            }
        }
        let saved_strict = self.strict;
        self.push_fn(ctx);
        let mut has_use_strict = false;
        let body = if self.is(P::LBrace) {
            self.next().map(|_| ()).and_then(|_| {
                let body = self.with_in(true, |p| {
                    p.parse_directives_and_statements(true, |p| p.is(P::RBrace))
                })?;
                has_use_strict = self.saw_use_strict;
                self.expect(P::RBrace)?;
                Ok(FunctionBody::Block(body))
            })
        } else {
            self.parse_assignment()
                .map(|e| FunctionBody::Expr(Box::new(e)))
        };
        let fctx = self.pop_fn();
        let body = body?;
        let strict = self.strict;
        self.strict = saved_strict;
        let f = Function {
            id: None,
            kind: FunctionKind::Arrow,
            is_async,
            is_generator: false,
            simple_params: is_simple(&params, &rest),
            expected_args: expected_args(&params),
            params,
            rest,
            body,
            strict,
            has_use_strict,
            span: self.span_from(start),
            uses_arguments: fctx.uses_arguments,
            uses_this: fctx.uses_this,
            has_direct_eval: fctx.has_direct_eval,
        };
        self.validate_function(&f, true)?;
        Ok(Expr::Arrow(Rc::new(f)))
    }

    /// A method's parameters and body; the current token is `(`.
    pub fn parse_method(
        &mut self,
        start: usize,
        kind: MethodKind,
        is_async: bool,
        is_generator: bool,
        derived_ctor: bool,
    ) -> PResult<Function> {
        let mut ctx = self.child_ctx(is_async, is_generator);
        ctx.allow_super_property = true;
        ctx.allow_super_call = derived_ctor;
        let fkind = match kind {
            MethodKind::Method => FunctionKind::Method,
            MethodKind::Getter => FunctionKind::Getter,
            MethodKind::Setter => FunctionKind::Setter,
        };
        let f = self.parse_function_rest(start, None, fkind, ctx, true)?;
        match kind {
            MethodKind::Getter if !f.params.is_empty() || f.rest.is_some() => {
                self.err("getters must not have parameters", f.span.start as usize)
            }
            MethodKind::Setter if f.params.len() != 1 || f.rest.is_some() => self.err(
                "setters must have exactly one parameter",
                f.span.start as usize,
            ),
            _ => Ok(f),
        }
    }

    /// ClassDeclaration or ClassExpression; the current token is `class`.
    pub fn parse_class(&mut self, is_declaration: bool) -> PResult<Rc<Class>> {
        self.parse_class_with(is_declaration, false)
    }

    pub fn parse_class_with(
        &mut self,
        is_declaration: bool,
        name_optional: bool,
    ) -> PResult<Rc<Class>> {
        let start = self.start();
        self.expect_word("class")?;
        let saved_strict = self.strict;
        self.strict = true;
        let result = self.parse_class_inner(start, is_declaration, name_optional);
        self.strict = saved_strict;
        result.map(Rc::new)
    }

    fn parse_class_inner(
        &mut self,
        start: usize,
        is_declaration: bool,
        name_optional: bool,
    ) -> PResult<Class> {
        let id = if self.tok.kind == Tok::Ident && !self.is_word("extends") {
            Some(self.binding_identifier()?)
        } else {
            if is_declaration && !name_optional {
                return self.unexpected();
            }
            None
        };
        let super_class = if self.eat_word("extends")? {
            let lhs_start = self.start();
            let mut cover = super::expr::Cover::default();
            let e = self.parse_lhs(&mut cover)?;
            cover.check(self)?;
            if matches!(e, Expr::Arrow(_)) {
                return self.err(
                    "class heritage must be a left-hand-side expression",
                    lhs_start,
                );
            }
            Some(Box::new(e))
        } else {
            None
        };
        let derived = super_class.is_some();
        self.expect(P::LBrace)?;
        self.classes.push(ClassScope::default());
        let result = self.parse_class_body(derived);
        let scope = self.classes.pop().expect("class scope");
        let (constructor, elements) = result?;
        // Resolve private name references made in the body.
        for (name, pos) in scope.unresolved {
            if !scope.declared.contains_key(&name) {
                self.reference_private(&name, pos)?;
            }
        }
        Ok(Class {
            id,
            super_class,
            constructor,
            elements,
            span: self.span_from(start),
        })
    }

    fn parse_class_body(
        &mut self,
        derived: bool,
    ) -> PResult<(Option<Rc<Function>>, Vec<ClassElement>)> {
        let mut constructor = None;
        let mut elements = Vec::new();
        while !self.eat(P::RBrace)? {
            if self.eat(P::Semi)? {
                continue;
            }
            let mut start = self.start();
            let mut is_static = false;
            if self.is_word("static") {
                let next = self.peek()?;
                if next.is(P::LBrace) {
                    self.next()?;
                    elements.push(ClassElement::StaticBlock(Rc::new(
                        self.parse_static_block(start)?,
                    )));
                    continue;
                }
                if !matches!(
                    next.kind,
                    Tok::Punct(P::LParen | P::Eq | P::Semi | P::RBrace)
                ) {
                    is_static = true;
                    self.next()?;
                    // The element's source text (a method's [[SourceText]]) is
                    // the MethodDefinition or FieldDefinition after `static`.
                    start = self.start();
                }
            }
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
            let key = self.parse_property_key()?;
            let key_name = key.static_name().map(|s| String::from_utf16_lossy(&s));
            if let PropKey::Private(name, span) = &key
                && name == "constructor"
            {
                return self.err(
                    "#constructor is not a valid private name",
                    span.start as usize,
                );
            }
            if self.is(P::LParen) {
                let kind = accessor.unwrap_or(MethodKind::Method);
                let is_ctor = !is_static
                    && key_name.as_deref() == Some("constructor")
                    && !matches!(key, PropKey::Computed(_));
                if is_ctor {
                    if kind != MethodKind::Method || is_async || is_generator {
                        return self.err(
                            "class constructor may not be an accessor, generator or async",
                            start,
                        );
                    }
                    if constructor.is_some() {
                        return self.err("a class may only have one constructor", start);
                    }
                    let mut f =
                        self.parse_method(start, MethodKind::Method, false, false, derived)?;
                    f.kind = if derived {
                        FunctionKind::DerivedConstructor
                    } else {
                        FunctionKind::ClassConstructor
                    };
                    constructor = Some(Rc::new(f));
                    continue;
                }
                if is_static
                    && key_name.as_deref() == Some("prototype")
                    && !matches!(key, PropKey::Computed(_))
                {
                    return self.err(
                        "classes may not have a static property named 'prototype'",
                        start,
                    );
                }
                if let PropKey::Private(name, span) = &key {
                    let pk = match (kind, is_static) {
                        (MethodKind::Method, false) => PrivateKind::Method,
                        (MethodKind::Method, true) => PrivateKind::StaticMethod,
                        (MethodKind::Getter, false) => PrivateKind::Getter,
                        (MethodKind::Getter, true) => PrivateKind::StaticGetter,
                        (MethodKind::Setter, false) => PrivateKind::Setter,
                        (MethodKind::Setter, true) => PrivateKind::StaticSetter,
                    };
                    self.declare_private(name, pk, span.start as usize)?;
                }
                let f = self.parse_method(start, kind, is_async, is_generator, false)?;
                elements.push(ClassElement::Method {
                    key,
                    kind,
                    is_static,
                    function: Rc::new(f),
                });
                continue;
            }
            if accessor.is_some() || is_async || is_generator {
                return self.unexpected();
            }
            // Field definition.
            if !matches!(key, PropKey::Computed(_) | PropKey::Private(..)) {
                if key_name.as_deref() == Some("constructor") {
                    return self.err("classes may not have a field named 'constructor'", start);
                }
                if is_static && key_name.as_deref() == Some("prototype") {
                    return self.err(
                        "classes may not have a static field named 'prototype'",
                        start,
                    );
                }
            }
            if let PropKey::Private(name, span) = &key {
                let pk = if is_static {
                    PrivateKind::StaticField
                } else {
                    PrivateKind::Field
                };
                self.declare_private(name, pk, span.start as usize)?;
            }
            let init = if self.is(P::Eq) {
                let init_start = self.start();
                self.next()?;
                Some(Rc::new(self.parse_field_initializer(init_start)?))
            } else {
                None
            };
            self.semicolon()?;
            elements.push(ClassElement::Field {
                key,
                is_static,
                init,
                span: self.span_from(start),
            });
        }
        Ok((constructor, elements))
    }

    fn declare_private(&mut self, name: &str, kind: PrivateKind, pos: usize) -> PResult<()> {
        let scope = self.classes.last_mut().expect("class scope");
        use PrivateKind::*;
        match (scope.declared.get(name).copied(), kind) {
            (None, k) => {
                scope.declared.insert(name.to_string(), k);
                Ok(())
            }
            (Some(Getter), Setter) | (Some(Setter), Getter) => {
                scope.declared.insert(name.to_string(), GetterSetter);
                Ok(())
            }
            (Some(StaticGetter), StaticSetter) | (Some(StaticSetter), StaticGetter) => {
                scope.declared.insert(name.to_string(), StaticGetterSetter);
                Ok(())
            }
            _ => self.err(format!("duplicate private name #{name}"), pos),
        }
    }

    fn field_ctx(&self) -> FnCtx {
        let mut ctx = self.child_ctx(false, false);
        ctx.allow_return = false;
        ctx.allow_arguments = false;
        ctx.allow_super_property = true;
        ctx
    }

    /// A field initializer, wrapped as its own function (it gets a fresh `this`).
    fn parse_field_initializer(&mut self, start: usize) -> PResult<Function> {
        self.push_fn(self.field_ctx());
        let e = self.with_in(true, |p| p.parse_assignment());
        let fctx = self.pop_fn();
        let e = e?;
        if contains_super_call(&e) {
            return self.err("'super' call is not allowed in a field initializer", start);
        }
        Ok(Function {
            id: None,
            kind: FunctionKind::ClassFieldInit,
            is_async: false,
            is_generator: false,
            params: Vec::new(),
            rest: None,
            body: FunctionBody::Expr(Box::new(e)),
            strict: true,
            has_use_strict: false,
            simple_params: true,
            span: self.span_from(start),
            uses_arguments: false,
            uses_this: fctx.uses_this,
            has_direct_eval: fctx.has_direct_eval,
            expected_args: 0,
        })
    }

    /// `static { ... }`; the current token is `{`.
    fn parse_static_block(&mut self, start: usize) -> PResult<Function> {
        let mut ctx = self.field_ctx();
        ctx.in_static_block = true;
        self.push_fn(ctx);
        let body = (|| {
            self.expect(P::LBrace)?;
            let mut body = Vec::new();
            while !self.is(P::RBrace) {
                if self.tok.kind == Tok::Eof {
                    return self.unexpected();
                }
                body.push(self.parse_statement_list_item()?);
            }
            self.next()?;
            Ok(body)
        })();
        let fctx = self.pop_fn();
        let body = body?;
        early::check_function_body(&[], &body)?;
        Ok(Function {
            id: None,
            kind: FunctionKind::ClassFieldInit,
            is_async: false,
            is_generator: false,
            params: Vec::new(),
            rest: None,
            body: FunctionBody::Block(body),
            strict: true,
            has_use_strict: false,
            simple_params: true,
            span: self.span_from(start),
            uses_arguments: false,
            uses_this: fctx.uses_this,
            has_direct_eval: fctx.has_direct_eval,
            expected_args: 0,
        })
    }
}

fn is_simple(params: &[Param], rest: &Option<Pattern>) -> bool {
    rest.is_none()
        && params
            .iter()
            .all(|p| p.default.is_none() && matches!(p.target, Pattern::Ident(_)))
}

/// ExpectedArgumentCount: parameters before the first default or rest.
fn expected_args(params: &[Param]) -> u32 {
    params.iter().take_while(|p| p.default.is_none()).count() as u32
}

/// Whether `e` contains a SuperCall outside nested non-arrow functions.
fn contains_super_call(e: &Expr) -> bool {
    let mut found = false;
    early::visit_expr(e, &mut |x| {
        if matches!(x, Expr::SuperCall(..)) {
            found = true;
        }
    });
    found
}
