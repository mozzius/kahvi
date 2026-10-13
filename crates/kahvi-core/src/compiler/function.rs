//! Function bodies, parameters and the declaration lists that the
//! declaration-instantiation algorithms (in `vm::instantiate`) consume.

use std::sync::Arc;

use kahvi_parser::ast::{self, Function, FunctionBody, FunctionKind, Pattern, Stmt, VarKind};
use kahvi_parser::early;

use super::{CodeKind, Compiler, FunctionCode, FunctionFlavor, Op, ScopeInfo};
use crate::agent::Agent;
use crate::function::ThisMode;
use crate::value::JsString;

/// Compiles a function's body into its own [`FunctionCode`].
pub(super) fn compile_function(
    agent: &mut Agent,
    f: &Function,
    source: Arc<str>,
    script: u32,
    is_expression: bool,
) -> Arc<FunctionCode> {
    let mut c = Compiler::new(agent, CodeKind::Function, source, script, f.span);
    c.code.strict = f.strict;
    c.code.is_async = f.is_async;
    c.code.is_generator = f.is_generator;
    c.code.flavor = match f.kind {
        FunctionKind::Normal => FunctionFlavor::Normal,
        FunctionKind::Arrow => FunctionFlavor::Arrow,
        FunctionKind::Method => FunctionFlavor::Method,
        FunctionKind::Getter => FunctionFlavor::Getter,
        FunctionKind::Setter => FunctionFlavor::Setter,
        FunctionKind::ClassConstructor => FunctionFlavor::ClassConstructor,
        FunctionKind::DerivedConstructor => FunctionFlavor::DerivedConstructor,
        FunctionKind::ClassFieldInit => FunctionFlavor::ClassFieldInit,
    };
    c.code.this_mode = if f.kind == FunctionKind::Arrow {
        ThisMode::Lexical
    } else if f.strict {
        ThisMode::Strict
    } else {
        ThisMode::Global
    };
    c.code.name = f.id.as_ref().map(|id| c.agent.string(&id.name));
    c.code.binds_own_name = is_expression && f.id.is_some();
    c.code.length = f.expected_args;
    c.code.uses_arguments = f.uses_arguments;
    c.code.has_direct_eval = f.has_direct_eval;

    // Parameters (BoundNames of the formals, in order).
    let mut names: Vec<JsString> = Vec::new();
    for p in &f.params {
        for n in early::bound_names(&p.target) {
            let s = c.string(n);
            names.push(s);
        }
    }
    if let Some(r) = &f.rest {
        for n in early::bound_names(r) {
            let s = c.string(n);
            names.push(s);
        }
    }
    let has_duplicates = names
        .iter()
        .enumerate()
        .any(|(i, n)| names[..i].contains(n));
    c.code.params.names = names;
    c.code.params.simple = f.simple_params;
    c.code.params.has_duplicates = has_duplicates;
    c.code.params.has_expressions = f
        .params
        .iter()
        .any(|p| p.default.is_some() || pattern_has_expressions(&p.target))
        || f.rest.as_ref().is_some_and(pattern_has_expressions);

    // Body declarations.
    let empty: Vec<Stmt> = Vec::new();
    let body: &[Stmt] = match &f.body {
        FunctionBody::Block(b) => b,
        FunctionBody::Expr(_) => &empty,
    };
    c.collect_declarations(body);

    // FunctionDeclarationInstantiation steps 24-28: bind the parameters.
    c.compile_parameters(f);
    // Steps 29-36.
    c.emit(Op::InstantiateFunctionBody);
    if f.is_generator {
        c.emit(Op::GeneratorStart);
    }

    match &f.body {
        FunctionBody::Block(stmts) => {
            c.compile_statements(stmts);
            c.emit(Op::Undefined);
            c.emit(Op::Return);
        }
        FunctionBody::Expr(e) => {
            c.compile_expr(e);
            c.emit(Op::Return);
        }
    }
    c.finish()
}

/// ContainsExpression of a binding pattern.
fn pattern_has_expressions(p: &Pattern) -> bool {
    match p {
        Pattern::Ident(_) => false,
        Pattern::Expr(_) => true,
        Pattern::Object { props, rest, .. } => {
            props.iter().any(|prop| {
                matches!(prop.key, ast::PropKey::Computed(_))
                    || prop.value.default.is_some()
                    || pattern_has_expressions(&prop.value.target)
            }) || rest.as_deref().is_some_and(pattern_has_expressions)
        }
        Pattern::Array { elements, rest, .. } => {
            elements
                .iter()
                .flatten()
                .any(|e| e.default.is_some() || pattern_has_expressions(&e.target))
                || rest.as_deref().is_some_and(pattern_has_expressions)
        }
    }
}

impl Compiler<'_> {
    /// Script and eval code: declarations, then the statements.
    pub(super) fn compile_toplevel(&mut self, body: &[Stmt]) {
        self.collect_declarations(body);
        self.emit(Op::Undefined);
        self.emit(Op::SetCompletion);
        self.compile_statements(body);
        // The frame returns its completion value.
        self.emit(Op::Undefined);
        self.emit(Op::Return);
    }

    /// Fills `code.decls` from a function/script body's top-level statements.
    pub(super) fn collect_declarations(&mut self, body: &[Stmt]) {
        // VarDeclaredNames (without functions).
        let mut var_names: Vec<JsString> = Vec::new();
        for d in early::var_declared_names(body) {
            let s = self.string(d.name);
            if !var_names.contains(&s) {
                var_names.push(s);
            }
        }
        // Var-scoped function declarations: the last one for a name wins,
        // positioned where its name first appears (step 14 walks in reverse).
        let mut functions: Vec<(JsString, u32)> = Vec::new();
        for f in early::top_level_functions(body).into_iter().rev() {
            let name = f.id.as_ref().map(|i| i.name.as_str()).unwrap_or("");
            let name = self.string(name);
            if functions.iter().any(|(n, _)| *n == name) {
                continue;
            }
            let index = self.compile_nested_function(f, false);
            functions.insert(0, (name, index));
        }
        let lexical = self.lexical_declarations(body);
        self.code.decls.var_names = var_names;
        self.code.decls.functions = functions;
        self.code.decls.lexical = lexical;
    }

    /// Top-level lexical declarations (let/const/class) as (name, is_const).
    fn lexical_declarations(&mut self, body: &[Stmt]) -> Vec<(JsString, bool)> {
        let mut out = Vec::new();
        for s in body {
            match s {
                Stmt::Var(d) if d.kind.is_lexical() => {
                    let is_const = d.kind != VarKind::Let;
                    for decl in &d.declarations {
                        for n in early::bound_names(&decl.target) {
                            let s = self.string(n);
                            out.push((s, is_const));
                        }
                    }
                }
                Stmt::Class(cl) => {
                    if let Some(id) = &cl.id {
                        let s = self.string(&id.name);
                        out.push((s, false));
                    }
                }
                _ => {}
            }
        }
        out
    }

    /// The scope of a block or case block: its lexical declarations and the
    /// function declarations directly in it. Returns `None` if empty.
    pub(super) fn block_scope(&mut self, body: &[Stmt]) -> Option<u32> {
        let lexical = self.lexical_declarations(body);
        let mut functions: Vec<(JsString, u32)> = Vec::new();
        for s in body {
            let mut s = s;
            while let Stmt::Labeled { body, .. } = s {
                s = body;
            }
            if let Stmt::Function(f) = s {
                let name = f.id.as_ref().map(|i| i.name.as_str()).unwrap_or("");
                let name = self.string(name);
                let index = self.compile_nested_function(f, false);
                // Sloppy duplicates: the last declaration wins.
                functions.retain(|(n, _)| *n != name);
                functions.push((name, index));
            }
        }
        if lexical.is_empty() && functions.is_empty() {
            return None;
        }
        self.code.scopes.push(ScopeInfo { lexical, functions });
        Some((self.code.scopes.len() - 1) as u32)
    }

    pub(super) fn compile_nested_function(&mut self, f: &Function, is_expression: bool) -> u32 {
        let source = self.code.source.clone();
        let script = self.code.script;
        let code = compile_function(self.agent, f, source, script, is_expression);
        self.add_function(code)
    }

    /// Parameter binding (IteratorBindingInitialization of the formals).
    fn compile_parameters(&mut self, f: &Function) {
        // With duplicates, parameters are bound with PutValue semantics.
        let init = !self.code.params.has_duplicates;
        for (i, p) in f.params.iter().enumerate() {
            self.emit(Op::GetArg(i as u32));
            if let Some(default) = &p.default {
                let j = self.emit(Op::JumpIfNotUndefined(0));
                self.emit(Op::Pop);
                self.compile_named_expr(default, &p.target);
                self.patch(j);
            }
            self.compile_binding(&p.target, init);
        }
        if let Some(rest) = &f.rest {
            self.emit(Op::GetRestArgs(f.params.len() as u32));
            self.compile_binding(rest, init);
        }
    }
}
