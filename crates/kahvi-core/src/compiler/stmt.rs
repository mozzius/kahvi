//! Statements and control flow.

use kahvi_parser::ast::{
    Block, CatchClause, Expr, ForHead, ForInit, Pattern, Stmt, SwitchCase, VarDecl, VarKind,
};
use kahvi_parser::early;

use super::expr::FnName;
use super::{BinOp, Compiler, JumpScope, Op, ScopeInfo};

/// Whether a StatementList declares `using` at its top level: `None` if not,
/// `Some(true)` if any of them is `await using`.
pub(super) fn using_declarations<'s>(stmts: impl IntoIterator<Item = &'s Stmt>) -> Option<bool> {
    let mut found = None;
    for s in stmts {
        if let Stmt::Var(d) = s {
            match d.kind {
                VarKind::Using => found = Some(found.unwrap_or(false)),
                VarKind::AwaitUsing => found = Some(true),
                _ => {}
            }
        }
    }
    found
}

impl Compiler<'_> {
    pub(super) fn compile_statements(&mut self, stmts: &[Stmt]) {
        for s in stmts {
            self.compile_stmt(s);
        }
    }

    /// UpdateEmpty(stmtCompletion, undefined): statements whose completion
    /// is undefined unless their body produces a value.
    fn reset_completion(&mut self) {
        if self.completion {
            self.emit(Op::Undefined);
            self.emit(Op::SetCompletion);
        }
    }

    pub(super) fn compile_stmt(&mut self, s: &Stmt) {
        self.position(s.span());
        match s {
            Stmt::Var(decl) => self.compile_var_decl(decl),
            // Hoisted by declaration instantiation.
            Stmt::Function(_) => {}
            Stmt::Class(c) => {
                self.compile_class(c, FnName::Own);
                let name = c.id.as_ref().map(|i| i.name.as_str()).unwrap_or("");
                let name = self.string(name);
                self.emit(Op::InitName(name));
            }
            Stmt::Expr(e, _) => {
                self.compile_expr(e);
                if self.completion {
                    self.emit(Op::SetCompletion);
                } else {
                    self.emit(Op::Pop);
                }
            }
            Stmt::Block(b) => self.compile_block(b),
            Stmt::Empty(_) | Stmt::Debugger(_) => {}
            Stmt::If {
                test,
                consequent,
                alternate,
                ..
            } => {
                self.reset_completion();
                self.compile_expr(test);
                let j = self.emit(Op::JumpIfFalse(0));
                self.compile_stmt(consequent);
                match alternate {
                    Some(alt) => {
                        let end = self.emit(Op::Jump(0));
                        self.patch(j);
                        self.compile_stmt(alt);
                        self.patch(end);
                    }
                    None => self.patch(j),
                }
            }
            Stmt::For {
                init,
                test,
                update,
                body,
                ..
            } => self.compile_for(init.as_ref(), test.as_ref(), update.as_ref(), body),
            Stmt::ForIn {
                left, right, body, ..
            } => self.compile_for_in_of(left, right, body, false, false),
            Stmt::ForOf {
                left,
                right,
                body,
                is_await,
                ..
            } => self.compile_for_in_of(left, right, body, true, *is_await),
            Stmt::While { test, body, .. } => {
                self.reset_completion();
                let index = self.push_breakable(true, true);
                let top = self.here();
                self.compile_expr(test);
                let exit = self.emit(Op::JumpIfFalse(0));
                self.compile_stmt(body);
                self.emit(Op::Jump(top));
                self.patch(exit);
                debug_assert_eq!(index, self.scopes.len() - 1);
                self.pop_breakable(Some(top));
            }
            Stmt::DoWhile { body, test, .. } => {
                self.reset_completion();
                let index = self.push_breakable(true, true);
                let top = self.here();
                self.compile_stmt(body);
                self.patch_continues_here(index);
                self.compile_expr(test);
                self.emit(Op::JumpIfTrue(top));
                self.pop_breakable(None);
            }
            Stmt::Continue(label, _) => {
                let _ = self.emit_continue(label.as_ref().map(|l| l.name.as_str()));
            }
            Stmt::Break(label, _) => {
                let _ = self.emit_break(label.as_ref().map(|l| l.name.as_str()));
            }
            Stmt::Return(arg, _) => {
                match arg {
                    Some(e) => {
                        self.compile_expr(e);
                        // In an async generator, `return x` awaits x.
                        if self.code.is_async && self.code.is_generator {
                            self.emit(Op::Await);
                        }
                    }
                    None => {
                        self.emit(Op::Undefined);
                    }
                }
                self.emit_return();
            }
            Stmt::With { object, body, .. } => {
                self.reset_completion();
                self.compile_expr(object);
                self.emit(Op::PushWith);
                self.scopes.push(JumpScope::With);
                self.compile_stmt(body);
                self.scopes.pop();
                self.emit(Op::PopScope);
            }
            Stmt::Switch {
                discriminant,
                cases,
                ..
            } => self.compile_switch(discriminant, cases),
            Stmt::Labeled { label, body, .. } => {
                self.pending_labels.push(label.name.clone());
                match &**body {
                    Stmt::For { .. }
                    | Stmt::ForIn { .. }
                    | Stmt::ForOf { .. }
                    | Stmt::While { .. }
                    | Stmt::DoWhile { .. }
                    | Stmt::Labeled { .. } => self.compile_stmt(body),
                    _ => {
                        self.push_breakable(false, false);
                        self.compile_stmt(body);
                        self.pop_breakable(None);
                    }
                }
            }
            Stmt::Throw(e, span) => {
                self.compile_expr(e);
                self.position(*span);
                self.emit(Op::Throw);
            }
            Stmt::Try {
                block,
                handler,
                finalizer,
                ..
            } => self.compile_try(block, handler.as_ref(), finalizer.as_ref()),
        }
    }

    fn compile_block(&mut self, b: &Block) {
        match self.block_scope(&b.body) {
            Some(scope) => {
                self.emit(Op::PushScope(scope));
                self.scopes.push(JumpScope::Env);
                // Block : { StatementList }
                //   Let blockValue be Completion(Evaluation of StatementList).
                //   Set blockValue to Completion(DisposeResources(blockEnv.[[DisposableResourceStack]], blockValue)).
                match using_declarations(&b.body) {
                    Some(is_await) => {
                        self.compile_disposing(is_await, |c| c.compile_statements(&b.body))
                    }
                    None => self.compile_statements(&b.body),
                }
                self.scopes.pop();
                self.emit(Op::PopScope);
            }
            None => self.compile_statements(&b.body),
        }
    }

    /// Compiles `body` inside the running LexicalEnvironment (already
    /// pushed), followed on every way out of it — normal, throw, break,
    /// continue, return — by DisposeResources(env.[[DisposableResourceStack]],
    /// completion), whose result replaces the completion. This is a finally
    /// block whose code is DisposeResources. `has_await_using` says the scope
    /// declares `await using`, so DisposeResources may Await.
    pub(super) fn compile_disposing(
        &mut self,
        has_await_using: bool,
        body: impl FnOnce(&mut Self),
    ) {
        self.code.finally_targets.push(Vec::new());
        let index = (self.code.finally_targets.len() - 1) as u32;
        self.scopes.push(JumpScope::Finally {
            entry_jumps: Vec::new(),
            continuations: Vec::new(),
        });
        let handler = self.emit(Op::PushHandler(0));
        self.scopes.push(JumpScope::Handler);
        body(self);
        self.scopes.pop();
        self.emit(Op::PopHandler);
        let Some(JumpScope::Finally {
            entry_jumps,
            continuations,
        }) = self.scopes.pop()
        else {
            unreachable!("finally scope expected")
        };
        // Normal completion: [undefined 0].
        self.emit(Op::Undefined);
        self.emit(Op::Number(0.0));
        let to_dispose = self.emit(Op::Jump(0));
        // Throw completion: [exception 1].
        self.patch(handler);
        self.emit(Op::Number(1.0));
        self.patch(to_dispose);
        for j in entry_jumps {
            self.patch(j);
        }
        // [value kind] -> DisposeResources -> [value' kind'].
        self.emit(Op::DisposeStart);
        if has_await_using {
            // Each Await DisposeResources performs; a rejection of an async
            // dispose method's result is handed back to it.
            let top = self.here();
            let threw = self.emit(Op::PushHandler(0));
            let done = self.emit(Op::DisposeStep(0));
            self.emit(Op::Await);
            self.emit(Op::PopHandler);
            self.emit(Op::Pop);
            self.emit(Op::Jump(top));
            self.patch(threw);
            self.emit(Op::DisposeAwaitThrew);
            self.emit(Op::Jump(top));
            self.patch(done);
            self.emit(Op::PopHandler);
        } else {
            // Without `await using` every resource is ~sync-dispose~: no Await.
            let done = self.emit(Op::DisposeStep(0));
            self.patch(done);
        }
        self.emit(Op::DisposeEnd);
        self.emit(Op::EndFinally(index));
        if !continuations.is_empty() {
            let skip = self.emit(Op::Jump(0));
            self.emit_finally_continuations(index, continuations);
            self.patch(skip);
        }
    }

    fn compile_var_decl(&mut self, decl: &VarDecl) {
        match decl.kind {
            VarKind::Var => {
                for d in &decl.declarations {
                    let Some(init) = &d.init else { continue };
                    if let Pattern::Ident(id) = &d.target {
                        let name = self.string(&id.name);
                        self.emit(Op::ResolveName(name));
                        self.compile_named_expr(init, &d.target);
                        self.emit(Op::PutRef);
                        self.emit(Op::Pop);
                    } else {
                        self.compile_expr(init);
                        self.compile_binding(&d.target, false);
                    }
                }
            }
            VarKind::Let | VarKind::Const => {
                for d in &decl.declarations {
                    match &d.init {
                        Some(init) => self.compile_named_expr(init, &d.target),
                        None => {
                            self.emit(Op::Undefined);
                        }
                    }
                    self.compile_binding(&d.target, true);
                }
            }
            VarKind::Using | VarKind::AwaitUsing => {
                let is_await = decl.kind == VarKind::AwaitUsing;
                for d in &decl.declarations {
                    // LexicalBinding : BindingIdentifier Initializer
                    // 1-3. Let value be the value of Initializer (NamedEvaluation for an anonymous function).
                    match &d.init {
                        Some(init) => self.compile_named_expr(init, &d.target),
                        None => {
                            self.emit(Op::Undefined);
                        }
                    }
                    // 4-5. Return ? InitializeReferencedBinding(lhs, value, sync-dispose / async-dispose):
                    //      InitializeBinding first performs ? AddDisposableResource(envRec.[[DisposableResourceStack]], V, hint).
                    self.emit(Op::Dup);
                    self.emit(Op::AddDisposable(is_await));
                    self.compile_binding(&d.target, true);
                }
            }
        }
    }

    fn compile_for(
        &mut self,
        init: Option<&ForInit>,
        test: Option<&Expr>,
        update: Option<&Expr>,
        body: &Stmt,
    ) {
        self.reset_completion();
        // `for (let/const ...)`: a loop environment, copied per iteration for `let`.
        let mut per_iteration = false;
        let mut pushed_env = false;
        match init {
            Some(ForInit::Var(d)) if d.kind.is_lexical() => {
                let is_const = d.kind != VarKind::Let;
                let mut lexical = Vec::new();
                for decl in &d.declarations {
                    for n in early::bound_names(&decl.target) {
                        let s = self.string(n);
                        lexical.push((s, is_const));
                    }
                }
                per_iteration = !is_const && !lexical.is_empty();
                self.code.scopes.push(ScopeInfo {
                    lexical,
                    functions: Vec::new(),
                });
                let scope = (self.code.scopes.len() - 1) as u32;
                self.emit(Op::PushScope(scope));
                self.scopes.push(JumpScope::Env);
                pushed_env = true;
                if matches!(d.kind, VarKind::Using | VarKind::AwaitUsing) {
                    // Let forDcl be Completion(Evaluation of LexicalDeclaration); if it is
                    // abrupt, return ? DisposeResources(loopEnv.[[DisposableResourceStack]], forDcl).
                    // Let bodyResult be Completion(ForBodyEvaluation(...)).
                    // Return ? DisposeResources(loopEnv.[[DisposableResourceStack]], bodyResult).
                    self.compile_disposing(d.kind == VarKind::AwaitUsing, |c| {
                        c.compile_var_decl(d);
                        c.compile_for_body(false, test, update, body);
                    });
                    self.scopes.pop();
                    self.emit(Op::PopScope);
                    return;
                }
                self.compile_var_decl(d);
            }
            Some(ForInit::Var(d)) => self.compile_var_decl(d),
            Some(ForInit::Expr(e)) => {
                self.compile_expr(e);
                self.emit(Op::Pop);
            }
            None => {}
        }
        self.compile_for_body(per_iteration, test, update, body);
        if pushed_env {
            self.scopes.pop();
            self.emit(Op::PopScope);
        }
    }

    /// ForBodyEvaluation(test, increment, stmt, perIterationBindings, labelSet)
    fn compile_for_body(
        &mut self,
        per_iteration: bool,
        test: Option<&Expr>,
        update: Option<&Expr>,
        body: &Stmt,
    ) {
        if per_iteration {
            self.emit(Op::CopyScope);
        }
        let index = self.push_breakable(true, true);
        let top = self.here();
        let exit = test.map(|t| {
            self.compile_expr(t);
            self.emit(Op::JumpIfFalse(0))
        });
        self.compile_stmt(body);
        self.patch_continues_here(index);
        if per_iteration {
            self.emit(Op::CopyScope);
        }
        if let Some(u) = update {
            self.compile_expr(u);
            self.emit(Op::Pop);
        }
        self.emit(Op::Jump(top));
        if let Some(exit) = exit {
            self.patch(exit);
        }
        self.pop_breakable(None);
    }

    fn compile_for_in_of(
        &mut self,
        left: &ForHead,
        right: &Expr,
        body: &Stmt,
        is_of: bool,
        is_await: bool,
    ) {
        self.reset_completion();
        let index = self.push_breakable(true, true);

        // ForIn/OfHeadEvaluation: the expression sees the loop's lexical
        // names in their temporal dead zone.
        let lexical_names: Vec<(crate::value::JsString, bool)> = match left {
            ForHead::Var { kind, target, .. } if kind.is_lexical() => {
                let is_const = *kind != VarKind::Let;
                early::bound_names(target)
                    .into_iter()
                    .map(|n| (self.agent.string(n), is_const))
                    .collect()
            }
            _ => Vec::new(),
        };
        let tdz_scope = (!lexical_names.is_empty()).then(|| {
            let names = lexical_names.iter().map(|(n, _)| (*n, false)).collect();
            self.code.scopes.push(ScopeInfo {
                lexical: names,
                functions: Vec::new(),
            });
            (self.code.scopes.len() - 1) as u32
        });
        // Annex B: `for (var x = init in obj)`.
        if let ForHead::Var {
            kind: VarKind::Var,
            target: Pattern::Ident(id),
            init: Some(init),
        } = left
        {
            let name = self.string(&id.name);
            self.emit(Op::ResolveName(name));
            self.compile_named_expr(init, &Pattern::Ident(id.clone()));
            self.emit(Op::PutRef);
            self.emit(Op::Pop);
        }
        if let Some(scope) = tdz_scope {
            self.emit(Op::PushScope(scope));
            self.compile_expr(right);
            self.emit(Op::PopScope);
        } else {
            self.compile_expr(right);
        }

        let iteration_scope = (!lexical_names.is_empty()).then(|| {
            self.code.scopes.push(ScopeInfo {
                lexical: lexical_names.clone(),
                functions: Vec::new(),
            });
            (self.code.scopes.len() - 1) as u32
        });

        if is_await {
            self.emit(Op::GetAsyncIterator);
            self.scopes.push(JumpScope::Iterator {
                close: true,
                is_async: true,
            });
        } else if is_of {
            self.emit(Op::GetIterator);
            self.scopes.push(JumpScope::Iterator {
                close: true,
                is_async: false,
            });
        } else {
            self.emit(Op::ForInStart);
            self.scopes.push(JumpScope::Iterator {
                close: false,
                is_async: false,
            });
        }
        // Sync for-of: one handler around the whole loop (a throwing next()
        // marks the iterator done, so it is not closed). for await: the
        // handler covers only the body, since the awaited next() result must
        // not close the iterator either.
        let handler = (is_of && !is_await).then(|| {
            let h = self.emit(Op::PushHandler(0));
            self.scopes.push(JumpScope::Handler);
            h
        });
        self.set_continue_depth_here(index);

        let next = self.here();
        let (done, handler) = if is_await {
            // nextResult = ? Call(next); nextResult = ? Await(nextResult); checks; done?
            self.emit(Op::IteratorCallNext);
            self.emit(Op::Await);
            let done = self.emit(Op::IteratorResultUnwrap(0));
            let h = self.emit(Op::PushHandler(0));
            self.scopes.push(JumpScope::Handler);
            (done, Some(h))
        } else {
            let done = self.emit(if is_of {
                Op::IteratorNext(0)
            } else {
                Op::ForInNext(0)
            });
            (done, handler)
        };
        if let Some(scope) = iteration_scope {
            self.emit(Op::PushScope(scope));
            self.scopes.push(JumpScope::Env);
        }
        let using = match left {
            ForHead::Var {
                kind: kind @ (VarKind::Using | VarKind::AwaitUsing),
                ..
            } => Some(*kind == VarKind::AwaitUsing),
            _ => None,
        };
        match left {
            ForHead::Var { kind, target, .. } => {
                if let Some(is_await) = using {
                    // InitializeReferencedBinding(lhsRef, nextValue, hint): ? AddDisposableResource
                    // on iterationEnv.[[DisposableResourceStack]] (if it throws, the iterator is closed).
                    self.emit(Op::Dup);
                    self.emit(Op::AddDisposable(is_await));
                }
                self.compile_binding(target, kind.is_lexical())
            }
            ForHead::Assign(target) => match target {
                Pattern::Ident(id) => {
                    let name = self.string(&id.name);
                    self.emit(Op::ResolveName(name));
                    self.emit(Op::PutRef);
                    self.emit(Op::Pop);
                }
                _ => self.compile_assignment_pattern(target),
            },
        }
        match using {
            // Let result be Completion(Evaluation of stmt); if lhsKind is lexicalBinding,
            // set result to Completion(DisposeResources(iterationEnv.[[DisposableResourceStack]], result)).
            Some(is_await) => self.compile_disposing(is_await, |c| c.compile_stmt(body)),
            None => self.compile_stmt(body),
        }
        if iteration_scope.is_some() {
            self.scopes.pop();
            self.emit(Op::PopScope);
        }
        if is_await {
            self.scopes.pop();
            self.emit(Op::PopHandler);
        }
        self.patch_continues_here(index);
        self.emit(Op::Jump(next));

        // Done: the iterator is exhausted.
        self.patch(done);
        if handler.is_some() && !is_await {
            self.scopes.pop();
            self.emit(Op::PopHandler);
        }
        self.scopes.pop();
        self.emit(Op::PopIterator);
        let end = self.emit(Op::Jump(0));
        match handler {
            Some(h) if is_await => {
                // AsyncIteratorClose(iteratorRecord, throw completion): errors
                // while closing are ignored and the original exception rethrown.
                self.patch(h);
                let skip = self.emit(Op::AsyncIteratorCloseForThrow(0));
                let ignore = self.emit(Op::PushHandler(0));
                self.emit(Op::Await);
                self.emit(Op::PopHandler);
                self.emit(Op::Pop);
                let to_throw = self.emit(Op::Jump(0));
                self.patch(ignore);
                self.emit(Op::Pop);
                self.emit(Op::Pop);
                self.patch(skip);
                self.patch(to_throw);
                self.emit(Op::Throw);
            }
            Some(h) => {
                self.patch(h);
                self.emit(Op::IteratorCloseForThrow);
                self.emit(Op::Throw);
            }
            None => {}
        }
        self.patch(end);
        self.pop_breakable(None);
    }

    fn compile_switch(&mut self, discriminant: &Expr, cases: &[SwitchCase]) {
        self.reset_completion();
        self.compile_expr(discriminant);
        self.scopes.push(JumpScope::Stack(1));
        let all: Vec<Stmt> = cases.iter().flat_map(|c| c.body.iter().cloned()).collect();
        let scope = self.block_scope(&all);
        if let Some(scope) = scope {
            self.emit(Op::PushScope(scope));
            self.scopes.push(JumpScope::Env);
        }
        // 7. Let blockResult be Completion(CaseBlockEvaluation of CaseBlock with argument switchValue).
        // 8. Assert: blockEnv.[[DisposableResourceStack]] is an empty List (case clauses cannot declare `using`).
        self.compile_case_block(cases);
        if scope.is_some() {
            self.scopes.pop();
            self.emit(Op::PopScope);
        }
        self.scopes.pop();
        self.emit(Op::Pop);
    }

    /// CaseBlockEvaluation, with the switch value on the stack.
    fn compile_case_block(&mut self, cases: &[SwitchCase]) {
        self.push_breakable(false, true);
        let mut case_jumps = Vec::new();
        for c in cases {
            if let Some(test) = &c.test {
                self.emit(Op::Dup);
                self.compile_expr(test);
                self.emit(Op::Binary(BinOp::StrictEq));
                case_jumps.push(Some(self.emit(Op::JumpIfTrue(0))));
            } else {
                case_jumps.push(None);
            }
        }
        let no_match = self.emit(Op::Jump(0));
        let mut default_target = None;
        for (c, j) in cases.iter().zip(case_jumps) {
            match j {
                Some(j) => self.patch(j),
                None => default_target = Some(self.here()),
            }
            self.compile_statements(&c.body);
        }
        match default_target {
            Some(t) => self.patch_to(no_match, t),
            None => self.patch(no_match),
        }
        self.pop_breakable(None);
    }

    fn compile_try(
        &mut self,
        block: &Block,
        handler: Option<&CatchClause>,
        finalizer: Option<&Block>,
    ) {
        self.reset_completion();
        let finally_index = finalizer.map(|_| {
            self.code.finally_targets.push(Vec::new());
            let index = (self.code.finally_targets.len() - 1) as u32;
            self.scopes.push(JumpScope::Finally {
                entry_jumps: Vec::new(),
                continuations: Vec::new(),
            });
            index
        });

        // try { block }
        let try_handler = self.emit(Op::PushHandler(0));
        self.scopes.push(JumpScope::Handler);
        self.compile_block(block);
        self.scopes.pop();
        self.emit(Op::PopHandler);

        // catch (param) { body }
        let mut throw_into_finally = vec![try_handler];
        if let Some(h) = handler {
            let after = self.emit(Op::Jump(0));
            self.patch(try_handler);
            throw_into_finally.clear();
            if finally_index.is_some() {
                throw_into_finally.push(self.emit(Op::PushHandler(0)));
                self.scopes.push(JumpScope::Handler);
            }
            match &h.param {
                Some(param) => {
                    let lexical = early::bound_names(param)
                        .into_iter()
                        .map(|n| (self.agent.string(n), false))
                        .collect();
                    self.code.scopes.push(ScopeInfo {
                        lexical,
                        functions: Vec::new(),
                    });
                    let scope = (self.code.scopes.len() - 1) as u32;
                    self.emit(Op::PushScope(scope));
                    self.scopes.push(JumpScope::Env);
                    self.compile_binding(param, true);
                    self.compile_block(&h.body);
                    self.scopes.pop();
                    self.emit(Op::PopScope);
                }
                None => {
                    self.emit(Op::Pop);
                    self.compile_block(&h.body);
                }
            }
            if finally_index.is_some() {
                self.scopes.pop();
                self.emit(Op::PopHandler);
            }
            self.patch(after);
        }

        // finally { finalizer }
        if let (Some(index), Some(fin)) = (finally_index, finalizer) {
            let Some(JumpScope::Finally {
                entry_jumps,
                continuations,
                ..
            }) = self.scopes.pop()
            else {
                unreachable!("finally scope expected")
            };
            // Normal completion: [undefined 0].
            self.emit(Op::Undefined);
            self.emit(Op::Number(0.0));
            let to_finally = self.emit(Op::Jump(0));
            // Throw completion: [exception 1].
            for h in throw_into_finally {
                self.patch(h);
            }
            self.emit(Op::Number(1.0));
            self.patch(to_finally);
            for j in entry_jumps {
                self.patch(j);
            }
            self.scopes.push(JumpScope::Stack(2));
            let completion = std::mem::replace(&mut self.completion, false);
            self.compile_block(fin);
            self.completion = completion;
            self.scopes.pop();
            self.emit(Op::EndFinally(index));
            if !continuations.is_empty() {
                let skip = self.emit(Op::Jump(0));
                self.emit_finally_continuations(index, continuations);
                self.patch(skip);
            }
        } else if handler.is_none() {
            // try without catch or finally cannot parse.
            self.patch(try_handler);
            self.emit(Op::Throw);
        }
    }
}
