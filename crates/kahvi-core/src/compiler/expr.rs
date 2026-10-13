//! Expressions.

use kahvi_parser::ast::{
    Arg, ArrayElem, AssignOp, BinaryOp, Expr, LogicalOp, MemberProp, ObjectProp, Pattern, PropKey,
    Template, UnaryOp, UpdateOp,
};

use super::{BinOp, Compiler, ErrorType, MethodDef, MethodKind, Op, TemplateInfo};
use crate::object::PropertyKey;
use crate::value::JsString;

/// How a function or class created by NamedEvaluation gets its name.
#[derive(Clone, Copy)]
pub(super) enum FnName {
    /// Its own BindingIdentifier, or "" (no NamedEvaluation).
    Own,
    Static(JsString),
    /// The name (a property key value) is on top of the stack and is consumed.
    OnStack,
}

pub(super) fn binary_op(op: BinaryOp) -> BinOp {
    match op {
        BinaryOp::Add => BinOp::Add,
        BinaryOp::Sub => BinOp::Sub,
        BinaryOp::Mul => BinOp::Mul,
        BinaryOp::Div => BinOp::Div,
        BinaryOp::Mod => BinOp::Mod,
        BinaryOp::Exp => BinOp::Exp,
        BinaryOp::Shl => BinOp::Shl,
        BinaryOp::Shr => BinOp::Shr,
        BinaryOp::UShr => BinOp::UShr,
        BinaryOp::BitAnd => BinOp::BitAnd,
        BinaryOp::BitOr => BinOp::BitOr,
        BinaryOp::BitXor => BinOp::BitXor,
        BinaryOp::Eq => BinOp::Eq,
        BinaryOp::NotEq => BinOp::NotEq,
        BinaryOp::StrictEq => BinOp::StrictEq,
        BinaryOp::StrictNotEq => BinOp::StrictNotEq,
        BinaryOp::Lt => BinOp::Lt,
        BinaryOp::Gt => BinOp::Gt,
        BinaryOp::LtEq => BinOp::LtEq,
        BinaryOp::GtEq => BinOp::GtEq,
        BinaryOp::In => BinOp::In,
        BinaryOp::InstanceOf => BinOp::InstanceOf,
    }
}

impl Compiler<'_> {
    /// The property key of a non-computed PropertyName.
    pub(super) fn static_key(&mut self, key: &PropKey) -> Option<PropertyKey> {
        let units = key.static_name()?;
        let s = self.utf16(&units);
        Some(self.agent.key_from_string(s))
    }

    /// Pushes a property name's key value: [] -> [key]. Computed keys are
    /// evaluated and converted with ToPropertyKey.
    pub(super) fn compile_prop_key(&mut self, key: &PropKey) {
        match key {
            PropKey::Computed(e) => {
                self.compile_expr(e);
                self.emit(Op::ToPropertyKey);
            }
            PropKey::Private(name, _) => {
                let s = self.string(&format!("#{name}"));
                self.emit(Op::String(s));
            }
            _ => {
                let units = key.static_name().unwrap_or_default();
                let s = self.utf16(&units);
                self.emit(Op::String(s));
            }
        }
    }

    pub(super) fn compile_expr(&mut self, e: &Expr) {
        match e {
            Expr::Ident(id) => {
                let name = self.string(&id.name);
                self.emit(Op::GetName(name));
            }
            Expr::This(_) => {
                self.emit(Op::This);
            }
            Expr::Null(_) => {
                self.emit(Op::Null);
            }
            Expr::Bool(b, _) => {
                self.emit(if *b { Op::True } else { Op::False });
            }
            Expr::Number(n, _) => {
                self.emit(Op::Number(*n));
            }
            Expr::BigInt(digits, _) => {
                let b = crate::ops::bigint::from_literal(self.agent, digits);
                self.emit(Op::BigInt(b));
            }
            Expr::String(s, _) => {
                let s = self.utf16(s);
                self.emit(Op::String(s));
            }
            Expr::RegExp {
                pattern,
                flags,
                span,
            } => {
                self.position(*span);
                let p = self.utf16(pattern);
                let f = self.string(flags);
                self.emit(Op::RegExp(p, f));
            }
            Expr::Template(t) => self.compile_template(t),
            Expr::TaggedTemplate { tag, quasi, span } => {
                self.compile_callee(tag);
                let index = self.template_object(quasi);
                self.emit(Op::GetTemplateObject(index));
                for e in &quasi.exprs {
                    self.compile_expr(e);
                }
                self.position(*span);
                self.emit(Op::Call(quasi.exprs.len() as u32 + 1));
            }
            Expr::Array(elems, _) => {
                self.emit(Op::NewArray);
                for el in elems {
                    match el {
                        None => {
                            self.emit(Op::ArrayHole);
                        }
                        Some(ArrayElem::Expr(e)) => {
                            self.compile_expr(e);
                            self.emit(Op::ArrayPush);
                        }
                        Some(ArrayElem::Spread(e)) => {
                            self.compile_expr(e);
                            self.emit(Op::ArraySpread);
                        }
                    }
                }
            }
            Expr::Object(props, _) => self.compile_object(props),
            Expr::Function(f) => {
                let i = self.compile_nested_function(f, true);
                self.emit(Op::Closure(i));
            }
            Expr::Arrow(f) => {
                let i = self.compile_nested_function(f, false);
                self.emit(Op::Closure(i));
            }
            Expr::Class(c) => self.compile_class(c, FnName::Own),
            Expr::Unary(op, arg, span) => self.compile_unary(*op, arg, *span),
            Expr::Update {
                op,
                prefix,
                target,
                span,
            } => self.compile_update(*op, *prefix, target, *span),
            Expr::Binary(op, l, r, span) => {
                self.compile_expr(l);
                self.compile_expr(r);
                self.position(*span);
                self.emit(Op::Binary(binary_op(*op)));
            }
            Expr::PrivateIn(name, obj, span) => {
                self.compile_expr(obj);
                self.position(*span);
                let s = self.string(&format!("#{name}"));
                self.emit(Op::PrivateIn(s));
            }
            Expr::Logical(op, l, r, _) => {
                self.compile_expr(l);
                let j = self.emit(match op {
                    LogicalOp::And => Op::JumpIfFalseKeep(0),
                    LogicalOp::Or => Op::JumpIfTrueKeep(0),
                    LogicalOp::Coalesce => Op::JumpIfNotNullishKeep(0),
                });
                self.emit(Op::Pop);
                self.compile_expr(r);
                self.patch(j);
            }
            Expr::Conditional {
                test,
                consequent,
                alternate,
                ..
            } => {
                self.compile_expr(test);
                let j = self.emit(Op::JumpIfFalse(0));
                self.compile_expr(consequent);
                let end = self.emit(Op::Jump(0));
                self.patch(j);
                self.compile_expr(alternate);
                self.patch(end);
            }
            Expr::Assign {
                op,
                target,
                value,
                span,
            } => self.compile_assign(*op, target, value, *span),
            Expr::Sequence(es, _) => {
                for (i, e) in es.iter().enumerate() {
                    self.compile_expr(e);
                    if i + 1 < es.len() {
                        self.emit(Op::Pop);
                    }
                }
            }
            Expr::Member {
                object,
                property,
                optional,
                span,
            } => {
                self.compile_expr(object);
                if *optional {
                    self.optional_check(0);
                }
                self.position(*span);
                self.compile_member_get(property);
            }
            Expr::SuperMember(prop, span) => {
                self.position(*span);
                match prop {
                    MemberProp::Ident(name, _) => {
                        let k = self.agent.key(name);
                        self.emit(Op::GetSuperProp(k));
                    }
                    MemberProp::Computed(e) => {
                        self.compile_expr(e);
                        self.emit(Op::GetSuperElem);
                    }
                    MemberProp::Private(..) => self.not_implemented("super.#private"),
                }
            }
            Expr::Call {
                callee,
                args,
                optional,
                span,
            } => {
                let direct_eval =
                    !*optional && matches!(&**callee, Expr::Ident(id) if id.name == "eval");
                self.compile_callee(callee);
                if *optional {
                    // [func this]: short-circuit if func is undefined or null.
                    self.emit(Op::Swap);
                    self.optional_check(1);
                    self.emit(Op::Swap);
                }
                self.compile_call_args(args, *span, direct_eval);
            }
            Expr::SuperCall(args, span) => {
                self.emit(Op::GetSuperConstructor);
                if args.iter().any(|a| matches!(a, Arg::Spread(_))) {
                    self.compile_args_array(args);
                    self.position(*span);
                    self.emit(Op::SuperCallSpread);
                } else {
                    for a in args {
                        if let Arg::Expr(e) = a {
                            self.compile_expr(e);
                        }
                    }
                    self.position(*span);
                    self.emit(Op::SuperCall(args.len() as u32));
                }
            }
            Expr::New { callee, args, span } => {
                self.compile_expr(callee);
                if args.iter().any(|a| matches!(a, Arg::Spread(_))) {
                    self.compile_args_array(args);
                    self.position(*span);
                    self.emit(Op::NewSpread);
                } else {
                    for a in args {
                        if let Arg::Expr(e) = a {
                            self.compile_expr(e);
                        }
                    }
                    self.position(*span);
                    self.emit(Op::New(args.len() as u32));
                }
            }
            Expr::OptionalChain(inner, _) => {
                self.optional_exits.push(Vec::new());
                self.compile_expr(inner);
                let exits = self.optional_exits.pop().unwrap_or_default();
                let end = self.emit(Op::Jump(0));
                for j in exits {
                    self.patch(j);
                }
                self.emit(Op::Undefined);
                self.patch(end);
            }
            Expr::NewTarget(_) => {
                self.emit(Op::NewTarget);
            }
            Expr::ImportMeta(_) => {
                self.emit(Op::ImportMeta);
            }
            Expr::Import {
                specifier,
                options,
                span,
            } => {
                self.compile_expr(specifier);
                match options {
                    Some(o) => self.compile_expr(o),
                    None => {
                        self.emit(Op::Undefined);
                    }
                }
                self.position(*span);
                self.emit(Op::ImportCall);
            }
            Expr::Yield {
                argument,
                delegate,
                span,
            } => {
                match argument {
                    Some(a) => self.compile_expr(a),
                    None => {
                        self.emit(Op::Undefined);
                    }
                }
                self.position(*span);
                let async_gen = self.code.is_async && self.code.is_generator;
                // In an async generator: AsyncGeneratorYield(? Await(value)).
                if async_gen && !*delegate {
                    self.emit(Op::Await);
                }
                if *delegate && async_gen {
                    self.compile_async_yield_star();
                    return;
                }
                // yield*: the delegate iterator, and undefined as the first received value.
                if *delegate {
                    self.emit(Op::GetIterator);
                    self.emit(Op::Undefined);
                }
                let y = self.emit(if *delegate {
                    Op::YieldDelegate(0)
                } else {
                    Op::Yield(0)
                });
                // Normal resumption continues after this jump; a return
                // resumption lands on the stub with the value pushed.
                let after = self.emit(Op::Jump(0));
                self.patch(y);
                // AsyncGeneratorUnwrapYieldResumption: a return resumption awaits its value.
                if async_gen && !*delegate {
                    self.emit(Op::Await);
                }
                self.emit_return();
                self.patch(after);
            }
            Expr::Await(arg, span) => {
                self.compile_expr(arg);
                self.position(*span);
                self.emit(Op::Await);
            }
            Expr::Paren(inner, _) => self.compile_expr(inner),
        }
    }

    /// `yield*` in an async generator (§15.5.5 steps 5-8 with generatorKind
    /// async), the value to delegate to on the stack: [iterable] -> [result].
    fn compile_async_yield_star(&mut self) {
        self.emit(Op::GetAsyncIterator);
        // received = NormalCompletion(undefined).
        self.emit(Op::Undefined);
        self.emit(Op::SetDelegateKind(0));
        let top = self.here();
        let to_return = self.emit(Op::JumpIfDelegateKind(2, 0));
        // a/b: next, or throw; b.iii when there is no throw method: close, then a TypeError.
        let no_throw = self.emit(Op::AsyncDelegateCall(0));
        let to_await = self.emit(Op::Jump(0));
        self.patch(no_throw);
        self.emit(Op::Pop);
        let skip = self.emit(Op::AsyncIteratorClose(0));
        self.emit(Op::Await);
        self.emit(Op::IteratorCheckObject);
        self.patch(skip);
        self.throw_error(
            ErrorType::Type,
            "The iterator does not provide a 'throw' method",
        );
        // c: a return resumption.
        self.patch(to_return);
        let no_return = self.emit(Op::AsyncDelegateReturn(0));
        let to_await2 = self.emit(Op::Jump(0));
        // c.iii: no return method: return ? Await(received.[[Value]]).
        self.patch(no_return);
        self.emit(Op::Await);
        self.emit(Op::PopIterator);
        self.emit_return();
        // innerResult = ? Await(innerResult), then the result checks.
        self.patch(to_await);
        self.patch(to_await2);
        self.emit(Op::Await);
        let done = self.emit(Op::AsyncDelegateResult(0));
        // received = Completion(AsyncGeneratorYield(? IteratorValue(innerResult))).
        self.emit(Op::DelegateYield);
        let unwrap = self.emit(Op::JumpIfDelegateKind(2, 0));
        self.emit(Op::Jump(top));
        // AsyncGeneratorUnwrapYieldResumption: a return resumption awaits its
        // value; a rejection becomes a throw completion.
        self.patch(unwrap);
        let caught = self.emit(Op::PushHandler(0));
        self.emit(Op::Await);
        self.emit(Op::PopHandler);
        self.emit(Op::Jump(top));
        self.patch(caught);
        self.emit(Op::Swap);
        self.emit(Op::Pop);
        self.emit(Op::SetDelegateKind(1));
        self.emit(Op::Jump(top));
        // Done: the value is the result of yield*, or the return value.
        self.patch(done);
        let as_return = self.emit(Op::JumpIfDelegateKind(2, 0));
        let after = self.emit(Op::Jump(0));
        self.patch(as_return);
        self.emit_return();
        self.patch(after);
    }

    /// Inside an optional chain: if the value on top is undefined or null,
    /// drop it and `extra` values below it and leave the chain with undefined.
    fn optional_check(&mut self, extra: u32) {
        let ok = self.emit(Op::JumpIfNotNullishKeep(0));
        for _ in 0..=extra {
            self.emit(Op::Pop);
        }
        let exit = self.emit(Op::Jump(0));
        if let Some(exits) = self.optional_exits.last_mut() {
            exits.push(exit);
        }
        self.patch(ok);
    }

    /// [base] -> [value]
    fn compile_member_get(&mut self, property: &MemberProp) {
        match property {
            MemberProp::Ident(name, _) => {
                let k = self.agent.key(name);
                self.emit(Op::GetProp(k));
            }
            MemberProp::Computed(e) => {
                self.compile_expr(e);
                self.emit(Op::GetElem);
            }
            MemberProp::Private(name, _) => {
                let s = self.string(&format!("#{name}"));
                self.emit(Op::GetPrivate(s));
            }
        }
    }

    /// Evaluates a callee as a reference: [] -> [func this].
    pub(super) fn compile_callee(&mut self, callee: &Expr) {
        match callee {
            Expr::Ident(id) => {
                let name = self.string(&id.name);
                self.position(id.span);
                self.emit(Op::GetNameForCall(name));
            }
            Expr::Member {
                object,
                property,
                optional,
                span,
            } => {
                self.compile_expr(object);
                if *optional {
                    self.optional_check(0);
                }
                self.position(*span);
                match property {
                    MemberProp::Ident(name, _) => {
                        let k = self.agent.key(name);
                        self.emit(Op::GetPropForCall(k));
                    }
                    MemberProp::Computed(e) => {
                        self.compile_expr(e);
                        self.emit(Op::GetElemForCall);
                    }
                    MemberProp::Private(name, _) => {
                        let s = self.string(&format!("#{name}"));
                        self.emit(Op::GetPrivateForCall(s));
                    }
                }
            }
            Expr::SuperMember(prop, span) => {
                self.position(*span);
                match prop {
                    MemberProp::Ident(name, _) => {
                        let k = self.agent.key(name);
                        self.emit(Op::GetSuperPropForCall(k));
                    }
                    MemberProp::Computed(e) => {
                        self.compile_expr(e);
                        self.emit(Op::GetSuperElemForCall);
                    }
                    MemberProp::Private(..) => self.not_implemented("super.#private"),
                }
            }
            // A parenthesized reference keeps its this value: `(a.b)()`.
            Expr::Paren(inner, _)
                if matches!(
                    inner.unparen(),
                    Expr::Member { .. } | Expr::SuperMember(..) | Expr::Ident(_)
                ) =>
            {
                self.compile_callee(inner.unparen())
            }
            _ => {
                self.compile_expr(callee);
                self.emit(Op::Undefined);
            }
        }
    }

    /// [func this] -> [result]
    fn compile_call_args(
        &mut self,
        args: &[Arg],
        span: kahvi_parser::ast::Span,
        direct_eval: bool,
    ) {
        if args.iter().any(|a| matches!(a, Arg::Spread(_))) {
            self.compile_args_array(args);
            self.position(span);
            self.emit(if direct_eval {
                Op::CallEvalSpread
            } else {
                Op::CallSpread
            });
        } else {
            for a in args {
                if let Arg::Expr(e) = a {
                    self.compile_expr(e);
                }
            }
            self.position(span);
            let n = args.len() as u32;
            self.emit(if direct_eval {
                Op::CallEval(n)
            } else {
                Op::Call(n)
            });
        }
    }

    /// [] -> [array of the argument values]
    fn compile_args_array(&mut self, args: &[Arg]) {
        self.emit(Op::NewArray);
        for a in args {
            match a {
                Arg::Expr(e) => {
                    self.compile_expr(e);
                    self.emit(Op::ArrayPush);
                }
                Arg::Spread(e) => {
                    self.compile_expr(e);
                    self.emit(Op::ArraySpread);
                }
            }
        }
    }

    fn compile_template(&mut self, t: &Template) {
        let first = t.cooked.first().cloned().flatten().unwrap_or_default();
        let s = self.utf16(&first);
        self.emit(Op::String(s));
        for (i, e) in t.exprs.iter().enumerate() {
            self.compile_expr(e);
            self.emit(Op::ToString);
            self.emit(Op::Binary(BinOp::Add));
            let tail = t.cooked.get(i + 1).cloned().flatten().unwrap_or_default();
            if !tail.is_empty() {
                let s = self.utf16(&tail);
                self.emit(Op::String(s));
                self.emit(Op::Binary(BinOp::Add));
            }
        }
    }

    fn template_object(&mut self, t: &Template) -> u32 {
        let cooked = t
            .cooked
            .iter()
            .map(|c| c.as_ref().map(|c| self.agent.intern(c)))
            .collect();
        let raw = t.raw.iter().map(|r| self.agent.intern(r)).collect();
        self.code.templates.push(TemplateInfo {
            cooked,
            raw,
            site: t.span.start,
        });
        (self.code.templates.len() - 1) as u32
    }

    fn compile_object(&mut self, props: &[ObjectProp]) {
        self.emit(Op::NewObject);
        for p in props {
            match p {
                ObjectProp::Init(key, value, _) => match key {
                    PropKey::Computed(_) => {
                        self.compile_prop_key(key);
                        if value.is_anonymous_function_definition() {
                            self.emit(Op::Dup);
                            self.compile_named(value, FnName::OnStack);
                        } else {
                            self.compile_expr(value);
                        }
                        self.emit(Op::DefineFieldComputed);
                    }
                    _ => {
                        let k = self.static_key(key).expect("static key");
                        if value.is_anonymous_function_definition() {
                            let name = match k {
                                PropertyKey::String(s) => s,
                                _ => {
                                    let v = self.agent.key_to_value(k);
                                    v.as_string().unwrap_or(self.agent.names.empty)
                                }
                            };
                            self.compile_named(value, FnName::Static(name));
                        } else {
                            self.compile_expr(value);
                        }
                        self.emit(Op::DefineField(k));
                    }
                },
                ObjectProp::Shorthand(id) => {
                    let name = self.string(&id.name);
                    self.emit(Op::GetName(name));
                    let k = self.agent.key_from_string(name);
                    self.emit(Op::DefineField(k));
                }
                ObjectProp::CoverInit(_, _, span) => {
                    self.position(*span);
                    self.throw_error(ErrorType::Syntax, "invalid shorthand property initializer");
                }
                ObjectProp::Method(key, f)
                | ObjectProp::Getter(key, f)
                | ObjectProp::Setter(key, f) => {
                    let kind = match p {
                        ObjectProp::Getter(..) => MethodKind::Getter,
                        ObjectProp::Setter(..) => MethodKind::Setter,
                        _ => MethodKind::Method,
                    };
                    self.compile_prop_key(key);
                    let i = self.compile_nested_function(f, false);
                    self.emit(Op::Closure(i));
                    self.emit(Op::DefineMethod(MethodDef {
                        kind,
                        enumerable: true,
                    }));
                }
                ObjectProp::Spread(e, _) => {
                    self.compile_expr(e);
                    self.emit(Op::CopyDataProperties);
                }
                ObjectProp::Proto(e, _) => {
                    self.compile_expr(e);
                    self.emit(Op::SetPrototypeLiteral);
                }
            }
        }
    }

    /// NamedEvaluation of an anonymous function or class definition.
    pub(super) fn compile_named(&mut self, e: &Expr, name: FnName) {
        match e.unparen() {
            Expr::Function(f) | Expr::Arrow(f) => {
                let is_expr = matches!(e.unparen(), Expr::Function(_));
                let i = self.compile_nested_function(f, is_expr);
                match name {
                    FnName::Own => {
                        self.emit(Op::Closure(i));
                    }
                    FnName::Static(s) => {
                        self.emit(Op::String(s));
                        self.emit(Op::ClosureNamed(i));
                    }
                    FnName::OnStack => {
                        self.emit(Op::ClosureNamed(i));
                    }
                }
            }
            Expr::Class(c) => self.compile_class(c, name),
            other => {
                if matches!(name, FnName::OnStack) {
                    self.emit(Op::Pop);
                }
                self.compile_expr(other)
            }
        }
    }

    /// Evaluates an initializer for a binding target, applying NamedEvaluation
    /// when the target is an identifier and the value an anonymous function.
    pub(super) fn compile_named_expr(&mut self, value: &Expr, target: &Pattern) {
        if let Pattern::Ident(id) = target
            && value.is_anonymous_function_definition()
        {
            let name = self.string(&id.name);
            self.compile_named(value, FnName::Static(name));
        } else {
            self.compile_expr(value);
        }
    }

    fn compile_unary(&mut self, op: UnaryOp, arg: &Expr, span: kahvi_parser::ast::Span) {
        match op {
            UnaryOp::Delete => self.compile_delete(arg, span),
            UnaryOp::Void => {
                self.compile_expr(arg);
                self.emit(Op::Pop);
                self.emit(Op::Undefined);
            }
            UnaryOp::Typeof => {
                if let Expr::Ident(id) = arg.unparen() {
                    let name = self.string(&id.name);
                    self.emit(Op::TypeofName(name));
                } else {
                    self.compile_expr(arg);
                    self.emit(Op::Typeof);
                }
            }
            UnaryOp::Plus => {
                self.compile_expr(arg);
                self.position(span);
                self.emit(Op::Plus);
            }
            UnaryOp::Minus => {
                self.compile_expr(arg);
                self.position(span);
                self.emit(Op::Neg);
            }
            UnaryOp::BitNot => {
                self.compile_expr(arg);
                self.position(span);
                self.emit(Op::BitNot);
            }
            UnaryOp::Not => {
                self.compile_expr(arg);
                self.emit(Op::Not);
            }
        }
    }

    fn compile_delete(&mut self, arg: &Expr, span: kahvi_parser::ast::Span) {
        self.position(span);
        match arg.unparen() {
            Expr::Ident(id) => {
                let name = self.string(&id.name);
                self.emit(Op::DeleteName(name));
            }
            Expr::Member {
                object,
                property,
                optional,
                ..
            } => {
                self.compile_expr(object);
                if *optional {
                    self.optional_check(0);
                }
                match property {
                    MemberProp::Ident(name, _) => {
                        let k = self.agent.key(name);
                        self.emit(Op::DeleteProp(k));
                    }
                    MemberProp::Computed(e) => {
                        self.compile_expr(e);
                        self.emit(Op::DeleteElem);
                    }
                    MemberProp::Private(..) => {
                        self.emit(Op::Pop);
                        self.throw_error(ErrorType::Syntax, "private fields can't be deleted");
                    }
                }
            }
            Expr::SuperMember(prop, _) => {
                // The reference is evaluated (this binding checked), then deleting it throws.
                self.emit(Op::This);
                self.emit(Op::Pop);
                if let MemberProp::Computed(e) = prop {
                    self.compile_expr(e);
                    self.emit(Op::Pop);
                }
                self.throw_error(ErrorType::Reference, "unsupported reference to 'super'");
            }
            Expr::OptionalChain(inner, _) => {
                self.optional_exits.push(Vec::new());
                self.compile_delete(inner, span);
                let exits = self.optional_exits.pop().unwrap_or_default();
                let end = self.emit(Op::Jump(0));
                for j in exits {
                    self.patch(j);
                }
                self.emit(Op::True);
                self.patch(end);
            }
            other => {
                self.compile_expr(other);
                self.emit(Op::Pop);
                self.emit(Op::True);
            }
        }
    }

    fn compile_update(
        &mut self,
        op: UpdateOp,
        prefix: bool,
        target: &Expr,
        span: kahvi_parser::ast::Span,
    ) {
        let step = if op == UpdateOp::Increment {
            Op::Inc
        } else {
            Op::Dec
        };
        self.position(span);
        match target.unparen() {
            Expr::Ident(id) => {
                let name = self.string(&id.name);
                self.emit(Op::ResolveName(name));
                self.emit(Op::GetRef);
                self.emit(Op::ToNumeric);
                if !prefix {
                    self.emit(Op::Dup);
                }
                self.emit(step);
                self.emit(Op::PutRef);
                if !prefix {
                    self.emit(Op::Pop);
                }
            }
            Expr::Member {
                object, property, ..
            } => {
                self.compile_expr(object);
                match property {
                    MemberProp::Ident(name, _) => {
                        let k = self.agent.key(name);
                        self.emit(Op::Dup);
                        self.emit(Op::GetProp(k));
                        self.emit(Op::ToNumeric);
                        if prefix {
                            self.emit(step);
                        } else {
                            // [base old] -> [old base new]
                            self.emit(Op::Dup);
                            self.emit(step);
                            self.emit(Op::Swap);
                            self.emit(Op::RotRight3);
                        }
                        self.emit(Op::SetProp(k));
                    }
                    MemberProp::Computed(e) => {
                        self.compile_expr(e);
                        self.emit(Op::GetElemForUpdate);
                        self.emit(Op::ToNumeric);
                        if prefix {
                            self.emit(step);
                        } else {
                            // [base key old] -> [old base key new]
                            self.emit(Op::Dup);
                            self.emit(step);
                            self.emit(Op::Swap);
                            self.emit(Op::RotRight4);
                        }
                        self.emit(Op::SetElem);
                    }
                    MemberProp::Private(name, _) => {
                        let s = self.string(&format!("#{name}"));
                        self.emit(Op::Dup);
                        self.emit(Op::GetPrivate(s));
                        self.emit(Op::ToNumeric);
                        if prefix {
                            self.emit(step);
                        } else {
                            self.emit(Op::Dup);
                            self.emit(step);
                            self.emit(Op::Swap);
                            self.emit(Op::RotRight3);
                        }
                        self.emit(Op::SetPrivate(s));
                    }
                }
                if !prefix {
                    self.emit(Op::Pop);
                }
            }
            Expr::SuperMember(..) => self.not_implemented("update expression on a super property"),
            other => {
                // Early error in strict code; a ReferenceError at run time otherwise.
                self.compile_expr(other);
                self.emit(Op::Pop);
                self.throw_error(ErrorType::Reference, "invalid update expression target");
            }
        }
    }

    fn compile_assign(
        &mut self,
        op: AssignOp,
        target: &Pattern,
        value: &Expr,
        span: kahvi_parser::ast::Span,
    ) {
        match op {
            AssignOp::Assign => match target {
                Pattern::Ident(id) => {
                    let name = self.string(&id.name);
                    self.emit(Op::ResolveName(name));
                    self.compile_named_expr(value, target);
                    self.position(span);
                    self.emit(Op::PutRef);
                }
                Pattern::Expr(e) => self.compile_assign_to_expr(e, value, span),
                Pattern::Object { .. } | Pattern::Array { .. } => {
                    self.compile_expr(value);
                    self.emit(Op::Dup);
                    self.compile_assignment_pattern(target);
                }
            },
            AssignOp::Binary(bop) => {
                let bop = binary_op(bop);
                match target {
                    Pattern::Ident(id) => {
                        let name = self.string(&id.name);
                        self.emit(Op::ResolveName(name));
                        self.emit(Op::GetRef);
                        self.compile_expr(value);
                        self.position(span);
                        self.emit(Op::Binary(bop));
                        self.emit(Op::PutRef);
                    }
                    Pattern::Expr(e) => match e.unparen() {
                        Expr::Member {
                            object, property, ..
                        } => {
                            self.compile_expr(object);
                            match property {
                                MemberProp::Ident(name, _) => {
                                    let k = self.agent.key(name);
                                    self.emit(Op::Dup);
                                    self.emit(Op::GetProp(k));
                                    self.compile_expr(value);
                                    self.position(span);
                                    self.emit(Op::Binary(bop));
                                    self.emit(Op::SetProp(k));
                                }
                                MemberProp::Computed(k) => {
                                    self.compile_expr(k);
                                    self.emit(Op::GetElemForUpdate);
                                    self.compile_expr(value);
                                    self.position(span);
                                    self.emit(Op::Binary(bop));
                                    self.emit(Op::SetElem);
                                }
                                MemberProp::Private(name, _) => {
                                    let s = self.string(&format!("#{name}"));
                                    self.emit(Op::Dup);
                                    self.emit(Op::GetPrivate(s));
                                    self.compile_expr(value);
                                    self.position(span);
                                    self.emit(Op::Binary(bop));
                                    self.emit(Op::SetPrivate(s));
                                }
                            }
                        }
                        Expr::SuperMember(prop, _) => match prop {
                            MemberProp::Ident(name, _) => {
                                let k = self.agent.key(name);
                                self.emit(Op::GetSuperProp(k));
                                self.compile_expr(value);
                                self.emit(Op::Binary(bop));
                                self.emit(Op::SetSuperProp(k));
                            }
                            _ => self.not_implemented(
                                "compound assignment to a computed super property",
                            ),
                        },
                        other => {
                            self.compile_expr(other);
                            self.emit(Op::Pop);
                            self.throw_error(ErrorType::Reference, "invalid assignment target");
                        }
                    },
                    _ => self.throw_error(ErrorType::Syntax, "invalid compound assignment target"),
                }
            }
            AssignOp::Logical(lop) => {
                let jump = |lop: LogicalOp| match lop {
                    LogicalOp::And => Op::JumpIfFalseKeep(0),
                    LogicalOp::Or => Op::JumpIfTrueKeep(0),
                    LogicalOp::Coalesce => Op::JumpIfNotNullishKeep(0),
                };
                match target {
                    Pattern::Ident(id) => {
                        let name = self.string(&id.name);
                        self.emit(Op::ResolveName(name));
                        self.emit(Op::GetRef);
                        let skip = self.emit(jump(lop));
                        self.emit(Op::Pop);
                        self.compile_named_expr(value, target);
                        self.emit(Op::PutRef);
                        let end = self.emit(Op::Jump(0));
                        self.patch(skip);
                        self.emit(Op::DropRef);
                        self.patch(end);
                    }
                    Pattern::Expr(e) => match e.unparen() {
                        Expr::Member {
                            object, property, ..
                        } => {
                            self.compile_expr(object);
                            match property {
                                MemberProp::Ident(name, _) => {
                                    let k = self.agent.key(name);
                                    self.emit(Op::Dup);
                                    self.emit(Op::GetProp(k));
                                    let skip = self.emit(jump(lop));
                                    self.emit(Op::Pop);
                                    self.compile_expr(value);
                                    self.emit(Op::SetProp(k));
                                    let end = self.emit(Op::Jump(0));
                                    self.patch(skip);
                                    // [base value] -> [value]
                                    self.emit(Op::Swap);
                                    self.emit(Op::Pop);
                                    self.patch(end);
                                }
                                MemberProp::Computed(k) => {
                                    self.compile_expr(k);
                                    self.emit(Op::GetElemForUpdate);
                                    let skip = self.emit(jump(lop));
                                    self.emit(Op::Pop);
                                    self.compile_expr(value);
                                    self.emit(Op::SetElem);
                                    let end = self.emit(Op::Jump(0));
                                    self.patch(skip);
                                    // [base key value] -> [value]
                                    self.emit(Op::RotRight3);
                                    self.emit(Op::Pop);
                                    self.emit(Op::Pop);
                                    self.patch(end);
                                }
                                MemberProp::Private(name, _) => {
                                    let s = self.string(&format!("#{name}"));
                                    self.emit(Op::Dup);
                                    self.emit(Op::GetPrivate(s));
                                    let skip = self.emit(jump(lop));
                                    self.emit(Op::Pop);
                                    self.compile_expr(value);
                                    self.emit(Op::SetPrivate(s));
                                    let end = self.emit(Op::Jump(0));
                                    self.patch(skip);
                                    self.emit(Op::Swap);
                                    self.emit(Op::Pop);
                                    self.patch(end);
                                }
                            }
                        }
                        _ => self.not_implemented("logical assignment to this target"),
                    },
                    _ => self.throw_error(ErrorType::Syntax, "invalid logical assignment target"),
                }
            }
        }
    }

    /// `target = value` where target is a member (or, in sloppy code, a call) expression.
    fn compile_assign_to_expr(
        &mut self,
        target: &Expr,
        value: &Expr,
        span: kahvi_parser::ast::Span,
    ) {
        match target.unparen() {
            Expr::Member {
                object, property, ..
            } => {
                self.compile_expr(object);
                match property {
                    MemberProp::Ident(name, _) => {
                        let k = self.agent.key(name);
                        self.compile_expr(value);
                        self.position(span);
                        self.emit(Op::SetProp(k));
                    }
                    MemberProp::Computed(k) => {
                        self.compile_expr(k);
                        self.compile_expr(value);
                        self.position(span);
                        self.emit(Op::SetElem);
                    }
                    MemberProp::Private(name, _) => {
                        let s = self.string(&format!("#{name}"));
                        self.compile_expr(value);
                        self.position(span);
                        self.emit(Op::SetPrivate(s));
                    }
                }
            }
            Expr::SuperMember(prop, _) => match prop {
                MemberProp::Ident(name, _) => {
                    let k = self.agent.key(name);
                    self.emit(Op::This);
                    self.emit(Op::Pop);
                    self.compile_expr(value);
                    self.emit(Op::SetSuperProp(k));
                }
                MemberProp::Computed(k) => {
                    self.emit(Op::This);
                    self.emit(Op::Pop);
                    self.compile_expr(k);
                    self.compile_expr(value);
                    self.emit(Op::SetSuperElem);
                }
                MemberProp::Private(..) => self.not_implemented("super.#private"),
            },
            other => {
                // `f() = 1` in sloppy code: the call happens, then a ReferenceError.
                self.compile_expr(other);
                self.emit(Op::Pop);
                self.throw_error(ErrorType::Reference, "invalid assignment target");
            }
        }
    }
}
