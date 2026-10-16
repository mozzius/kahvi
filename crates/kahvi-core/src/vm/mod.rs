//! The bytecode interpreter.
//!
//! A [`Frame`] holds everything a running function body needs (operand
//! stack, environments, handlers, iterators), so suspending a generator is
//! just keeping its frame. Calls from JavaScript to JavaScript recurse on
//! the Rust stack through [`call_ecma_function`].

mod async_from_sync;
mod async_function;
mod async_generator;
mod call;
mod dynamic_function;
mod eval;
mod generator;
mod instantiate;
mod module;
mod operators;

use std::collections::VecDeque;
use std::sync::Arc;

pub use async_from_sync::{
    async_from_sync_next, async_from_sync_return, async_from_sync_throw,
    create_async_from_sync_iterator,
};
pub use async_generator::{
    AsyncGeneratorState, async_generator_next, async_generator_return, async_generator_throw,
};
pub use call::{
    call_ecma_function, construct_ecma_function, create_closure, define_field,
    initialize_instance_elements, private_get, private_set,
};
pub use dynamic_function::{DynamicFunctionKind, create_dynamic_function};
pub use eval::{ParsedScript, evaluate_script, line_col, parse_script, perform_eval};
pub use generator::{GeneratorData, GeneratorState, generator_resume, generator_resume_abrupt};
pub use module::{
    HostLoadModule, LoadedModuleSource, ModuleRecord, ModuleStatus, evaluate as evaluate_module,
    get_active_script_or_module, get_module_namespace, link as link_module, load_requested_modules,
    module_namespace_get, parse_module, run_module,
};
pub use operators::{apply_binary_operator, number_exponentiate, type_of};

use crate::ScriptOrModule;
use crate::agent::Agent;
use crate::compiler::{BinOp, CodeKind, ErrorType, FunctionCode, LocalKind, Op};
use crate::env::{EnvId, Environment};
use crate::error::{JsError, JsResult};
use crate::function::{PrivateName, ThisMode};
use crate::object::{ObjectKind, Property, PropertyKey};
use crate::ops::dispose::{DisposeKind, DisposeResources, DisposeStep};
use crate::ops::{self, IteratorRecord};
use crate::realm::RealmId;
use crate::value::{JsString, Object, Value};

/// A loaded script, module or eval code.
#[derive(Debug)]
pub struct ScriptRecord {
    pub realm: RealmId,
    pub source: Arc<str>,
    /// The file name or URL the host gave, for error messages.
    pub name: String,
    /// The module this is the source of, for module code.
    pub module: Option<u32>,
    /// An eval or dynamic function's record: freed (and its index reused by
    /// a record with the same realm and name) once no code refers to it.
    pub collectable: bool,
    /// What parsing the source learned about its lazy function bodies, for
    /// reparsing them (`kahvi_parser::preparse`).
    pub preparse: Option<Arc<kahvi_parser::preparse::Preparse>>,
    /// Byte offsets of the starts of the source's lines, built the first
    /// time a position in it is turned into a line and column (an error
    /// stack), so later ones need not scan the source.
    pub line_starts: std::sync::OnceLock<Box<[u32]>>,
    /// The source's key in `code_cache`, if it is large enough to cache.
    pub(crate) cache_key: Option<crate::code_cache::Key>,
}

/// The [[ScriptOrModule]] of code compiled from `agent.scripts[script]`.
pub fn script_or_module_of(agent: &Agent, script: u32) -> ScriptOrModule {
    match agent.scripts[script as usize].module {
        Some(m) => ScriptOrModule::Module(m),
        None => ScriptOrModule::Script(script),
    }
}

/// A PrivateEnvironment Record (§9.2): the private names of a class body.
#[derive(Debug)]
pub struct PrivateEnv {
    pub outer: Option<Arc<PrivateEnv>>,
    pub names: Vec<PrivateName>,
}

impl PrivateEnv {
    /// ResolvePrivateIdentifier(privEnv, identifier) (§9.2.1.2)
    pub fn resolve(&self, description: JsString) -> Option<PrivateName> {
        match self.names.iter().find(|n| n.description == description) {
            Some(n) => Some(*n),
            None => self.outer.as_ref().and_then(|o| o.resolve(description)),
        }
    }
}

/// A Reference Record to a binding (§6.2.5); property references live on
/// the operand stack instead.
#[derive(Debug, Clone, Copy)]
pub enum Reference {
    Env(EnvId, JsString),
    /// A binding of a declarative record, found at position `slot` of its
    /// bindings (a resolution shortcut: GetValue and PutValue use the slot
    /// while it still holds `name`).
    Slot(EnvId, u32, JsString),
    Unresolvable(JsString),
    /// A frame local of the running code (`FunctionCode::locals`).
    Local(u32),
}

/// An exception handler installed by `PushHandler`.
#[derive(Debug, Clone)]
pub(crate) struct Handler {
    pc: usize,
    stack_len: usize,
    refs_len: usize,
    iters_len: usize,
    env: EnvId,
    private_env: Option<Arc<PrivateEnv>>,
}

/// A DisposeResources in progress at the end of a `using` scope, kept on the
/// operand stack (it survives the Awaits of `await using`).
#[derive(Debug)]
struct DisposeState {
    /// `None` only while a dispose op is running it.
    dispose: Option<DisposeResources>,
    /// The finally completion kind it was started with (see `Op::EndFinally`).
    kind: f64,
}

fn dispose_state(v: Value) -> Object {
    match v {
        Value::Object(o) => o,
        _ => unreachable!("a DisposeResources state on the stack"),
    }
}

fn take_dispose(agent: &mut Agent, state: Object) -> DisposeResources {
    agent
        .native_mut::<DisposeState>(state)
        .and_then(|s| s.dispose.take())
        .expect("a DisposeResources state")
}

fn put_dispose(agent: &mut Agent, state: Object, dispose: DisposeResources) {
    agent
        .native_mut::<DisposeState>(state)
        .expect("a DisposeResources state")
        .dispose = Some(dispose);
}

/// State of a for-in enumeration (EnumerateObjectProperties, §14.7.5.9).
#[derive(Debug)]
pub struct ForInIterator {
    object: Option<Object>,
    object_was_visited: bool,
    visited: crate::hash::FxHashSet<PropertyKey>,
    remaining: VecDeque<PropertyKey>,
}

impl ForInIterator {
    fn new(object: Option<Object>) -> ForInIterator {
        ForInIterator {
            object,
            object_was_visited: false,
            visited: Default::default(),
            remaining: VecDeque::new(),
        }
    }

    /// %ForInIteratorPrototype%.next (§14.7.5.10.2.1)
    fn next(&mut self, agent: &mut Agent) -> JsResult<Option<PropertyKey>> {
        loop {
            let Some(object) = self.object else {
                return Ok(None);
            };
            if !self.object_was_visited {
                let keys = object.internal_own_property_keys(agent)?;
                self.remaining = keys.into_iter().filter(|k| !k.is_symbol()).collect();
                self.object_was_visited = true;
            }
            while let Some(r) = self.remaining.pop_front() {
                if self.visited.contains(&r) {
                    continue;
                }
                if let Some(desc) = object.internal_get_own_property(agent, r)? {
                    self.visited.insert(r);
                    if desc.enumerable == Some(true) {
                        return Ok(Some(r));
                    }
                }
            }
            self.object = object.internal_get_prototype_of(agent)?;
            self.object_was_visited = false;
        }
    }
}

#[derive(Debug)]
pub enum FrameIter {
    Iter(IteratorRecord),
    ForIn(ForInIterator),
}

/// FunctionDeclarationInstantiation state carried from the call setup to
/// `InstantiateFunctionBody`.
#[derive(Debug, Clone, Default)]
pub struct PendingBody {
    pub env_record: Option<EnvId>,
    /// parameterBindings is the parameter names plus "arguments" when
    /// this is set (step 22.f).
    pub arguments_bound: bool,
}

/// A running (or suspended) function body, script or eval.
#[derive(Debug)]
pub struct Frame {
    pub code: Arc<FunctionCode>,
    pub pc: usize,
    pub stack: Vec<Value>,
    pub refs: Vec<Reference>,
    pub iters: Vec<FrameIter>,
    handlers: Vec<Handler>,
    /// LexicalEnvironment
    pub env: EnvId,
    /// VariableEnvironment
    pub var_env: EnvId,
    /// PrivateEnvironment
    pub private_env: Option<Arc<PrivateEnv>>,
    /// The function object, for function code.
    pub function: Option<Object>,
    pub args: Vec<Value>,
    /// The completion value of script and eval code.
    pub completion: Value,
    pub pending: PendingBody,
    /// Where a return resumption of the current `yield` continues.
    pub yield_return_pc: usize,
    /// Suspended inside `yield*`: resumptions go to the inner iterator.
    pub delegating: bool,
    /// How a `yield*` was resumed: 0 normal, 1 throw, 2 return.
    pub delegate_kind: u8,
    /// Set by an instruction that must run again on resumption.
    pub resume_pc: Option<usize>,
    /// Annex B.3.2: the block-level function names that declaration
    /// instantiation gave a var binding (from `code.decls`, which keeps the
    /// strings alive).
    pub annex_b: Vec<JsString>,
    /// The values of the code's frame locals (`FunctionCode::locals`),
    /// `None` for one not initialized yet.
    pub locals: Vec<Option<Value>>,
}

impl Frame {
    pub fn new(
        code: Arc<FunctionCode>,
        env: EnvId,
        private_env: Option<Arc<PrivateEnv>>,
        function: Option<Object>,
    ) -> Frame {
        Frame {
            code,
            pc: 0,
            stack: Vec::new(),
            refs: Vec::new(),
            iters: Vec::new(),
            handlers: Vec::new(),
            env,
            var_env: env,
            private_env,
            function,
            args: Vec::new(),
            completion: Value::Undefined,
            pending: PendingBody::default(),
            yield_return_pc: 0,
            delegating: false,
            delegate_kind: 0,
            resume_pc: None,
            annex_b: Vec::new(),
            locals: Vec::new(),
        }
    }

    #[inline]
    fn pop(&mut self) -> Value {
        self.stack.pop().expect("operand stack underflow")
    }

    fn push(&mut self, v: Value) {
        self.stack.push(v);
    }

    fn peek(&self) -> Value {
        *self.stack.last().expect("operand stack underflow")
    }

    fn pop_n(&mut self, n: usize) -> Vec<Value> {
        let at = self.stack.len() - n;
        self.stack.split_off(at)
    }

    fn top_iter(&mut self) -> &mut FrameIter {
        self.iters.last_mut().expect("iterator stack underflow")
    }

    /// Unwinds to the innermost handler for a thrown value; `false` if the
    /// frame has none (the exception leaves the frame).
    pub(crate) fn handle_throw(&mut self, value: Value) -> bool {
        let Some(h) = self.handlers.pop() else {
            return false;
        };
        self.stack.truncate(h.stack_len);
        self.refs.truncate(h.refs_len);
        self.iters.truncate(h.iters_len);
        self.env = h.env;
        self.private_env = h.private_env;
        self.stack.push(value);
        self.pc = h.pc;
        true
    }

    fn strict(&self) -> bool {
        self.code.strict || {
            let pc = self.pc as u32;
            self.code
                .strict_ranges
                .iter()
                .any(|&(s, e)| s <= pc && pc < e)
        }
    }
}

/// How a frame stopped running.
#[derive(Debug)]
pub enum FrameResult {
    Return(Value),
    /// A generator yielded (or reached GeneratorStart, with undefined).
    Yield(Value),
    /// `yield*` yielded the inner iterator's result object as it is.
    YieldDelegate(Value),
    Await(Value),
}

/// What an instruction asks the run loop to do next.
enum Flow {
    Next,
    Jump(usize),
    Done(FrameResult),
}

/// Runs a frame until it returns, yields, or throws an exception it does
/// not handle.
pub fn run_frame(agent: &mut Agent, frame: &mut Frame) -> JsResult<FrameResult> {
    // The frame is a root while it runs (docs/GC.md).
    agent.gc.frames.push(crate::gc::FramePtr(&*frame));
    let r = run_frame_loop(agent, frame);
    agent.gc.frames.pop();
    r
}

/// A safe point in `frame` (function entry, loop back-edges): collects if
/// the heap has grown and no Rust code above holds unrooted handles.
#[inline]
fn safe_point(agent: &mut Agent, frame: &Frame) {
    if agent.gc_wanted() {
        if let Some(top) = agent.gc.frames.last_mut() {
            *top = crate::gc::FramePtr(frame);
        }
        agent.collect_garbage();
    }
}

fn run_frame_loop(agent: &mut Agent, frame: &mut Frame) -> JsResult<FrameResult> {
    safe_point(agent, frame);
    // (The frame's code never changes while it runs; a handle of our own
    // keeps the instructions in a register instead of behind the frame.)
    let code = frame.code.clone();
    let ops = &code.ops[..];
    loop {
        let pc = frame.pc;
        let op = ops[pc];
        if fast_step(agent, frame, op, pc) {
            continue;
        }
        // Calls run outside `execute`, whose large native frame would
        // otherwise stay on the stack for the whole callee (React Native's
        // JavaScript thread has 1 MiB of stack).
        let flow = match op {
            Op::Call(argc) => call_op(agent, frame, argc).map(|()| Flow::Next),
            Op::CallSpread => call_spread_op(agent, frame).map(|()| Flow::Next),
            Op::New(argc) => new_op(agent, frame, argc).map(|()| Flow::Next),
            // (Here rather than in `execute`, which costs more to enter.)
            Op::Return => {
                let v = frame.pop();
                let v = match frame.code.kind {
                    CodeKind::Script | CodeKind::Eval => frame.completion,
                    _ => v,
                };
                Ok(Flow::Done(FrameResult::Return(v)))
            }
            _ => execute(agent, frame, op),
        };
        match flow {
            Ok(Flow::Next) => frame.pc = pc + 1,
            Ok(Flow::Jump(target)) => {
                frame.pc = target;
                if target <= pc {
                    safe_point(agent, frame);
                }
            }
            Ok(Flow::Done(result)) => {
                frame.pc = frame.resume_pc.take().unwrap_or(pc + 1);
                return Ok(result);
            }
            Err(JsError::Throw(v)) => {
                if !frame.handle_throw(v) {
                    return Err(JsError::Throw(v));
                }
            }
            Err(e) => return Err(e),
        }
    }
}

/// The commonest instructions in their common cases, without the call into
/// `execute`: the same steps as there, for operands that cannot run user
/// code or throw. Returns false (having done nothing) otherwise.
#[inline(always)]
fn fast_step(agent: &mut Agent, frame: &mut Frame, op: Op, pc: usize) -> bool {
    match op {
        Op::GetName(name, ic) => {
            let Some((env, slot)) = cached_binding(agent, &frame.code, frame.env, name, ic) else {
                return false;
            };
            let Some(v) = env.initialized_value_at(agent, slot) else {
                return false;
            };
            frame.stack.push(v);
        }
        Op::ResolveName(name, ic) => {
            let Some((env, slot)) = cached_binding(agent, &frame.code, frame.env, name, ic) else {
                return false;
            };
            frame.refs.push(Reference::Slot(env, slot as u32, name));
        }
        Op::GetLocal(slot) => {
            let Some(v) = frame.locals[slot as usize] else {
                return false;
            };
            frame.stack.push(v);
        }
        Op::ResolveLocal(slot) => frame.refs.push(Reference::Local(slot)),
        Op::InitLocal(slot) => {
            let v = frame.pop();
            frame.locals[slot as usize] = Some(v);
        }
        Op::GetRef if matches!(frame.refs.last(), Some(Reference::Local(_))) => {
            let Some(&Reference::Local(slot)) = frame.refs.last() else {
                unreachable!()
            };
            let Some(v) = frame.locals[slot as usize] else {
                return false;
            };
            frame.stack.push(v);
        }
        Op::GetRef => {
            let Some(&Reference::Slot(env, slot, name)) = frame.refs.last() else {
                return false;
            };
            if env.binding_name_at(agent, slot as usize) != Some(name) {
                return false;
            }
            let Some(v) = env.initialized_value_at(agent, slot as usize) else {
                return false;
            };
            frame.stack.push(v);
        }
        Op::PutRef if matches!(frame.refs.last(), Some(Reference::Local(_))) => {
            let Some(&Reference::Local(slot)) = frame.refs.last() else {
                unreachable!()
            };
            let locals = frame.code.locals.as_ref().expect("code with locals");
            if frame.locals[slot as usize].is_none()
                || locals[slot as usize].kind == LocalKind::Const
            {
                return false;
            }
            frame.locals[slot as usize] = Some(frame.peek());
            frame.refs.pop();
        }
        Op::PutRef => {
            let Some(&Reference::Slot(env, slot, name)) = frame.refs.last() else {
                return false;
            };
            let v = frame.peek();
            if !env.set_initialized_mutable_at(agent, slot as usize, name, v) {
                return false;
            }
            frame.refs.pop();
        }
        Op::ToNumeric => {
            if !matches!(frame.stack.last(), Some(Value::Number(_))) {
                return false;
            }
        }
        Op::Inc | Op::Dec => {
            let Some(Value::Number(n)) = frame.stack.last_mut() else {
                return false;
            };
            *n = if op == Op::Inc { *n + 1.0 } else { *n - 1.0 };
        }
        Op::Pop => {
            frame.stack.pop();
        }
        Op::Dup => {
            let v = frame.peek();
            frame.stack.push(v);
        }
        Op::GetProp(key) => {
            let Some(&Value::Object(o)) = frame.stack.last() else {
                return false;
            };
            let Some(v) = crate::object::internal::ordinary_get_data(agent, o, key) else {
                return false;
            };
            *frame.stack.last_mut().expect("checked") = v;
        }
        Op::GetPropForCall(key) => {
            let Some(&Value::Object(o)) = frame.stack.last() else {
                return false;
            };
            let Some(f) = crate::object::internal::ordinary_get_data(agent, o, key) else {
                return false;
            };
            *frame.stack.last_mut().expect("checked") = f;
            frame.stack.push(Value::Object(o));
        }
        // CreateDataPropertyOrThrow on an object literal's ordinary object
        // that lacks the key: ValidateAndApplyPropertyDescriptor creates
        // the data property.
        Op::DefineField(key) => {
            let n = frame.stack.len();
            let (Some(&Value::Object(o)), Some(&v)) =
                (frame.stack.get(n.wrapping_sub(2)), frame.stack.last())
            else {
                return false;
            };
            let obj = agent.obj_mut(o);
            if !matches!(obj.kind, ObjectKind::Ordinary)
                || !obj.extensible
                || obj.properties.get(&key).is_some()
            {
                return false;
            }
            obj.properties.insert(
                key,
                Property::Data {
                    value: v,
                    writable: true,
                    enumerable: true,
                    configurable: true,
                },
            );
            frame.stack.pop();
        }
        // PutValue to an ordinary object's own writable data property:
        // OrdinarySet's [[DefineOwnProperty]] of { [[Value]]: V } on the
        // object itself only changes the value.
        Op::SetProp(key) => {
            let n = frame.stack.len();
            let (Some(&Value::Object(o)), Some(&v)) =
                (frame.stack.get(n.wrapping_sub(2)), frame.stack.last())
            else {
                return false;
            };
            let obj = agent.obj_mut(o);
            if !matches!(obj.kind, ObjectKind::Ordinary) {
                return false;
            }
            let Some(Property::Data {
                value,
                writable: true,
                ..
            }) = obj.properties.get_mut(&key)
            else {
                return false;
            };
            *value = v;
            frame.stack.truncate(n - 2);
            frame.stack.push(v);
        }
        // (ToPropertyKey of a primitive runs no user code.)
        Op::GetElem => {
            let n = frame.stack.len();
            let (Some(&Value::Object(o)), Some(&key)) =
                (frame.stack.get(n.wrapping_sub(2)), frame.stack.last())
            else {
                return false;
            };
            if !matches!(key, Value::Number(_) | Value::String(_) | Value::Symbol(_)) {
                return false;
            }
            let Ok(key) = ops::to_property_key(agent, key) else {
                return false;
            };
            let Some(v) = crate::object::internal::ordinary_get_data(agent, o, key) else {
                return false;
            };
            frame.stack.truncate(n - 2);
            frame.stack.push(v);
        }
        Op::Undefined => frame.stack.push(Value::Undefined),
        Op::Null => frame.stack.push(Value::Null),
        Op::String(s) => frame.stack.push(Value::String(s)),
        Op::GetArg(i) => {
            let v = frame
                .args
                .get(i as usize)
                .copied()
                .unwrap_or(Value::Undefined);
            frame.stack.push(v);
        }
        Op::Not => {
            let v = frame.pop();
            let b = ops::to_boolean(agent, v);
            frame.stack.push(Value::Boolean(!b));
        }
        // (IsStrictlyEqual runs no user code and cannot throw.)
        Op::Binary(bop @ (BinOp::StrictEq | BinOp::StrictNotEq)) => {
            let b = frame.pop();
            let a = frame.pop();
            let eq = ops::is_strictly_equal(agent, a, b);
            frame
                .stack
                .push(Value::Boolean(eq == (bop == BinOp::StrictEq)));
        }
        Op::JumpIfTrue(l) => {
            let v = frame.pop();
            if ops::to_boolean(agent, v) {
                fast_jump(agent, frame, l, pc);
                return true;
            }
        }
        Op::JumpIfFalseKeep(l) => {
            if !ops::to_boolean(agent, frame.peek()) {
                fast_jump(agent, frame, l, pc);
                return true;
            }
        }
        Op::JumpIfTrueKeep(l) => {
            if ops::to_boolean(agent, frame.peek()) {
                fast_jump(agent, frame, l, pc);
                return true;
            }
        }
        // FunctionDeclarationInstantiation steps 27-36 for code with
        // locals (see `instantiate::function_body_instantiation`).
        Op::InstantiateFunctionBody if frame.code.locals.is_some() => {
            let Some(env_record) = frame.pending.env_record.take() else {
                return false;
            };
            frame.var_env = env_record;
            frame.env = env_record;
        }
        Op::Number(n) => frame.stack.push(Value::Number(n)),
        Op::Binary(bop) => {
            let n = frame.stack.len();
            let (Some(&Value::Number(a)), Some(&Value::Number(b))) = (
                frame.stack.get(n.wrapping_sub(2)),
                frame.stack.get(n.wrapping_sub(1)),
            ) else {
                return false;
            };
            let Some(v) = operators::number_binary(bop, a, b) else {
                return false;
            };
            frame.stack.truncate(n - 2);
            frame.stack.push(v);
        }
        Op::Jump(l) => {
            fast_jump(agent, frame, l, pc);
            return true;
        }
        Op::JumpIfFalse(l) => {
            let v = frame.pop();
            if !ops::to_boolean(agent, v) {
                fast_jump(agent, frame, l, pc);
                return true;
            }
        }
        _ => return false,
    }
    frame.pc = pc + 1;
    true
}

/// A jump from `fast_step`: a backward one is a safe point, as in the loop.
#[inline(always)]
fn fast_jump(agent: &mut Agent, frame: &mut Frame, l: u32, pc: usize) {
    frame.pc = l as usize;
    if frame.pc <= pc {
        safe_point(agent, frame);
    }
}

// ---- bindings ---------------------------------------------------------------------

/// ResolveBinding(name) from `env` (§9.4.2, GetIdentifierReference).
pub fn resolve_binding(agent: &mut Agent, env: EnvId, name: JsString) -> JsResult<Reference> {
    let mut e = Some(env);
    while let Some(id) = e {
        if id.has_binding(agent, name)? {
            return Ok(Reference::Env(id, name));
        }
        e = id.outer(agent);
    }
    Ok(Reference::Unresolvable(name))
}

/// ResolveBinding(name) for a name instruction with binding cache `ic` (see
/// `FunctionCode::name_caches`): a binding found before is looked for first
/// in the same place, without searching the environments in between. The
/// compiler only gives a cache where no environment in between can gain a
/// binding at run time, so the result is the one the search would find.
fn resolve_binding_cached(
    agent: &mut Agent,
    code: &FunctionCode,
    env: EnvId,
    name: JsString,
    ic: u32,
) -> JsResult<Reference> {
    use std::sync::atomic::Ordering::Relaxed;
    let Some(cache) = code.name_caches.get(ic as usize) else {
        return resolve_binding(agent, env, name);
    };
    if let Some((id, slot)) = cached_binding(agent, code, env, name, ic) {
        return Ok(Reference::Slot(id, slot as u32, name));
    }
    // A binding without a slot (the global object's) is cached as its
    // environment's distance with slot `ENV_BINDING`: the declarative
    // environments in between cannot gain the name, so only that one
    // environment needs asking again.
    let cached = cache.load(Relaxed);
    if cached != 0 && (cached - 1) as u32 == ENV_BINDING {
        let mut id = Some(env);
        for _ in 0..(cached - 1) >> 32 {
            id = id.and_then(|e| e.outer(agent));
        }
        if let Some(id) = id
            && id.has_binding(agent, name)?
        {
            return Ok(Reference::Env(id, name));
        }
    }
    let mut e = Some(env);
    let mut hops = 0u64;
    while let Some(id) = e {
        if id.has_binding(agent, name)? {
            if let Some(slot) = id.binding_slot(agent, name) {
                cache.store(((hops << 32) | slot as u64) + 1, Relaxed);
                return Ok(Reference::Slot(id, slot as u32, name));
            }
            cache.store(((hops << 32) | u64::from(ENV_BINDING)) + 1, Relaxed);
            return Ok(Reference::Env(id, name));
        }
        e = id.outer(agent);
        hops += 1;
    }
    Ok(Reference::Unresolvable(name))
}

/// The slot of a binding cache entry for a binding found in an object
/// environment record (see `resolve_binding_cached`).
const ENV_BINDING: u32 = u32::MAX;

/// The environment and slot where name instruction `ic` found `name` last
/// time, if it is still there (see `resolve_binding_cached`).
#[inline]
fn cached_binding(
    agent: &Agent,
    code: &FunctionCode,
    env: EnvId,
    name: JsString,
    ic: u32,
) -> Option<(EnvId, usize)> {
    let cached = code
        .name_caches
        .get(ic as usize)?
        .load(std::sync::atomic::Ordering::Relaxed);
    if cached == 0 {
        return None;
    }
    let (hops, slot) = ((cached - 1) >> 32, (cached - 1) as u32 as usize);
    let mut id = env;
    for _ in 0..hops {
        id = id.outer(agent)?;
    }
    (id.binding_name_at(agent, slot) == Some(name)).then_some((id, slot))
}

fn not_defined(agent: &mut Agent, name: JsString) -> JsError {
    let msg = format!("{} is not defined", agent.str_lossy(name));
    agent.throw_reference_error(&msg)
}

/// GetValue on a binding reference.
pub fn get_reference(agent: &mut Agent, r: Reference, strict: bool) -> JsResult<Value> {
    match r {
        Reference::Env(env, name) => env.get_binding_value(agent, name, strict),
        Reference::Slot(env, slot, name) => {
            if env.binding_name_at(agent, slot as usize) == Some(name) {
                env.binding_value_at(agent, slot as usize)
            } else {
                env.get_binding_value(agent, name, strict)
            }
        }
        Reference::Unresolvable(name) => Err(not_defined(agent, name)),
        Reference::Local(_) => unreachable!("a local reference outside its frame"),
    }
}

/// PutValue on a binding reference.
pub fn put_reference(agent: &mut Agent, r: Reference, value: Value, strict: bool) -> JsResult<()> {
    match r {
        Reference::Env(env, name) => env.set_mutable_binding(agent, name, value, strict),
        Reference::Slot(env, slot, name) => {
            env.set_mutable_binding_at(agent, slot as usize, name, value, strict)
        }
        Reference::Unresolvable(name) => {
            if strict {
                return Err(not_defined(agent, name));
            }
            let realm = agent.current_realm();
            let global = agent.realm(realm).global_object;
            let key = agent.key_from_string(name);
            ops::set(agent, global, key, value, false)
        }
        Reference::Local(_) => unreachable!("a local reference outside its frame"),
    }
}

/// GetValue of frame local `slot`: GetBindingValue of a declarative
/// record's binding (§9.1.1.1.6).
#[inline]
fn get_local(agent: &mut Agent, frame: &Frame, slot: u32) -> JsResult<Value> {
    match frame.locals[slot as usize] {
        Some(v) => Ok(v),
        // 2. If the binding for N in envRec is an uninitialized binding, throw a ReferenceError exception.
        None => Err(uninitialized_local(agent, frame, slot)),
    }
}

/// PutValue to frame local `slot`: SetMutableBinding of a declarative
/// record's binding (§9.1.1.1.5).
fn put_local(agent: &mut Agent, frame: &mut Frame, slot: u32, value: Value) -> JsResult<()> {
    let info = &frame.code.locals.as_ref().expect("code with locals")[slot as usize];
    // 4. If the binding for N in envRec has not yet been initialized, throw a ReferenceError exception.
    if frame.locals[slot as usize].is_none() {
        return Err(uninitialized_local(agent, frame, slot));
    }
    // 6. Else, (an immutable binding, created strict by const declarations)
    //    if S is true, throw a TypeError exception.
    if info.kind == LocalKind::Const {
        let msg = format!("assignment to constant '{}'", agent.str_lossy(info.name));
        return Err(agent.throw_type_error(&msg));
    }
    // 5. Else if the binding for N in envRec is a mutable binding, change its bound value to V.
    frame.locals[slot as usize] = Some(value);
    Ok(())
}

#[cold]
fn uninitialized_local(agent: &mut Agent, frame: &Frame, slot: u32) -> JsError {
    let name = frame.code.locals.as_ref().expect("code with locals")[slot as usize].name;
    let msg = format!(
        "cannot access '{}' before initialization",
        agent.str_lossy(name)
    );
    agent.throw_reference_error(&msg)
}

/// ResolveThisBinding (§9.4.4)
pub fn resolve_this_binding(agent: &mut Agent, env: EnvId) -> JsResult<Value> {
    let env = agent.get_this_environment(env);
    env.get_this_binding(agent)
}

// ---- property access ---------------------------------------------------------------

fn base_error(agent: &mut Agent, base: Value, key: PropertyKey, action: &str) -> JsError {
    let what = if base.is_null() { "null" } else { "undefined" };
    let msg = format!(
        "Cannot {action} properties of {what} (reading '{}')",
        agent.key_display(key)
    );
    agent.throw_type_error(&msg)
}

/// GetValue of a property reference with base `base`.
pub fn get_property(agent: &mut Agent, base: Value, key: PropertyKey) -> JsResult<Value> {
    if base.is_nullish() {
        return Err(base_error(agent, base, key, "read"));
    }
    ops::get_v(agent, base, key)
}

/// GetValue of a property reference whose name still needs ToPropertyKey.
fn get_property_value_key(
    agent: &mut Agent,
    base: Value,
    key: Value,
) -> JsResult<(PropertyKey, Value)> {
    // 3.a Let baseObj be ? ToObject(V.[[Base]]).
    if base.is_nullish() {
        let k = match key {
            Value::String(s) => agent.key_from_string(s),
            _ => PropertyKey::String(agent.names.empty),
        };
        return Err(base_error(agent, base, k, "read"));
    }
    // 3.c Set the name to ? ToPropertyKey(name).
    let k = ops::to_property_key(agent, key)?;
    let v = ops::get_v(agent, base, k)?;
    Ok((k, v))
}

/// PutValue of a property reference (§6.2.5.6 step 3).
pub fn put_property(
    agent: &mut Agent,
    base: Value,
    key: PropertyKey,
    value: Value,
    strict: bool,
) -> JsResult<()> {
    if base.is_nullish() {
        let what = if base.is_null() { "null" } else { "undefined" };
        let msg = format!(
            "Cannot set properties of {what} (setting '{}')",
            agent.key_display(key)
        );
        return Err(agent.throw_type_error(&msg));
    }
    let obj = ops::to_object(agent, base)?;
    let ok = obj.internal_set(agent, key, value, base)?;
    if !ok && strict {
        let msg = format!(
            "Cannot assign to read only property '{}'",
            agent.key_display(key)
        );
        return Err(agent.throw_type_error(&msg));
    }
    Ok(())
}

fn put_property_value_key(
    agent: &mut Agent,
    base: Value,
    key: Value,
    value: Value,
    strict: bool,
) -> JsResult<()> {
    if base.is_nullish() {
        let what = if base.is_null() { "null" } else { "undefined" };
        let msg = format!("Cannot set properties of {what}");
        return Err(agent.throw_type_error(&msg));
    }
    let obj = ops::to_object(agent, base)?;
    let k = ops::to_property_key(agent, key)?;
    let ok = obj.internal_set(agent, k, value, base)?;
    if !ok && strict {
        let msg = format!(
            "Cannot assign to read only property '{}'",
            agent.key_display(k)
        );
        return Err(agent.throw_type_error(&msg));
    }
    Ok(())
}

fn delete_property(agent: &mut Agent, base: Value, key: Value, strict: bool) -> JsResult<bool> {
    // delete: baseObj = ? ToObject(base), then the name is converted.
    let obj = ops::to_object(agent, base)?;
    let k = ops::to_property_key(agent, key)?;
    let ok = obj.internal_delete(agent, k)?;
    if !ok && strict {
        let msg = format!("Cannot delete property '{}'", agent.key_display(k));
        return Err(agent.throw_type_error(&msg));
    }
    Ok(ok)
}

/// GetSuperBase of the this environment: the [[HomeObject]]'s prototype.
fn super_base(agent: &mut Agent, env: EnvId) -> JsResult<(Value, Value)> {
    let this_env = agent.get_this_environment(env);
    // MakeSuperPropertyReference: actualThis = ? env.GetThisBinding().
    let this = this_env.get_this_binding(agent)?;
    let base = get_super_base(agent, env)?;
    Ok((base, this))
}

/// GetSuperBase(env) for the function environment of `env`'s this binding.
fn get_super_base(agent: &mut Agent, env: EnvId) -> JsResult<Value> {
    let this_env = agent.get_this_environment(env);
    // 1. Let home be env.[[FunctionObject]].[[HomeObject]].
    let home = this_env
        .function_data(agent)
        .map(|f| f.function_object)
        .and_then(|f| match &agent.obj(f).kind {
            ObjectKind::Function(e) => e.home_object,
            _ => None,
        });
    Ok(match home {
        // 2. If home is undefined, return undefined.
        None => Value::Undefined,
        // 3-4. Return ? home.[[GetPrototypeOf]]().
        Some(h) => h
            .internal_get_prototype_of(agent)?
            .map(Value::Object)
            .unwrap_or(Value::Null),
    })
}

/// The first steps of GetValue / PutValue on a Super Reference
/// { [[Base]]: base, [[ReferencedName]]: key }: ? ToObject(base), then
/// ? ToPropertyKey(key).
fn super_reference(agent: &mut Agent, base: Value, key: Value) -> JsResult<(Object, PropertyKey)> {
    if base.is_nullish() {
        let what = if base.is_null() { "null" } else { "undefined" };
        return Err(agent.throw_type_error(&format!("Cannot use 'super' properties of {what}")));
    }
    let obj = ops::to_object(agent, base)?;
    let key = ops::to_property_key(agent, key)?;
    Ok((obj, key))
}

fn super_get(agent: &mut Agent, env: EnvId, key: PropertyKey) -> JsResult<Value> {
    let (base, this) = super_base(agent, env)?;
    if base.is_nullish() {
        return Err(base_error(agent, base, key, "read"));
    }
    let obj = ops::to_object(agent, base)?;
    obj.internal_get(agent, key, this)
}

fn resolve_private(
    agent: &mut Agent,
    frame: &Frame,
    description: JsString,
) -> JsResult<PrivateName> {
    match frame
        .private_env
        .as_ref()
        .and_then(|p| p.resolve(description))
    {
        Some(n) => Ok(n),
        None => {
            let msg = format!("undeclared private name {}", agent.str_lossy(description));
            Err(agent.throw_syntax_error(&msg))
        }
    }
}

/// The elements of an array built by the compiler (argument lists).
fn array_elements(agent: &mut Agent, array: Value) -> JsResult<Vec<Value>> {
    let Value::Object(a) = array else {
        return Ok(Vec::new());
    };
    let len = crate::object::array::array_length(agent, a);
    let mut out = Vec::with_capacity(len as usize);
    for i in 0..len {
        out.push(ops::get(agent, a, PropertyKey::Index(i))?);
    }
    Ok(out)
}

fn array_append(agent: &mut Agent, array: Value, value: Value) -> JsResult<()> {
    let Value::Object(a) = array else {
        return Ok(());
    };
    let len = crate::object::array::array_length(agent, a);
    let key = agent.key_from_index(len as f64);
    ops::create_data_property_or_throw(agent, a, key, value)
}

fn callable_error(agent: &mut Agent, what: &str) -> JsError {
    let msg = format!("{what} is not a function");
    agent.throw_type_error(&msg)
}

// ---- the instructions ----------------------------------------------------------------

fn execute(agent: &mut Agent, frame: &mut Frame, op: Op) -> JsResult<Flow> {
    match op {
        Op::Undefined => frame.push(Value::Undefined),
        Op::Null => frame.push(Value::Null),
        Op::True => frame.push(Value::Boolean(true)),
        Op::False => frame.push(Value::Boolean(false)),
        Op::Number(n) => frame.push(Value::Number(n)),
        Op::String(s) => frame.push(Value::String(s)),
        Op::BigInt(b) => frame.push(Value::BigInt(b)),

        Op::Pop => {
            frame.pop();
        }
        Op::Dup => {
            let v = frame.peek();
            frame.push(v);
        }
        Op::Dup2 => {
            let n = frame.stack.len();
            let (a, b) = (frame.stack[n - 2], frame.stack[n - 1]);
            frame.push(a);
            frame.push(b);
        }
        Op::Swap => {
            let n = frame.stack.len();
            frame.stack.swap(n - 1, n - 2);
        }
        Op::RotRight3 => {
            let c = frame.pop();
            let n = frame.stack.len();
            frame.stack.insert(n - 2, c);
        }
        Op::RotRight4 => {
            let d = frame.pop();
            let n = frame.stack.len();
            frame.stack.insert(n - 3, d);
        }
        Op::RotRight5 => {
            let e = frame.pop();
            let n = frame.stack.len();
            frame.stack.insert(n - 4, e);
        }
        Op::Pick(n) => {
            let v = frame.stack[frame.stack.len() - 1 - n as usize];
            frame.push(v);
        }
        Op::SetCompletion => frame.completion = frame.pop(),
        Op::GetCompletion => frame.push(frame.completion),

        // ---- identifiers
        Op::GetName(name, ic) => {
            let v = match cached_binding(agent, &frame.code, frame.env, name, ic) {
                Some((env, slot)) => env.binding_value_at(agent, slot)?,
                None => {
                    let r = resolve_binding_cached(agent, &frame.code, frame.env, name, ic)?;
                    get_reference(agent, r, frame.strict())?
                }
            };
            frame.push(v);
        }
        Op::TypeofName(name, ic) => {
            let r = resolve_binding_cached(agent, &frame.code, frame.env, name, ic)?;
            let v = match r {
                Reference::Unresolvable(_) => Value::Undefined,
                r => get_reference(agent, r, frame.strict())?,
            };
            let t = type_of(agent, v);
            frame.push(Value::String(t));
        }
        Op::GetNameForCall(name, ic) => {
            let r = resolve_binding_cached(agent, &frame.code, frame.env, name, ic)?;
            let v = get_reference(agent, r, frame.strict())?;
            let this = match r {
                Reference::Env(env, _) => env.with_base_object(agent),
                // A declarative record's WithBaseObject() is undefined.
                Reference::Slot(..) | Reference::Unresolvable(_) | Reference::Local(_) => {
                    Value::Undefined
                }
            };
            frame.push(v);
            frame.push(this);
        }
        Op::ResolveName(name, ic) => {
            let r = resolve_binding_cached(agent, &frame.code, frame.env, name, ic)?;
            frame.refs.push(r);
        }
        Op::GetRef => {
            let r = *frame.refs.last().expect("reference stack underflow");
            let v = match r {
                Reference::Local(slot) => get_local(agent, frame, slot)?,
                r => get_reference(agent, r, frame.strict())?,
            };
            frame.push(v);
        }
        Op::PutRef => {
            let r = frame.refs.pop().expect("reference stack underflow");
            let v = frame.peek();
            match r {
                Reference::Local(slot) => put_local(agent, frame, slot, v)?,
                r => put_reference(agent, r, v, frame.strict())?,
            }
        }
        Op::GetLocal(slot) => {
            let v = get_local(agent, frame, slot)?;
            frame.push(v);
        }
        Op::ResolveLocal(slot) => frame.refs.push(Reference::Local(slot)),
        Op::InitLocal(slot) => {
            let v = frame.pop();
            frame.locals[slot as usize] = Some(v);
        }
        Op::ClearLocals(start, count) => {
            frame.locals[start as usize..(start + count) as usize].fill(None);
        }
        Op::DropRef => {
            frame.refs.pop();
        }
        Op::InitName(name) => {
            let v = frame.pop();
            frame.env.initialize_binding(agent, name, v)?;
        }
        Op::AnnexBFunction(name) => {
            if frame.annex_b.contains(&name) {
                // i. Let fenv be the running execution context's VariableEnvironment.
                // ii. Let benv be the running execution context's LexicalEnvironment.
                // iii. Let fobj be ! benv.GetBindingValue(F, false).
                let fobj = frame.env.get_binding_value(agent, name, false)?;
                // iv. Perform ? fenv.SetMutableBinding(F, fobj, false).
                frame
                    .var_env
                    .set_mutable_binding(agent, name, fobj, false)?;
            }
        }
        Op::DeleteName(name) => {
            let r = resolve_binding(agent, frame.env, name)?;
            let result = match r {
                Reference::Unresolvable(_) => true,
                Reference::Env(env, name) | Reference::Slot(env, _, name) => {
                    env.delete_binding(agent, name)?
                }
                Reference::Local(_) => unreachable!("resolve_binding finds no locals"),
            };
            frame.push(Value::Boolean(result));
        }

        // ---- environments
        Op::PushScope(i) => {
            let code = frame.code.clone();
            let env = if code.scopes[i as usize].is_catch {
                Environment::new_catch(agent, Some(frame.env))
            } else {
                Environment::new_declarative(agent, Some(frame.env))
            };
            instantiate::block_declaration_instantiation(
                agent,
                frame,
                &code,
                &code.scopes[i as usize],
                env,
            )?;
            frame.env = env;
        }
        Op::PushEmptyScope => frame.env = Environment::new_declarative(agent, Some(frame.env)),
        Op::PopScope => {
            let scope = frame.env;
            frame.env = scope
                .outer(agent)
                .expect("PopScope at the outermost environment");
            if frame.code.envs_die_with_call {
                agent.free_env(scope);
            }
        }
        Op::CopyScope => {
            // CreatePerIterationEnvironment. Without closures nothing can
            // tell the copy from the last iteration's environment, so a
            // function whose environments die with its call keeps that one.
            if !frame.code.envs_die_with_call {
                let copy = agent.env(frame.env).clone();
                frame.env = agent.alloc_env(copy);
            }
        }
        Op::PushWith => {
            let v = frame.pop();
            let obj = ops::to_object(agent, v)?;
            frame.env = Environment::new_object(agent, obj, true, Some(frame.env));
        }

        // ---- properties
        Op::GetProp(key) => {
            let base = frame.pop();
            let v = get_property(agent, base, key)?;
            frame.push(v);
        }
        Op::GetElem => {
            let key = frame.pop();
            let base = frame.pop();
            let (_, v) = get_property_value_key(agent, base, key)?;
            frame.push(v);
        }
        Op::GetElemForUpdate => {
            let key = frame.pop();
            let base = frame.peek();
            let (k, v) = get_property_value_key(agent, base, key)?;
            let kv = agent.key_to_value(k);
            frame.push(kv);
            frame.push(v);
        }
        Op::GetPropForCall(key) => {
            let base = frame.pop();
            let f = get_property(agent, base, key)?;
            frame.push(f);
            frame.push(base);
        }
        Op::GetElemForCall => {
            let key = frame.pop();
            let base = frame.pop();
            let (_, f) = get_property_value_key(agent, base, key)?;
            frame.push(f);
            frame.push(base);
        }
        Op::SetProp(key) => {
            let v = frame.pop();
            let base = frame.pop();
            put_property(agent, base, key, v, frame.strict())?;
            frame.push(v);
        }
        Op::SetElem => {
            let v = frame.pop();
            let key = frame.pop();
            let base = frame.pop();
            put_property_value_key(agent, base, key, v, frame.strict())?;
            frame.push(v);
        }
        Op::DeleteProp(key) => {
            let base = frame.pop();
            let kv = agent.key_to_value(key);
            let r = delete_property(agent, base, kv, frame.strict())?;
            frame.push(Value::Boolean(r));
        }
        Op::DeleteElem => {
            let key = frame.pop();
            let base = frame.pop();
            let r = delete_property(agent, base, key, frame.strict())?;
            frame.push(Value::Boolean(r));
        }
        Op::ToPropertyKey => {
            let v = frame.pop();
            let k = ops::to_property_key(agent, v)?;
            let kv = agent.key_to_value(k);
            frame.push(kv);
        }
        Op::RequireObjectCoercible => {
            let v = frame.peek();
            ops::require_object_coercible(agent, v)?;
        }

        // ---- super and private references
        Op::GetSuperProp(key) => {
            let v = super_get(agent, frame.env, key)?;
            frame.push(v);
        }
        Op::GetSuperPropForCall(key) => {
            let f = super_get(agent, frame.env, key)?;
            let this = resolve_this_binding(agent, frame.env)?;
            frame.push(f);
            frame.push(this);
        }
        Op::SuperBase => {
            // MakeSuperPropertyReference: Let baseValue be GetSuperBase(env).
            let base = get_super_base(agent, frame.env)?;
            frame.push(base);
        }
        Op::GetSuperElem => {
            let base = frame.pop();
            let key = frame.pop();
            let this = frame.pop();
            let (obj, key) = super_reference(agent, base, key)?;
            // Return ? baseObj.[[Get]](V.[[ReferencedName]], GetThisValue(V)).
            let v = obj.internal_get(agent, key, this)?;
            frame.push(v);
        }
        Op::GetSuperElemForCall => {
            let base = frame.pop();
            let key = frame.pop();
            let this = frame.pop();
            let (obj, key) = super_reference(agent, base, key)?;
            let f = obj.internal_get(agent, key, this)?;
            frame.push(f);
            frame.push(this);
        }
        Op::GetSuperElemForUpdate => {
            let base = frame.pop();
            let key = frame.pop();
            let this = frame.pop();
            let (obj, key) = super_reference(agent, base, key)?;
            let v = obj.internal_get(agent, key, this)?;
            frame.push(this);
            frame.push(agent.key_to_value(key));
            frame.push(Value::Object(obj));
            frame.push(v);
        }
        Op::SetSuperElem => {
            let v = frame.pop();
            let base = frame.pop();
            let key = frame.pop();
            let this = frame.pop();
            let (obj, key) = super_reference(agent, base, key)?;
            // PutValue: Let succeeded be ? baseObj.[[Set]](V.[[ReferencedName]], W, GetThisValue(V)).
            let ok = obj.internal_set(agent, key, v, this)?;
            // If succeeded is false and V.[[Strict]] is true, throw a TypeError exception.
            if !ok && frame.strict() {
                let msg = format!(
                    "Cannot assign to read only property '{}'",
                    agent.key_display(key)
                );
                return Err(agent.throw_type_error(&msg));
            }
            frame.push(v);
        }
        Op::GetPrivate(name) => {
            let base = frame.pop();
            let n = resolve_private(agent, frame, name)?;
            let obj = ops::to_object(agent, base)?;
            let v = private_get(agent, obj, n)?;
            frame.push(v);
        }
        Op::SetPrivate(name) => {
            let v = frame.pop();
            let base = frame.pop();
            let n = resolve_private(agent, frame, name)?;
            let obj = ops::to_object(agent, base)?;
            private_set(agent, obj, n, v)?;
            frame.push(v);
        }
        Op::GetPrivateForCall(name) => {
            let base = frame.pop();
            let n = resolve_private(agent, frame, name)?;
            let obj = ops::to_object(agent, base)?;
            let f = private_get(agent, obj, n)?;
            frame.push(f);
            frame.push(base);
        }
        Op::PrivateIn(name) => {
            let v = frame.pop();
            let Value::Object(o) = v else {
                return Err(agent.throw_type_error("right-hand side of 'in' must be an object"));
            };
            let n = resolve_private(agent, frame, name)?;
            let found = agent.obj(o).private_elements.iter().any(|e| e.key == n);
            frame.push(Value::Boolean(found));
        }

        // ---- operators
        Op::Binary(bop) => {
            let r = frame.pop();
            let l = frame.pop();
            let v = apply_binary_operator(agent, bop, l, r)?;
            frame.push(v);
        }
        Op::Neg => {
            let v = frame.pop();
            let r = operators::unary_minus(agent, v)?;
            frame.push(r);
        }
        Op::Plus => {
            let v = frame.pop();
            let n = ops::to_number(agent, v)?;
            frame.push(Value::Number(n));
        }
        Op::BitNot => {
            let v = frame.pop();
            let r = operators::bitwise_not(agent, v)?;
            frame.push(r);
        }
        Op::Not => {
            let v = frame.pop();
            let b = ops::to_boolean(agent, v);
            frame.push(Value::Boolean(!b));
        }
        Op::Typeof => {
            let v = frame.pop();
            let t = type_of(agent, v);
            frame.push(Value::String(t));
        }
        Op::ToNumeric => {
            let v = frame.pop();
            let n = match ops::to_numeric(agent, v)? {
                ops::Numeric::Number(n) => Value::Number(n),
                ops::Numeric::BigInt(b) => Value::BigInt(b),
            };
            frame.push(n);
        }
        Op::Inc | Op::Dec => {
            let v = frame.pop();
            let r = match v {
                Value::Number(n) => Value::Number(if op == Op::Inc { n + 1.0 } else { n - 1.0 }),
                // BigInt::add(oldValue, 1ℤ) / BigInt::subtract(oldValue, 1ℤ)
                Value::BigInt(b) => {
                    let one = ops::bigint::BigIntValue::from_i128(1);
                    let old = agent.bigint(b);
                    let value = if op == Op::Inc {
                        old.add(&one)
                    } else {
                        old.sub(&one)
                    };
                    Value::BigInt(agent.new_bigint(value))
                }
                _ => unreachable!("Inc/Dec follow ToNumeric"),
            };
            frame.push(r);
        }
        Op::ToString => {
            let v = frame.pop();
            let s = ops::to_string(agent, v)?;
            frame.push(Value::String(s));
        }

        // ---- control flow
        Op::Jump(l) => return Ok(Flow::Jump(l as usize)),
        Op::JumpIfFalse(l) => {
            let v = frame.pop();
            if !ops::to_boolean(agent, v) {
                return Ok(Flow::Jump(l as usize));
            }
        }
        Op::JumpIfTrue(l) => {
            let v = frame.pop();
            if ops::to_boolean(agent, v) {
                return Ok(Flow::Jump(l as usize));
            }
        }
        Op::JumpIfFalseKeep(l) => {
            if !ops::to_boolean(agent, frame.peek()) {
                return Ok(Flow::Jump(l as usize));
            }
        }
        Op::JumpIfTrueKeep(l) => {
            if ops::to_boolean(agent, frame.peek()) {
                return Ok(Flow::Jump(l as usize));
            }
        }
        Op::JumpIfNullishKeep(l) => {
            if frame.peek().is_nullish() {
                return Ok(Flow::Jump(l as usize));
            }
        }
        Op::JumpIfNotNullishKeep(l) => {
            if !frame.peek().is_nullish() {
                return Ok(Flow::Jump(l as usize));
            }
        }
        Op::JumpIfNotUndefined(l) => {
            if !frame.peek().is_undefined() {
                return Ok(Flow::Jump(l as usize));
            }
        }

        // ---- exceptions
        Op::Throw => {
            let v = frame.pop();
            return Err(JsError::Throw(v));
        }
        Op::ThrowError(kind, m) => {
            let msg = frame.code.messages[m as usize].clone();
            return Err(match kind {
                ErrorType::Type => agent.throw_type_error(&msg),
                ErrorType::Reference => agent.throw_reference_error(&msg),
                ErrorType::Syntax => agent.throw_syntax_error(&msg),
            });
        }
        Op::PushHandler(l) => frame.handlers.push(Handler {
            pc: l as usize,
            stack_len: frame.stack.len(),
            refs_len: frame.refs.len(),
            iters_len: frame.iters.len(),
            env: frame.env,
            private_env: frame.private_env.clone(),
        }),
        Op::PopHandler => {
            frame.handlers.pop();
        }
        Op::EndFinally(index) => {
            let kind = frame.pop();
            let value = frame.pop();
            let kind = kind.as_number().unwrap_or(0.0) as usize;
            match kind {
                0 => {}
                1 => return Err(JsError::Throw(value)),
                k => {
                    frame.push(value);
                    let target = frame.code.finally_targets[index as usize][k - 2];
                    return Ok(Flow::Jump(target as usize));
                }
            }
        }

        // ---- explicit resource management
        Op::AddDisposable(is_async) => {
            let v = frame.pop();
            let kind = if is_async {
                DisposeKind::Async
            } else {
                DisposeKind::Sync
            };
            // ? AddDisposableResource(envRec.[[DisposableResourceStack]], V, hint)
            let mut capability = std::mem::take(frame.env.disposable_resource_stack_mut(agent));
            let result =
                ops::dispose::add_disposable_resource(agent, &mut capability, v, kind, None);
            *frame.env.disposable_resource_stack_mut(agent) = capability;
            result?;
        }
        Op::DisposeStart => {
            let kind = frame.pop();
            let value = frame.pop();
            let kind = kind.as_number().unwrap_or(0.0);
            let stack = std::mem::take(frame.env.disposable_resource_stack_mut(agent));
            let completion = if kind == 1.0 { Err(value) } else { Ok(value) };
            let state = agent.object_with_kind(
                None,
                ObjectKind::Native(Box::new(DisposeState {
                    dispose: Some(DisposeResources::new(stack, completion)),
                    kind,
                })),
            );
            frame.push(Value::Object(state));
        }
        Op::DisposeStep(done) => {
            let state = dispose_state(frame.peek());
            let mut dispose = take_dispose(agent, state);
            let step = dispose.step(agent);
            put_dispose(agent, state, dispose);
            match step? {
                DisposeStep::Done => return Ok(Flow::Jump(done as usize)),
                DisposeStep::Await(v) => frame.push(v),
            }
        }
        Op::DisposeAwaitThrew => {
            let e = frame.pop();
            let state = dispose_state(frame.peek());
            let mut dispose = take_dispose(agent, state);
            let result = dispose.await_threw(agent, e);
            put_dispose(agent, state, dispose);
            result?;
        }
        Op::DisposeEnd => {
            let state = dispose_state(frame.pop());
            let s = agent
                .native::<DisposeState>(state)
                .expect("a DisposeResources state");
            let kind = s.kind;
            match s
                .dispose
                .as_ref()
                .expect("a DisposeResources state")
                .output()
            {
                // A normal (or break/continue/return) completion goes on as it was.
                Ok(v) => {
                    frame.push(v);
                    frame.push(Value::Number(kind));
                }
                Err(e) => {
                    frame.push(e);
                    frame.push(Value::Number(1.0));
                }
            }
        }

        // ---- functions and calls
        Op::Closure(i) => {
            let code = frame.code.functions[i as usize].clone();
            let f = create_closure(agent, frame.env, frame.private_env.clone(), code, None)?;
            frame.push(Value::Object(f));
        }
        Op::ClosureNamed(i) => {
            let name = frame.pop();
            let key = ops::to_property_key(agent, name)?;
            let code = frame.code.functions[i as usize].clone();
            let f = create_closure(agent, frame.env, frame.private_env.clone(), code, Some(key))?;
            frame.push(Value::Object(f));
        }
        Op::FieldInitializerName => {
            let name = frame
                .function
                .and_then(|f| match &agent.obj(f).kind {
                    ObjectKind::Function(e) => e.class_field_initializer_name,
                    _ => None,
                })
                .map(|n| match n {
                    crate::function::FieldName::Key(k) => agent.key_to_value(k),
                    crate::function::FieldName::Private(p) => Value::String(p.description),
                })
                .unwrap_or(Value::String(agent.names.empty));
            frame.push(name);
        }
        // The call instructions leave the callee, this and the arguments on
        // the operand stack until the callee returns: the frame roots them
        // (docs/GC.md).
        Op::Call(argc) => call_op(agent, frame, argc)?,
        Op::CallSpread => call_spread_op(agent, frame)?,
        Op::CallEval(argc) => {
            let base = frame.stack.len() - argc as usize - 2;
            let (f, this) = (frame.stack[base], frame.stack[base + 1]);
            let args = frame.stack[base + 2..].to_vec();
            let r = call_maybe_eval(agent, frame, f, this, &args)?;
            frame.stack.truncate(base);
            frame.push(r);
        }
        Op::CallEvalSpread => {
            let base = frame.stack.len() - 3;
            let (f, this, array) = (
                frame.stack[base],
                frame.stack[base + 1],
                frame.stack[base + 2],
            );
            let args = array_elements(agent, array)?;
            let r = call_maybe_eval(agent, frame, f, this, &args)?;
            frame.stack.truncate(base);
            frame.push(r);
        }
        Op::New(argc) => new_op(agent, frame, argc)?,
        Op::NewSpread => {
            let base = frame.stack.len() - 2;
            let (c, array) = (frame.stack[base], frame.stack[base + 1]);
            let args = array_elements(agent, array)?;
            let r = construct_value(agent, c, &args)?;
            frame.stack.truncate(base);
            frame.push(r);
        }
        Op::GetSuperConstructor => {
            let env = agent.get_this_environment(frame.env);
            let active = env.function_data(agent).map(|f| f.function_object);
            let func = match active {
                Some(f) => f
                    .internal_get_prototype_of(agent)?
                    .map(Value::Object)
                    .unwrap_or(Value::Null),
                None => Value::Undefined,
            };
            frame.push(func);
        }
        Op::SuperCall(argc) => {
            let base = frame.stack.len() - argc as usize - 1;
            let func = frame.stack[base];
            let this = call::super_call(agent, frame.env, func, &frame.stack[base + 1..])?;
            frame.stack.truncate(base);
            frame.push(this);
        }
        Op::SuperCallSpread => {
            let base = frame.stack.len() - 2;
            let (func, array) = (frame.stack[base], frame.stack[base + 1]);
            let args = array_elements(agent, array)?;
            let this = call::super_call(agent, frame.env, func, &args)?;
            frame.stack.truncate(base);
            frame.push(this);
        }
        Op::SuperCallForward => {
            let env = agent.get_this_environment(frame.env);
            let active = env.function_data(agent).map(|f| f.function_object);
            let func = match active {
                Some(f) => f
                    .internal_get_prototype_of(agent)?
                    .map(Value::Object)
                    .unwrap_or(Value::Null),
                None => Value::Undefined,
            };
            let args = frame.args.clone();
            let this = call::super_call(agent, frame.env, func, &args)?;
            frame.push(this);
        }
        Op::Return => {
            let v = frame.pop();
            let v = match frame.code.kind {
                CodeKind::Script | CodeKind::Eval => frame.completion,
                _ => v,
            };
            return Ok(Flow::Done(FrameResult::Return(v)));
        }
        Op::This => {
            let v = resolve_this_binding(agent, frame.env)?;
            frame.push(v);
        }
        Op::NewTarget => {
            let env = agent.get_this_environment(frame.env);
            let nt = env
                .function_data(agent)
                .map(|f| f.new_target)
                .unwrap_or(Value::Undefined);
            frame.push(nt);
        }
        Op::ImportMeta => {
            let v = module::import_meta(agent)?;
            frame.push(v);
        }
        Op::ImportCall => {
            let options = frame.pop();
            let specifier = frame.pop();
            let p = module::evaluate_import_call(agent, specifier, options)?;
            frame.push(p);
        }
        Op::GetArg(i) => {
            let v = frame
                .args
                .get(i as usize)
                .copied()
                .unwrap_or(Value::Undefined);
            frame.push(v);
        }
        Op::GetRestArgs(i) => {
            let rest: Vec<Value> = frame.args.iter().skip(i as usize).copied().collect();
            let a = ops::create_array_from_list(agent, &rest);
            frame.push(Value::Object(a));
        }
        Op::InstantiateFunctionBody => instantiate::function_body_instantiation(agent, frame)?,

        // ---- literals
        Op::NewObject => {
            let proto = agent.intrinsic_c("%Object.prototype%");
            let o = agent.ordinary_object_create(Some(proto));
            frame.push(Value::Object(o));
        }
        Op::NewArray => {
            let a = crate::object::array::array_create(agent, 0.0, None)?;
            frame.push(Value::Object(a));
        }
        Op::DefineField(key) => {
            let v = frame.pop();
            let Value::Object(o) = frame.peek() else {
                unreachable!("object literal")
            };
            ops::create_data_property_or_throw(agent, o, key, v)?;
        }
        Op::DefineFieldComputed => {
            let v = frame.pop();
            let key = frame.pop();
            let Value::Object(o) = frame.peek() else {
                unreachable!("object literal")
            };
            let k = ops::to_property_key(agent, key)?;
            ops::create_data_property_or_throw(agent, o, k, v)?;
        }
        Op::ArrayPush => {
            let v = frame.pop();
            let a = frame.peek();
            array_append(agent, a, v)?;
        }
        Op::ArrayHole => {
            let Value::Object(a) = frame.peek() else {
                unreachable!("array literal")
            };
            let len = crate::object::array::array_length(agent, a);
            let key = PropertyKey::String(agent.names.length);
            ops::set(agent, a, key, Value::Number(len as f64 + 1.0), true)?;
        }
        Op::ArraySpread => {
            let iterable = frame.pop();
            let a = frame.peek();
            let mut record = ops::get_iterator(agent, iterable, ops::IteratorKind::Sync)?;
            while let Some(v) = ops::iterator_step_value(agent, &mut record)? {
                array_append(agent, a, v)?;
            }
        }
        Op::CopyDataProperties => {
            let source = frame.pop();
            let Value::Object(o) = frame.peek() else {
                unreachable!("object literal")
            };
            ops::copy_data_properties(agent, o, source, &[])?;
        }
        Op::CopyDataPropertiesExcluding(n) => {
            let source = frame.pop();
            let target = frame.pop();
            let keys = frame.pop_n(n as usize);
            let mut excluded = Vec::with_capacity(keys.len());
            for k in keys {
                excluded.push(ops::to_property_key(agent, k)?);
            }
            let Value::Object(t) = target else {
                unreachable!("rest object")
            };
            ops::copy_data_properties(agent, t, source, &excluded)?;
            frame.push(target);
        }
        Op::SetPrototypeLiteral => {
            let proto = frame.pop();
            let Value::Object(o) = frame.peek() else {
                unreachable!("object literal")
            };
            match proto {
                Value::Object(p) => {
                    o.internal_set_prototype_of(agent, Some(p))?;
                }
                Value::Null => {
                    o.internal_set_prototype_of(agent, None)?;
                }
                _ => {}
            }
        }
        Op::DefineMethod(def) => {
            let closure = frame.pop();
            let key = frame.pop();
            let Value::Object(target) = frame.peek() else {
                unreachable!("method target")
            };
            let Value::Object(f) = closure else {
                unreachable!("method closure")
            };
            let k = ops::to_property_key(agent, key)?;
            call::define_method_property(agent, target, k, f, def)?;
        }
        Op::GetTemplateObject(i) => {
            let code = frame.code.clone();
            let t = call::get_template_object(agent, &code, &code.templates[i as usize])?;
            frame.push(Value::Object(t));
        }
        Op::RegExp(pattern, flags) => {
            let ctor = agent.intrinsic_c("%RegExp%");
            let r = ops::construct(
                agent,
                ctor,
                &[Value::String(pattern), Value::String(flags)],
                None,
            )?;
            frame.push(Value::Object(r));
        }

        // ---- iteration
        Op::GetIterator => {
            let v = frame.pop();
            let record = ops::get_iterator(agent, v, ops::IteratorKind::Sync)?;
            frame.iters.push(FrameIter::Iter(record));
        }
        Op::GetAsyncIterator => {
            let v = frame.pop();
            let record = ops::get_iterator(agent, v, ops::IteratorKind::Async)?;
            frame.iters.push(FrameIter::Iter(record));
        }
        Op::IteratorCallNext => {
            let FrameIter::Iter(record) = frame.top_iter() else {
                unreachable!("iterator expected")
            };
            let record = *record;
            let r = ops::call(
                agent,
                record.next_method,
                Value::Object(record.iterator),
                &[],
            );
            if r.is_err()
                && let FrameIter::Iter(top) = frame.top_iter()
            {
                top.done = true;
            }
            frame.push(r?);
        }
        Op::IteratorResultUnwrap(l) => {
            let result = frame.pop();
            // If nextResult is not an Object, throw a TypeError exception.
            let unwrapped = match result {
                Value::Object(o) => match ops::iterator_complete(agent, o) {
                    // Let done be ? IteratorComplete(nextResult). If done is true, the loop ends.
                    Ok(true) => Ok(None),
                    // Let nextValue be ? IteratorValue(nextResult).
                    Ok(false) => ops::iterator_value(agent, o).map(Some),
                    Err(e) => Err(e),
                },
                _ => Err(agent.throw_type_error("iterator result is not an object")),
            };
            let finished = !matches!(unwrapped, Ok(Some(_)));
            if finished && let FrameIter::Iter(top) = frame.top_iter() {
                top.done = true;
            }
            match unwrapped? {
                Some(v) => frame.push(v),
                None => return Ok(Flow::Jump(l as usize)),
            }
        }
        Op::AsyncIteratorClose(l) => {
            let Some(FrameIter::Iter(record)) = frame.iters.pop() else {
                unreachable!("iterator expected")
            };
            // 3. Let innerResult be Completion(GetMethod(iterator, "return")).
            let return_key = PropertyKey::String(agent.names.r#return);
            let r = ops::get_method(agent, Value::Object(record.iterator), return_key)?;
            // 4.b If return is undefined, return ? completion.
            if r.is_undefined() {
                return Ok(Flow::Jump(l as usize));
            }
            // 4.c Set innerResult to Completion(Call(return, iterator)); the Await follows.
            let inner = ops::call(agent, r, Value::Object(record.iterator), &[])?;
            frame.push(inner);
        }
        Op::AsyncIteratorCloseForThrow(l) => {
            let Some(FrameIter::Iter(record)) = frame.iters.pop() else {
                unreachable!("iterator expected")
            };
            if record.done {
                return Ok(Flow::Jump(l as usize));
            }
            // With a throw completion, errors while closing are ignored.
            let return_key = PropertyKey::String(agent.names.r#return);
            let r = match ops::get_method(agent, Value::Object(record.iterator), return_key) {
                Ok(r) if !r.is_undefined() => r,
                Err(JsError::NotImplemented(m)) => return Err(JsError::NotImplemented(m)),
                _ => return Ok(Flow::Jump(l as usize)),
            };
            match ops::call(agent, r, Value::Object(record.iterator), &[]) {
                Ok(inner) => frame.push(inner),
                Err(JsError::NotImplemented(m)) => return Err(JsError::NotImplemented(m)),
                Err(_) => return Ok(Flow::Jump(l as usize)),
            }
        }
        Op::SetDelegateKind(k) => frame.delegate_kind = k,
        Op::JumpIfDelegateKind(k, l) => {
            if frame.delegate_kind == k {
                return Ok(Flow::Jump(l as usize));
            }
        }
        Op::AsyncDelegateCall(l) => {
            let received = frame.peek();
            let FrameIter::Iter(record) = frame.top_iter() else {
                unreachable!("yield* iterator")
            };
            let record = *record;
            let iterator = Value::Object(record.iterator);
            let inner = if frame.delegate_kind == 1 {
                // b.i Let throw be ? GetMethod(iterator, "throw").
                let throw_key = PropertyKey::String(agent.names.throw);
                let throw = ops::get_method(agent, iterator, throw_key)?;
                if throw.is_undefined() {
                    return Ok(Flow::Jump(l as usize));
                }
                // b.ii.1 Let innerResult be ? Call(throw, iterator, « received.[[Value]] »).
                ops::call(agent, throw, iterator, &[received])?
            } else {
                // a.i Let innerResult be ? Call(iteratorRecord.[[NextMethod]], iteratorRecord.[[Iterator]], « received.[[Value]] »).
                ops::call(agent, record.next_method, iterator, &[received])?
            };
            frame.pop();
            frame.push(inner);
        }
        Op::AsyncDelegateReturn(l) => {
            let received = frame.peek();
            let FrameIter::Iter(record) = frame.top_iter() else {
                unreachable!("yield* iterator")
            };
            let iterator = Value::Object(record.iterator);
            // c.ii Let return be ? GetMethod(iterator, "return").
            let return_key = PropertyKey::String(agent.names.r#return);
            let r = ops::get_method(agent, iterator, return_key)?;
            if r.is_undefined() {
                return Ok(Flow::Jump(l as usize));
            }
            // c.iv Let innerReturnResult be ? Call(return, iterator, « received.[[Value]] »).
            let inner = ops::call(agent, r, iterator, &[received])?;
            frame.pop();
            frame.push(inner);
        }
        Op::AsyncDelegateResult(l) => {
            // If innerResult is not an Object, throw a TypeError exception.
            let Value::Object(inner) = frame.pop() else {
                return Err(agent.throw_type_error("iterator result is not an object"));
            };
            // Let done be ? IteratorComplete(innerResult).
            let done = ops::iterator_complete(agent, inner)?;
            // ? IteratorValue(innerResult)
            let value = ops::iterator_value(agent, inner)?;
            frame.push(value);
            if done {
                frame.iters.pop();
                return Ok(Flow::Jump(l as usize));
            }
        }
        Op::DelegateYield => {
            let v = frame.pop();
            frame.delegating = true;
            return Ok(Flow::Done(FrameResult::Yield(v)));
        }
        Op::IteratorCheckObject => {
            if !frame.pop().is_object() {
                return Err(agent.throw_type_error("iterator.return() did not return an object"));
            }
        }
        Op::IteratorNext(l) => {
            let FrameIter::Iter(record) = frame.top_iter() else {
                unreachable!("iterator expected")
            };
            let mut record = *record;
            // The record is rooted by the frame's iterator stack.
            let r = ops::iterator_step_value_rooted(agent, &mut record);
            if let FrameIter::Iter(top) = frame.top_iter() {
                *top = record;
            }
            match r? {
                Some(v) => frame.push(v),
                None => return Ok(Flow::Jump(l as usize)),
            }
        }
        Op::IteratorNextOrUndefined => {
            let FrameIter::Iter(record) = frame.top_iter() else {
                unreachable!("iterator expected")
            };
            let mut record = *record;
            let r = if record.done {
                Ok(None)
            } else {
                // The record is rooted by the frame's iterator stack.
                ops::iterator_step_value_rooted(agent, &mut record)
            };
            if let FrameIter::Iter(top) = frame.top_iter() {
                *top = record;
            }
            frame.push(r?.unwrap_or(Value::Undefined));
        }
        Op::IteratorSkip => {
            let FrameIter::Iter(record) = frame.top_iter() else {
                unreachable!("iterator expected")
            };
            let mut record = *record;
            // Elision: if iteratorRecord.[[Done]] is false, then
            //   Let next be ? IteratorStep(iteratorRecord) (done sets [[Done]]).
            let r = if record.done {
                Ok(None)
            } else {
                ops::iterator_step_rooted(agent, &mut record)
            };
            if let FrameIter::Iter(top) = frame.top_iter() {
                *top = record;
            }
            r?;
        }
        Op::IteratorRest => {
            let FrameIter::Iter(record) = frame.top_iter() else {
                unreachable!("iterator expected")
            };
            let mut record = *record;
            let mut values = Vec::new();
            let mut result = Ok(());
            while !record.done {
                match ops::iterator_step_value(agent, &mut record) {
                    Ok(Some(v)) => values.push(v),
                    Ok(None) => break,
                    Err(e) => {
                        result = Err(e);
                        break;
                    }
                }
            }
            if let FrameIter::Iter(top) = frame.top_iter() {
                *top = record;
            }
            result?;
            let a = ops::create_array_from_list(agent, &values);
            frame.push(Value::Object(a));
        }
        Op::PopIterator => {
            frame.iters.pop();
        }
        Op::IteratorClose => {
            if let Some(FrameIter::Iter(record)) = frame.iters.pop()
                && !record.done
            {
                ops::iterator_close(agent, &record, Ok(()))?;
            }
        }
        Op::IteratorCloseForThrow => {
            let exception = frame.peek();
            if let Some(FrameIter::Iter(record)) = frame.iters.pop()
                && !record.done
            {
                let r: JsResult<()> =
                    ops::iterator_close(agent, &record, Err(JsError::Throw(exception)));
                if let Err(JsError::NotImplemented(m)) = r {
                    return Err(JsError::NotImplemented(m));
                }
            }
        }
        Op::ForInStart => {
            let v = frame.pop();
            let object = if v.is_nullish() {
                None
            } else {
                Some(ops::to_object(agent, v)?)
            };
            frame
                .iters
                .push(FrameIter::ForIn(ForInIterator::new(object)));
        }
        Op::ForInNext(l) => {
            let FrameIter::ForIn(it) = frame.iters.last_mut().expect("for-in iterator") else {
                unreachable!()
            };
            let mut it = std::mem::replace(it, ForInIterator::new(None));
            let r = it.next(agent);
            if let Some(FrameIter::ForIn(top)) = frame.iters.last_mut() {
                *top = it;
            }
            match r? {
                Some(k) => {
                    let v = agent.key_to_value(k);
                    frame.push(v);
                }
                None => return Ok(Flow::Jump(l as usize)),
            }
        }

        // ---- generators and async
        Op::GeneratorStart => return Ok(Flow::Done(FrameResult::Yield(Value::Undefined))),
        Op::Yield(l) => {
            let v = frame.pop();
            frame.yield_return_pc = l as usize;
            return Ok(Flow::Done(FrameResult::Yield(v)));
        }
        Op::YieldDelegate(ret) => return yield_delegate(agent, frame, ret as usize),
        Op::Await => {
            let v = frame.pop();
            return Ok(Flow::Done(FrameResult::Await(v)));
        }

        // ---- classes
        Op::ClassHeritage(has_heritage) => {
            let (proto_parent, ctor_parent) = if has_heritage {
                let superclass = frame.pop();
                call::class_heritage(agent, superclass)?
            } else {
                (
                    Some(agent.intrinsic_c("%Object.prototype%")),
                    agent.intrinsic_c("%Function.prototype%"),
                )
            };
            frame.push(proto_parent.map(Value::Object).unwrap_or(Value::Null));
            frame.push(Value::Object(ctor_parent));
        }
        Op::CreateClass(info) => {
            let ctor_parent = frame.pop();
            let proto_parent = frame.pop();
            let name = frame.pop();
            let code = frame.code.clone();
            let ctor_code =
                code.functions[info.constructor.expect("class constructor") as usize].clone();
            let class = &code.classes[info.class as usize];
            let (f, proto) = call::create_class(
                agent,
                frame,
                ctor_code,
                class,
                name,
                proto_parent,
                ctor_parent,
            )?;
            frame.push(Value::Object(f));
            frame.push(Value::Object(proto));
        }
        Op::PushPrivateScope(i) => {
            let names = frame.code.classes[i as usize].private_names.clone();
            let names = names
                .into_iter()
                .map(|d| agent.new_private_name(d))
                .collect();
            frame.private_env = Some(Arc::new(PrivateEnv {
                outer: frame.private_env.clone(),
                names,
            }));
        }
        Op::PopPrivateScope => {
            frame.private_env = frame.private_env.as_ref().and_then(|p| p.outer.clone());
        }
        Op::DefineClassField(def) => call::define_class_field(agent, frame, def)?,
        Op::DefinePrivateMethod(def) => call::define_private_method(agent, frame, def)?,
        Op::FinishClass => {
            let Value::Object(f) = frame.peek() else {
                unreachable!("class constructor")
            };
            call::finish_class(agent, f)?;
        }

        // ---- misc
        Op::NotImplemented(m) => {
            let msg = frame.code.messages[m as usize].clone();
            return Err(agent.not_implemented(&msg));
        }
        Op::Debugger | Op::Nop => {}
    }
    Ok(Flow::Next)
}

/// One step of `yield*` in a sync generator (§15.5.5 steps 5-7). The
/// delegate iterator is on the iterator stack and the received value on the
/// operand stack; the step runs again each time the generator resumes.
fn yield_delegate(agent: &mut Agent, frame: &mut Frame, ret: usize) -> JsResult<Flow> {
    if frame.code.is_async {
        return Err(agent.not_implemented("yield* in async generators"));
    }
    let received = frame.pop();
    let kind = std::mem::replace(&mut frame.delegate_kind, 0);
    let FrameIter::Iter(record) = frame.top_iter() else {
        unreachable!("yield* iterator")
    };
    let record = *record;
    let iterator = Value::Object(record.iterator);
    let inner = match kind {
        // a. If received is a normal completion, then
        //    i. Let innerResult be ? Call(iteratorRecord.[[NextMethod]], iteratorRecord.[[Iterator]], « received.[[Value]] »).
        0 => ops::call(agent, record.next_method, iterator, &[received])?,
        // b. Else if received is a throw completion, then
        1 => {
            // i. Let throw be ? GetMethod(iterator, "throw").
            let throw_key = PropertyKey::String(agent.names.throw);
            let throw = ops::get_method(agent, iterator, throw_key)?;
            // iii. Else (throw is undefined): close the iterator, then throw a TypeError.
            if throw.is_undefined() {
                frame.iters.pop();
                ops::iterator_close(agent, &record, Ok(()))?;
                return Err(
                    agent.throw_type_error("The iterator does not provide a 'throw' method")
                );
            }
            // ii.1 Let innerResult be ? Call(throw, iterator, « received.[[Value]] »).
            ops::call(agent, throw, iterator, &[received])?
        }
        // c. Else (a return completion),
        _ => {
            // ii. Let return be ? GetMethod(iterator, "return").
            let return_key = PropertyKey::String(agent.names.r#return);
            let r = ops::get_method(agent, iterator, return_key)?;
            // iii. If return is undefined, then return ReturnCompletion(received.[[Value]]).
            if r.is_undefined() {
                frame.iters.pop();
                frame.push(received);
                return Ok(Flow::Jump(ret));
            }
            // iv. Let innerReturnResult be ? Call(return, iterator, « received.[[Value]] »).
            ops::call(agent, r, iterator, &[received])?
        }
    };
    // If innerResult is not an Object, throw a TypeError exception.
    let Value::Object(inner_obj) = inner else {
        return Err(agent.throw_type_error("iterator result is not an object"));
    };
    // Let done be ? IteratorComplete(innerResult).
    if ops::iterator_complete(agent, inner_obj)? {
        // If done is true, return ? IteratorValue(innerResult) (a ReturnCompletion for return).
        let value = ops::iterator_value(agent, inner_obj)?;
        frame.iters.pop();
        frame.push(value);
        return Ok(if kind == 2 {
            Flow::Jump(ret)
        } else {
            Flow::Next
        });
    }
    // Set received to Completion(GeneratorYield(innerResult)).
    frame.delegating = true;
    frame.resume_pc = Some(frame.pc);
    Ok(Flow::Done(FrameResult::YieldDelegate(inner)))
}

/// Call(func, this, args) with a TypeError naming non-callables.
/// Call from an instruction whose operands stay on the frame's stack (rooted).
/// Op::Call: [func this args...] -> [result].
#[inline(never)]
fn call_op(agent: &mut Agent, frame: &mut Frame, argc: u32) -> JsResult<()> {
    let base = frame.stack.len() - argc as usize - 2;
    let (f, this) = (frame.stack[base], frame.stack[base + 1]);
    let r = call_value(agent, f, this, &frame.stack[base + 2..])?;
    frame.stack.truncate(base);
    frame.push(r);
    Ok(())
}

/// Op::CallSpread: [func this array] -> [result].
#[inline(never)]
fn call_spread_op(agent: &mut Agent, frame: &mut Frame) -> JsResult<()> {
    let base = frame.stack.len() - 3;
    let (f, this, array) = (
        frame.stack[base],
        frame.stack[base + 1],
        frame.stack[base + 2],
    );
    let args = array_elements(agent, array)?;
    let r = call_value(agent, f, this, &args)?;
    frame.stack.truncate(base);
    frame.push(r);
    Ok(())
}

/// Op::New: [constructor args...] -> [result].
#[inline(never)]
fn new_op(agent: &mut Agent, frame: &mut Frame, argc: u32) -> JsResult<()> {
    let base = frame.stack.len() - argc as usize - 1;
    let c = frame.stack[base];
    let r = construct_value(agent, c, &frame.stack[base + 1..])?;
    frame.stack.truncate(base);
    frame.push(r);
    Ok(())
}

fn call_value(agent: &mut Agent, f: Value, this: Value, args: &[Value]) -> JsResult<Value> {
    match f {
        Value::Object(o) if o.is_callable(agent) => o.internal_call_rooted(agent, this, args),
        _ => Err(not_callable(agent, f)),
    }
}

#[cold]
#[inline(never)]
fn not_callable(agent: &mut Agent, f: Value) -> JsError {
    callable_error(agent, &describe(agent, f))
}

fn construct_value(agent: &mut Agent, c: Value, args: &[Value]) -> JsResult<Value> {
    match c {
        Value::Object(o) if o.is_constructor(agent) => {
            Ok(Value::Object(o.internal_construct_rooted(agent, args, o)?))
        }
        _ => Err(not_constructor(agent, c)),
    }
}

#[cold]
#[inline(never)]
fn not_constructor(agent: &mut Agent, c: Value) -> JsError {
    let msg = format!("{} is not a constructor", describe(agent, c));
    agent.throw_type_error(&msg)
}

/// A short description of a value for error messages.
fn describe(agent: &Agent, v: Value) -> String {
    match v {
        Value::Undefined => "undefined".into(),
        Value::Null => "null".into(),
        Value::Boolean(b) => b.to_string(),
        Value::Number(n) => kahvi_parser::number::number_to_string(n),
        Value::String(s) => format!("\"{}\"", agent.str_lossy(s)),
        Value::Symbol(_) => "a symbol".into(),
        Value::BigInt(_) => "a bigint".into(),
        Value::Object(_) => "object".into(),
    }
}

/// A call through `eval` (§13.3.6.1 step 6): a direct eval if the callee is
/// the realm's %eval%.
fn call_maybe_eval(
    agent: &mut Agent,
    frame: &mut Frame,
    f: Value,
    this: Value,
    args: &[Value],
) -> JsResult<Value> {
    if let Value::Object(o) = f
        && agent.has_intrinsic("%eval%")
        && o == agent.intrinsic_c("%eval%")
    {
        // a. If argList has no elements, return undefined.
        let Some(&arg) = args.first() else {
            return Ok(Value::Undefined);
        };
        // b-e. Return ? PerformEval(evalArg, strictCaller, true).
        return perform_eval(agent, arg, frame.strict(), Some(frame));
    }
    call_value(agent, f, this, args)
}

/// The realm's global object.
pub fn global_object(agent: &Agent) -> Object {
    let realm = agent.current_realm();
    agent.realm(realm).global_object
}

/// The ThisMode of a function object, if it is an ECMAScript function.
pub fn this_mode_of(agent: &Agent, f: Object) -> Option<ThisMode> {
    match &agent.obj(f).kind {
        ObjectKind::Function(e) => Some(e.this_mode),
        _ => None,
    }
}

// ---- garbage collection ----------------------------------------------------

use crate::gc::{Trace, Tracer};

impl Trace for PrivateEnv {
    fn trace(&self, t: &mut Tracer) {
        self.outer.trace(t);
        self.names.trace(t);
    }
}

impl Trace for Reference {
    fn trace(&self, t: &mut Tracer) {
        match self {
            Reference::Env(env, name) | Reference::Slot(env, _, name) => {
                t.env(*env);
                t.string(*name);
            }
            Reference::Unresolvable(name) => t.string(*name),
            // (The frame traces its locals.)
            Reference::Local(_) => {}
        }
    }
}

impl Trace for ForInIterator {
    fn trace(&self, t: &mut Tracer) {
        self.object.trace(t);
        self.visited.trace(t);
        self.remaining.trace(t);
    }
}

impl Trace for FrameIter {
    fn trace(&self, t: &mut Tracer) {
        match self {
            FrameIter::Iter(r) => r.trace(t),
            FrameIter::ForIn(f) => f.trace(t),
        }
    }
}

impl Trace for Handler {
    fn trace(&self, t: &mut Tracer) {
        t.env(self.env);
        self.private_env.trace(t);
    }
}

impl Trace for PendingBody {
    fn trace(&self, t: &mut Tracer) {
        self.env_record.trace(t);
    }
}

impl Trace for Frame {
    fn trace(&self, t: &mut Tracer) {
        t.code(&self.code);
        self.stack.trace(t);
        self.refs.trace(t);
        self.iters.trace(t);
        self.handlers.trace(t);
        t.env(self.env);
        t.env(self.var_env);
        self.private_env.trace(t);
        self.function.trace(t);
        self.args.trace(t);
        t.value(self.completion);
        self.pending.trace(t);
        for v in self.locals.iter().flatten() {
            t.value(*v);
        }
    }
}

impl Trace for DisposeState {
    fn trace(&self, t: &mut Tracer) {
        self.dispose.trace(t);
    }
}
