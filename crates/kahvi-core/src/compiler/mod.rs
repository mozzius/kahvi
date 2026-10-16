//! Compiles the AST into bytecode (one [`FunctionCode`] per function body).
//!
//! The compiler is split by syntax: `stmt.rs` (statements and control flow),
//! `expr.rs` (expressions), `pattern.rs` (binding and assignment patterns),
//! `function.rs` (function bodies and declarations), `class.rs` (classes).
//! Anything not compiled yet emits [`Op::NotImplemented`] so it fails loudly
//! at run time rather than at compile time.

pub mod bytecode;
mod class;
pub(crate) mod codec;
mod expr;
mod function;
mod pattern;
mod stmt;

use std::sync::Arc;

pub use bytecode::*;
use kahvi_parser::ast::{self, Span};

use crate::agent::Agent;
use crate::function::ThisMode;
use crate::value::JsString;

/// Where a `break`/`continue`/`return` has to go, and what leaving each
/// enclosing construct requires.
#[derive(Debug)]
enum JumpScope {
    /// A loop, switch or labelled statement that `break` (and for loops,
    /// `continue`) can target.
    Breakable {
        labels: Vec<String>,
        is_loop: bool,
        /// A plain `break;` targets this (loops and switch, not labelled blocks).
        unlabelled_break: bool,
        break_jumps: Vec<usize>,
        continue_jumps: Vec<usize>,
        /// `continue` leaves only the scopes above this depth (for-of keeps
        /// its iterator open across iterations).
        continue_depth: usize,
    },
    /// A lexical environment pushed by the code; leaving pops it.
    Env,
    /// A `with` environment (same exit action as `Env`).
    With,
    /// An exception handler; leaving pops it.
    Handler,
    /// An iterator on the iterator stack. for-of closes it on exit, for-in
    /// and destructuring just pop it.
    Iterator { close: bool, is_async: bool },
    /// Values left on the operand stack (switch discriminant, finally state).
    Stack(u32),
    /// A finally block: jumps leaving its try/catch go through it first.
    Finally {
        entry_jumps: Vec<usize>,
        continuations: Vec<Continuation>,
    },
    /// A private environment pushed by a class body.
    PrivateScope,
}

/// What a jump routed through a finally block does once the block ends.
#[derive(Debug, Clone, Copy)]
enum Continuation {
    Break { scope: usize },
    Continue { scope: usize },
    Return,
}

/// Compiles one function body (or script / eval code).
pub(crate) struct Compiler<'a> {
    agent: &'a mut Agent,
    code: FunctionCode,
    scopes: Vec<JumpScope>,
    /// Labels for the next statement (`a: b: while (...)`).
    pending_labels: Vec<String>,
    /// Emit completion values (script and eval code).
    completion: bool,
    /// Jumps to the short-circuit exit of each enclosing optional chain.
    optional_exits: Vec<Vec<usize>>,
    /// Binding caches handed out so far (`name_cache`).
    name_cache_count: u32,
    /// Annex B.3.2: the block-level function declarations whose evaluation
    /// sets the var binding of their name (`Op::AnnexBFunction`).
    annex_b: Vec<*const ast::Function>,
    /// Compiling the bindings as frame locals (`FunctionCode::locals`).
    locals: Option<LocalScopes>,
}

/// The frame locals of code compiled with them: every slot so far, and the
/// slots of the scopes around the instruction being emitted.
#[derive(Default)]
struct LocalScopes {
    slots: Vec<LocalInfo>,
    /// For each scope, innermost last: (name, slot).
    scopes: Vec<Vec<(JsString, u32)>>,
    /// Something the locals cannot represent was emitted.
    failed: bool,
}

impl LocalScopes {
    /// The slot that `name` resolves to here, if it is a local.
    fn resolve(&self, name: JsString) -> Option<u32> {
        self.scopes
            .iter()
            .rev()
            .find_map(|s| s.iter().find(|(n, _)| *n == name).map(|&(_, slot)| slot))
    }

    /// A new scope with a fresh slot for each binding. Returns the first slot.
    fn push(&mut self, bindings: impl IntoIterator<Item = (JsString, LocalKind)>) -> u32 {
        let start = self.slots.len() as u32;
        let mut scope = Vec::new();
        for (name, kind) in bindings {
            if scope.iter().any(|&(n, _)| n == name) {
                continue;
            }
            scope.push((name, self.slots.len() as u32));
            self.slots.push(LocalInfo { name, kind });
        }
        self.scopes.push(scope);
        start
    }
}

/// Errors detected while compiling (things the parser accepts but that are
/// early errors only in some contexts are reported by the parser; this is
/// for internal limits).
pub type CompileResult<T> = Result<T, String>;

impl<'a> Compiler<'a> {
    fn new(
        agent: &'a mut Agent,
        kind: CodeKind,
        source: Arc<str>,
        script: u32,
        span: Span,
    ) -> Compiler<'a> {
        let constants = agent
            .gc
            .compiling
            .as_ref()
            .expect("code is compiled inside pinned()")
            .constants
            .clone();
        Compiler {
            agent,
            code: FunctionCode {
                kind,
                flavor: FunctionFlavor::Normal,
                is_async: false,
                is_generator: false,
                strict: false,
                this_mode: ThisMode::Global,
                name: None,
                length: 0,
                params: Params::default(),
                decls: Declarations::default(),
                uses_arguments: false,
                has_direct_eval: false,
                envs_die_with_call: false,
                locals: None,
                skips_function_env: false,
                binds_own_name: false,
                ops: Vec::new(),
                positions: Vec::new(),
                functions: Vec::new(),
                scopes: Vec::new(),
                templates: Vec::new(),
                classes: Vec::new(),
                messages: Vec::new(),
                finally_targets: Vec::new(),
                strict_ranges: Vec::new(),
                source,
                span: (span.start, span.end),
                script,
                constants,
                name_caches: Box::default(),
                prop_caches: Box::default(),
                lazy: None,
            },
            scopes: Vec::new(),
            pending_labels: Vec::new(),
            completion: false,
            optional_exits: Vec::new(),
            name_cache_count: 0,
            annex_b: Vec::new(),
            locals: None,
        }
    }

    fn finish(mut self) -> Arc<FunctionCode> {
        let c = &self.code;
        let envs_die_with_call = c.kind == CodeKind::Function
            && !c.is_async
            && !c.is_generator
            && !c.uses_arguments
            && !c.has_direct_eval
            && c.functions.is_empty()
            && c.classes.is_empty()
            && !c.ops.iter().any(|op| matches!(op, Op::PushWith));
        self.code.envs_die_with_call = envs_die_with_call;
        self.code.name_caches = (0..self.name_cache_count)
            .map(|_| std::sync::atomic::AtomicU64::new(0))
            .collect();
        let mut prop_caches = 0;
        for op in &mut self.code.ops {
            if let Op::GetProp(_, ic) | Op::GetPropForCall(_, ic) | Op::SetProp(_, ic) = op {
                *ic = prop_caches;
                prop_caches += 1;
            }
        }
        self.code.prop_caches = (0..prop_caches).map(|_| PropCache::default()).collect();
        Arc::new(self.code)
    }

    /// A binding cache for a name instruction about to be emitted, unless
    /// the environment chain here can gain bindings at run time.
    fn name_cache(&mut self) -> u32 {
        let dynamic = self.code.kind == CodeKind::Eval
            || self
                .agent
                .gc
                .compiling
                .as_ref()
                .is_some_and(|u| u.dynamic_scopes > 0);
        if dynamic {
            return NO_NAME_CACHE;
        }
        self.name_cache_count += 1;
        self.name_cache_count - 1
    }

    /// Compiles `f` with the environments around it marked dynamic (a `with`
    /// object, a function with a direct eval): name instructions inside get
    /// no binding cache, nor do those of the functions nested in it.
    fn dynamic_scope<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T {
        self.agent
            .gc
            .compiling
            .as_mut()
            .expect("compiling")
            .dynamic_scopes += 1;
        let r = f(self);
        self.agent
            .gc
            .compiling
            .as_mut()
            .expect("compiling")
            .dynamic_scopes -= 1;
        r
    }

    // ---- emitting -----------------------------------------------------------

    fn emit(&mut self, op: Op) -> usize {
        if self.locals.is_some() {
            self.emit_with_locals(op);
            return self.code.ops.len().wrapping_sub(1);
        }
        self.code.ops.push(op);
        self.code.ops.len() - 1
    }

    /// `emit` for code compiled with frame locals: the name instructions for
    /// locals become local instructions, and scopes allocate slots instead
    /// of environments.
    fn emit_with_locals(&mut self, op: Op) {
        let locals = self.locals.as_mut().expect("compiling with locals");
        let ops = &mut self.code.ops;
        match op {
            Op::GetName(name, _) => match locals.resolve(name) {
                Some(slot) => ops.push(Op::GetLocal(slot)),
                None => ops.push(op),
            },
            // typeof of a binding in its dead zone throws, as GetValue does.
            Op::TypeofName(name, _) => match locals.resolve(name) {
                Some(slot) => ops.extend([Op::GetLocal(slot), Op::Typeof]),
                None => ops.push(op),
            },
            // A declarative record's WithBaseObject() is undefined.
            Op::GetNameForCall(name, _) => match locals.resolve(name) {
                Some(slot) => ops.extend([Op::GetLocal(slot), Op::Undefined]),
                None => ops.push(op),
            },
            Op::ResolveName(name, _) => match locals.resolve(name) {
                Some(slot) => ops.push(Op::ResolveLocal(slot)),
                None => ops.push(op),
            },
            // InitName targets a binding of the running lexical environment,
            // which here is always a local.
            Op::InitName(name) => match locals.resolve(name) {
                Some(slot) => ops.push(Op::InitLocal(slot)),
                None => {
                    locals.failed = true;
                    ops.push(op);
                }
            },
            // A declarative record's DeleteBinding(N) of a binding it has
            // returns false (only non-strict code can delete a name).
            Op::DeleteName(name) => match locals.resolve(name) {
                Some(_) => ops.push(Op::False),
                None => ops.push(op),
            },
            Op::PushScope(i) => {
                let scope = &self.code.scopes[i as usize];
                if !scope.functions.is_empty() {
                    locals.failed = true;
                }
                let bindings = scope.lexical.iter().map(|&(name, is_const)| {
                    let kind = if is_const {
                        LocalKind::Const
                    } else {
                        LocalKind::Let
                    };
                    (name, kind)
                });
                let start = locals.push(bindings);
                let count = locals.slots.len() as u32 - start;
                if count > 0 {
                    ops.push(Op::ClearLocals(start, count));
                }
            }
            Op::PopScope => {
                locals.scopes.pop();
            }
            // Without closures nothing can tell a per-iteration copy from
            // the last iteration's bindings.
            Op::CopyScope => {}
            // These need environments (or an environment's resource stack).
            Op::PushWith
            | Op::PushEmptyScope
            | Op::AnnexBFunction(_)
            | Op::AddDisposable(_)
            | Op::PushPrivateScope(_) => {
                locals.failed = true;
                ops.push(op);
            }
            _ => ops.push(op),
        }
    }

    /// Records that the following instructions come from source offset `pos`.
    fn position(&mut self, span: Span) {
        let pc = self.code.ops.len() as u32;
        match self.code.positions.last_mut() {
            Some(last) if last.0 == pc => last.1 = span.start,
            _ => self.code.positions.push((pc, span.start)),
        }
    }

    fn here(&self) -> Label {
        self.code.ops.len() as Label
    }

    /// Points the jump at `at` to the current position.
    fn patch(&mut self, at: usize) {
        let target = self.here();
        self.patch_to(at, target);
    }

    fn patch_to(&mut self, at: usize, target: Label) {
        match &mut self.code.ops[at] {
            Op::Jump(l)
            | Op::JumpIfFalse(l)
            | Op::JumpIfTrue(l)
            | Op::JumpIfFalseKeep(l)
            | Op::JumpIfTrueKeep(l)
            | Op::JumpIfNullishKeep(l)
            | Op::JumpIfNotNullishKeep(l)
            | Op::JumpIfNotUndefined(l)
            | Op::PushHandler(l)
            | Op::IteratorNext(l)
            | Op::IteratorResultUnwrap(l)
            | Op::JumpIfDelegateKind(_, l)
            | Op::AsyncDelegateCall(l)
            | Op::AsyncDelegateReturn(l)
            | Op::AsyncDelegateResult(l)
            | Op::AsyncIteratorClose(l)
            | Op::AsyncIteratorCloseForThrow(l)
            | Op::ForInNext(l)
            | Op::DisposeStep(l)
            | Op::Yield(l)
            | Op::YieldDelegate(l) => *l = target,
            op => unreachable!("patching a non-jump {op:?}"),
        }
    }

    fn string(&mut self, s: &str) -> JsString {
        self.agent.string(s)
    }

    fn utf16(&mut self, s: &[u16]) -> JsString {
        self.agent.intern(s)
    }

    fn message(&mut self, msg: impl Into<String>) -> u32 {
        self.code.messages.push(msg.into());
        (self.code.messages.len() - 1) as u32
    }

    /// Emits a run-time "not implemented" failure for `what`.
    fn not_implemented(&mut self, what: &str) {
        let m = self.message(what);
        self.emit(Op::NotImplemented(m));
    }

    fn throw_error(&mut self, kind: ErrorType, msg: &str) {
        let m = self.message(msg);
        self.emit(Op::ThrowError(kind, m));
    }

    // ---- jumps across scopes ------------------------------------------------------

    /// Emits the exit actions for leaving every scope above `depth` (exclusive),
    /// innermost first. Returns `true` if a finally block took over (the
    /// jump continues from the finally's continuation stub).
    ///
    /// `value_on_stack` says whether the jump carries a value on the operand
    /// stack (a return value); a finally block needs one, so `undefined` is
    /// pushed for break/continue.
    fn exit_scopes(&mut self, depth: usize, cont: Continuation, value_on_stack: bool) -> bool {
        let mut i = self.scopes.len();
        while i > depth {
            i -= 1;
            match &self.scopes[i] {
                // (A scope of locals has no environment to leave.)
                JumpScope::Env | JumpScope::With if self.locals.is_none() => {
                    self.code.ops.push(Op::PopScope);
                }
                JumpScope::Env | JumpScope::With => {}
                JumpScope::Handler => {
                    self.emit(Op::PopHandler);
                }
                JumpScope::Iterator {
                    close: true,
                    is_async: false,
                } => {
                    self.emit(Op::IteratorClose);
                }
                JumpScope::Iterator {
                    close: true,
                    is_async: true,
                } => {
                    // AsyncIteratorClose(iteratorRecord, completion): await return()'s result.
                    let skip = self.emit(Op::AsyncIteratorClose(0));
                    self.emit(Op::Await);
                    self.emit(Op::IteratorCheckObject);
                    self.patch(skip);
                }
                JumpScope::Iterator { close: false, .. } => {
                    self.emit(Op::PopIterator);
                }
                JumpScope::Stack(n) => {
                    let n = *n;
                    for _ in 0..n {
                        if value_on_stack {
                            self.emit(Op::Swap);
                        }
                        self.emit(Op::Pop);
                    }
                }
                JumpScope::PrivateScope => {
                    self.emit(Op::PopPrivateScope);
                }
                JumpScope::Breakable { .. } => {}
                JumpScope::Finally { .. } => {
                    if !value_on_stack {
                        self.emit(Op::Undefined);
                    }
                    let JumpScope::Finally { continuations, .. } = &mut self.scopes[i] else {
                        unreachable!()
                    };
                    continuations.push(cont);
                    let kind = continuations.len() as u32 + 1;
                    self.emit(Op::Number(kind as f64));
                    let j = self.emit(Op::Jump(0));
                    let JumpScope::Finally { entry_jumps, .. } = &mut self.scopes[i] else {
                        unreachable!()
                    };
                    entry_jumps.push(j);
                    return true;
                }
            }
        }
        false
    }

    /// `break` / `break label`.
    fn emit_break(&mut self, label: Option<&str>) -> Result<(), String> {
        let target = self
            .find_breakable(label, false)
            .ok_or_else(|| "break target not found".to_string())?;
        self.jump_to_breakable(target, false);
        Ok(())
    }

    /// `continue` / `continue label`.
    fn emit_continue(&mut self, label: Option<&str>) -> Result<(), String> {
        let target = self
            .find_breakable(label, true)
            .ok_or_else(|| "continue target not found".to_string())?;
        self.jump_to_breakable(target, true);
        Ok(())
    }

    fn jump_to_breakable(&mut self, target: usize, is_continue: bool) {
        let cont = if is_continue {
            Continuation::Continue { scope: target }
        } else {
            Continuation::Break { scope: target }
        };
        let depth = match &self.scopes[target] {
            JumpScope::Breakable { continue_depth, .. } if is_continue => *continue_depth,
            _ => target + 1,
        };
        if self.exit_scopes(depth, cont, false) {
            return;
        }
        let j = self.emit(Op::Jump(0));
        if let JumpScope::Breakable {
            break_jumps,
            continue_jumps,
            ..
        } = &mut self.scopes[target]
        {
            if is_continue {
                continue_jumps.push(j)
            } else {
                break_jumps.push(j)
            }
        }
    }

    fn find_breakable(&self, label: Option<&str>, is_continue: bool) -> Option<usize> {
        self.scopes.iter().rposition(|s| match s {
            JumpScope::Breakable {
                labels,
                is_loop,
                unlabelled_break,
                ..
            } => match label {
                Some(l) => labels.iter().any(|x| x == l) && (!is_continue || *is_loop),
                None => {
                    if is_continue {
                        *is_loop
                    } else {
                        *unlabelled_break
                    }
                }
            },
            _ => false,
        })
    }

    /// `return` with the value on the stack.
    fn emit_return(&mut self) {
        if self.exit_scopes(0, Continuation::Return, true) {
            return;
        }
        self.emit(Op::Return);
    }

    /// Opens a breakable scope taking the pending labels.
    fn push_breakable(&mut self, is_loop: bool, unlabelled_break: bool) -> usize {
        let labels = std::mem::take(&mut self.pending_labels);
        let continue_depth = self.scopes.len() + 1;
        self.scopes.push(JumpScope::Breakable {
            labels,
            is_loop,
            unlabelled_break,
            break_jumps: Vec::new(),
            continue_jumps: Vec::new(),
            continue_depth,
        });
        self.scopes.len() - 1
    }

    /// Sets the depth `continue` exits to for the breakable scope at `index`
    /// to the current scope depth.
    fn set_continue_depth_here(&mut self, index: usize) {
        let here = self.scopes.len();
        if let JumpScope::Breakable { continue_depth, .. } = &mut self.scopes[index] {
            *continue_depth = here;
        }
    }

    /// Closes the breakable scope at `index` (which must be the top scope):
    /// patches `continue` jumps to `continue_target` and `break` jumps to
    /// the current position.
    fn pop_breakable(&mut self, continue_target: Option<Label>) {
        let Some(JumpScope::Breakable {
            break_jumps,
            continue_jumps,
            ..
        }) = self.scopes.pop()
        else {
            unreachable!("breakable scope expected")
        };
        for j in break_jumps {
            self.patch(j);
        }
        if let Some(t) = continue_target {
            for j in continue_jumps {
                self.patch_to(j, t);
            }
        }
    }

    /// Patches pending `continue` jumps of the breakable scope at `index` to
    /// the current position (for loops whose continue target is emitted
    /// after the body).
    fn patch_continues_here(&mut self, index: usize) {
        let here = self.here();
        if let JumpScope::Breakable { continue_jumps, .. } = &mut self.scopes[index] {
            let jumps = std::mem::take(continue_jumps);
            for j in jumps {
                self.patch_to(j, here);
            }
        }
    }

    /// Emits the continuation stubs after a finally block's `EndFinally`.
    /// Each stub starts with the jump's value on the stack.
    fn emit_finally_continuations(&mut self, index: u32, continuations: Vec<Continuation>) {
        let mut targets = Vec::new();
        for cont in continuations {
            targets.push(self.here());
            match cont {
                Continuation::Return => self.emit_return(),
                Continuation::Break { scope } => {
                    self.emit(Op::Pop);
                    self.jump_to_breakable(scope, false);
                }
                Continuation::Continue { scope } => {
                    self.emit(Op::Pop);
                    self.jump_to_breakable(scope, true);
                }
            }
        }
        self.code.finally_targets[index as usize] = targets;
    }

    fn add_function(&mut self, f: Arc<FunctionCode>) -> u32 {
        self.code.functions.push(f);
        (self.code.functions.len() - 1) as u32
    }
}

/// The result of compiling a script or eval.
pub fn compile_script(
    agent: &mut Agent,
    script: &ast::Script,
    source: Arc<str>,
    script_id: u32,
) -> Arc<FunctionCode> {
    pinned(agent, |agent| {
        let mut c = Compiler::new(agent, CodeKind::Script, source, script_id, script.span);
        c.code.strict = script.strict;
        c.completion = true;
        c.compile_toplevel(&script.body);
        c.finish()
    })
}

/// Compiles eval code. `strict` is the strictness of the eval code itself
/// (inherited or from a directive); `this_mode` is unused for eval.
pub fn compile_eval(
    agent: &mut Agent,
    script: &ast::Script,
    source: Arc<str>,
    script_id: u32,
) -> Arc<FunctionCode> {
    pinned(agent, |agent| {
        let mut c = Compiler::new(agent, CodeKind::Eval, source, script_id, script.span);
        c.code.strict = script.strict;
        c.completion = true;
        // A direct eval runs in environments that another eval can give
        // new var bindings, so the functions it creates get no binding
        // caches (`name_cache`) either.
        c.dynamic_scope(|c| c.compile_toplevel(&script.body));
        c.finish()
    })
}

/// Compiles module code; the module's declarations are in `decls`.
pub fn compile_module(
    agent: &mut Agent,
    module: &ast::Module,
    source: Arc<str>,
    script_id: u32,
) -> Arc<FunctionCode> {
    pinned(agent, |agent| {
        let mut c = Compiler::new(agent, CodeKind::Module, source, script_id, module.span);
        c.code.strict = true;
        c.compile_module_body(&module.body);
        // [[HasTLA]]: the module's own code awaits.
        c.code.is_async = c.code.ops.iter().any(|op| matches!(op, Op::Await));
        c.finish()
    })
}

/// Compiles a standalone function (CreateDynamicFunction).
pub fn compile_dynamic_function(
    agent: &mut Agent,
    f: &ast::Function,
    source: Arc<str>,
    script_id: u32,
) -> Arc<FunctionCode> {
    pinned(agent, |agent| {
        function::compile_function(agent, f, source, script_id, false)
    })
}

/// Compiles a lazy function (`FunctionCode::lazy`) from its reparsed AST.
pub fn compile_lazy_function(
    agent: &mut Agent,
    f: &ast::Function,
    stub: &FunctionCode,
    lazy: &LazyCode,
) -> Arc<FunctionCode> {
    pinned(agent, |agent| {
        agent
            .gc
            .compiling
            .as_mut()
            .expect("compiling")
            .dynamic_scopes = lazy.dynamic_scopes;
        function::compile_function(
            agent,
            f,
            stub.source.clone(),
            stub.script,
            lazy.is_expression,
        )
    })
}

/// The code cached as `bytes` (`codec::encode` of an earlier compilation of
/// the same function) for lazy function `stub`, as its own compilation unit
/// like [`compile_lazy_function`]; `None` if `bytes` does not decode.
pub fn decode_lazy_function(
    agent: &mut Agent,
    bytes: &[u8],
    stub: &FunctionCode,
) -> Option<Arc<FunctionCode>> {
    pinned(agent, |agent| {
        let constants = agent
            .gc
            .compiling
            .as_ref()
            .expect("compiling")
            .constants
            .clone();
        codec::decode(agent, bytes, stub, &constants).map(Arc::new)
    })
}

/// Compiles one unit: the strings and BigInts `f` creates or looks up become
/// the unit's constants, shared by every `FunctionCode` it produces and
/// traced from whatever holds that code (docs/GC.md).
fn pinned<T>(agent: &mut Agent, f: impl FnOnce(&mut Agent) -> T) -> T {
    let outer = agent
        .gc
        .compiling
        .replace(crate::gc::CompileUnit::default());
    let r = f(agent);
    let unit = std::mem::replace(&mut agent.gc.compiling, outer).expect("the unit compiled above");
    use crate::value::{BigInt, Value};
    let mut strings = unit.strings;
    strings.sort_unstable();
    strings.dedup();
    let mut bigints = unit.bigints;
    bigints.sort_unstable();
    bigints.dedup();
    let values: Vec<Value> = strings
        .into_iter()
        .map(|s| Value::String(JsString(s)))
        .chain(bigints.into_iter().map(|b| Value::BigInt(BigInt(b))))
        .collect();
    unit.constants.set(values.into_boxed_slice());
    r
}
