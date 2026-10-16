//! ECMAScript function objects at run time: creation (§10.2.3), [[Call]]
//! (§10.2.1), [[Construct]] (§10.2.2), super calls, and class definition
//! support (§15.7).

use std::sync::Arc;

use super::{Frame, FrameResult, PrivateEnv, generator, instantiate, run_frame};
use crate::agent::{Agent, ExecutionContext};
use crate::compiler::{
    ClassStatic, FieldDef, FunctionCode, FunctionFlavor, LocalKind, MethodDef, MethodKind,
    PrivateMethodDef, TemplateInfo,
};
use crate::env::{EnvId, Environment};
use crate::error::JsResult;
use crate::function::{
    ClassFieldDefinition, ClassPending, ConstructorKind, EcmaFunction, FieldName, PrivateElement,
    PrivateElementKind, PrivateName, ThisMode, make_constructor, set_function_length,
    set_function_name,
};
use crate::object::{ObjectData, ObjectKind, Property, PropertyDescriptor, PropertyKey};
use crate::ops;
use crate::value::{Object, Value};

/// The intrinsic [[Prototype]] of a function with this code.
fn function_prototype_intrinsic(code: &FunctionCode) -> &'static str {
    match (code.is_async, code.is_generator) {
        (false, false) => "%Function.prototype%",
        (false, true) => "%GeneratorFunction.prototype%",
        (true, false) => "%AsyncFunction.prototype%",
        (true, true) => "%AsyncGeneratorFunction.prototype%",
    }
}

/// OrdinaryFunctionCreate(functionPrototype, sourceText, ParameterList,
/// Body, thisMode, env, privateEnv) (§10.2.3)
pub(crate) fn ordinary_function_create(
    agent: &mut Agent,
    proto: Object,
    code: Arc<FunctionCode>,
    env: EnvId,
    private_env: Option<Arc<PrivateEnv>>,
) -> Object {
    let realm = agent.current_realm();
    let script_or_module = Some(super::script_or_module_of(agent, code.script));
    let length = code.length;
    let this_mode = code.this_mode;
    let f = agent.alloc(ObjectData::new(
        Some(proto),
        ObjectKind::Function(Box::new(EcmaFunction {
            code,
            environment: env,
            private_environment: private_env,
            home_object: None,
            this_mode,
            constructor_kind: ConstructorKind::Base,
            is_class_constructor: false,
            constructor: false,
            realm,
            script_or_module,
            fields: Vec::new(),
            private_methods: Vec::new(),
            class_field_initializer_name: None,
            source_override: None,
            class_pending: None,
        })),
    ));
    // 22. Perform SetFunctionLength(F, len).
    set_function_length(agent, f, length as f64);
    f
}

/// Creates the function object for a function declaration or expression,
/// arrow function, method or class field initializer
/// (InstantiateFunctionObject, InstantiateOrdinaryFunctionExpression, …).
/// `name` overrides the name for NamedEvaluation.
pub fn create_closure(
    agent: &mut Agent,
    env: EnvId,
    private_env: Option<Arc<PrivateEnv>>,
    code: Arc<FunctionCode>,
    name: Option<PropertyKey>,
) -> JsResult<Object> {
    let proto = agent.intrinsic_c(function_prototype_intrinsic(&code));
    // A named function expression binds its own name in funcEnv.
    let own_name = code.name.filter(|_| code.binds_own_name);
    let func_env = match own_name {
        Some(n) => {
            let fe = Environment::new_declarative(agent, Some(env));
            fe.create_immutable_binding(agent, n, false)?;
            fe
        }
        None => env,
    };
    let f = ordinary_function_create(agent, proto, code.clone(), func_env, private_env);
    let name = name.unwrap_or_else(|| PropertyKey::String(code.name.unwrap_or(agent.names.empty)));
    set_function_name(agent, f, name, None);
    match (code.is_async, code.is_generator) {
        // Plain functions (not arrows or methods) are constructors.
        (false, false) if code.flavor == FunctionFlavor::Normal => {
            if !code.strict {
                add_legacy_caller(agent, f);
            }
            make_constructor(agent, f, true, None)
        }
        (_, true) => {
            let intrinsic = if code.is_async {
                "%AsyncGeneratorPrototype%"
            } else {
                "%GeneratorPrototype%"
            };
            let proto_proto = agent.intrinsic_c(intrinsic);
            let prototype = agent.ordinary_object_create(Some(proto_proto));
            let key = PropertyKey::String(agent.names.prototype);
            agent.obj_mut(f).properties.insert(
                key,
                Property::Data {
                    value: Value::Object(prototype),
                    writable: true,
                    enumerable: false,
                    configurable: false,
                },
            );
        }
        _ => {}
    }
    if let Some(n) = own_name {
        func_env.initialize_binding(agent, n, Value::Object(f))?;
    }
    Ok(f)
}

/// The "caller" own property that web engines give non-strict plain
/// functions (a legacy extension; reading it must not reach
/// %Function.prototype%'s throwing accessor). Forbidden Extensions (§17.1)
/// allow it for non-strict functions only, and it must never reveal a
/// strict function: its getter returns the function that called the
/// innermost running invocation of this one when that caller is a
/// non-strict ECMAScript function, and null otherwise.
fn add_legacy_caller(agent: &mut Agent, f: Object) {
    let realm = agent.current_realm();
    let getter = match agent.realm(realm).intrinsics.get(LEGACY_CALLER_GETTER) {
        Some(&g) => g,
        None => {
            let name = agent.key("caller");
            let g = crate::function::create_builtin_function(
                agent,
                legacy_caller_getter,
                0,
                name,
                Vec::new(),
                Some(realm),
                None,
                Some("get"),
                false,
            );
            agent.set_intrinsic(realm, LEGACY_CALLER_GETTER, g);
            g
        }
    };
    let key = agent.key("caller");
    agent.obj_mut(f).properties.insert(
        key,
        Property::Accessor {
            get: Some(getter),
            set: None,
            enumerable: false,
            configurable: false,
        },
    );
}

/// The realm's getter for non-strict functions' "caller" property.
const LEGACY_CALLER_GETTER: &str = "%LegacyFunctionCallerGetter%";

fn legacy_caller_getter(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let Value::Object(f) = this else {
        return Ok(Value::Null);
    };
    // The contexts below the getter's own; the innermost one running f.
    let contexts = &agent.contexts[..agent.contexts.len().saturating_sub(1)];
    let Some(i) = contexts.iter().rposition(|c| c.function == Some(f)) else {
        return Ok(Value::Null);
    };
    // The context that called it, if that is a non-strict ECMAScript function.
    let caller = i.checked_sub(1).and_then(|j| contexts[j].function);
    Ok(match caller {
        Some(c) if matches!(&agent.obj(c).kind, ObjectKind::Function(e) if !e.code.strict) => {
            Value::Object(c)
        }
        _ => Value::Null,
    })
}

fn ecma(agent: &Agent, f: Object) -> &EcmaFunction {
    match &agent.obj(f).kind {
        ObjectKind::Function(e) => e,
        _ => unreachable!("ECMAScript function object expected"),
    }
}

fn ecma_mut(agent: &mut Agent, f: Object) -> &mut EcmaFunction {
    match &mut agent.obj_mut(f).kind {
        ObjectKind::Function(e) => e,
        _ => unreachable!("ECMAScript function object expected"),
    }
}

/// PrepareForOrdinaryCall(F, newTarget) (§10.2.1.1): pushes the callee
/// context and returns the new function environment.
fn prepare_for_ordinary_call(agent: &mut Agent, f: Object, new_target: Value) -> JsResult<EnvId> {
    // (A lazy function is compiled first: whether the call gets a function
    // environment depends on its code, and every call of the code must
    // agree, since its binding caches count environments.)
    let code = ecma(agent, f).code.clone();
    if code.lazy.is_some() {
        compile_lazy(agent, f, &code)?;
    }
    let e = ecma(agent, f);
    let (realm, script_or_module, outer, lexical) = (
        e.realm,
        e.script_or_module,
        e.environment,
        e.this_mode == ThisMode::Lexical,
    );
    let skip = e.code.skips_function_env;
    agent.contexts.push(ExecutionContext {
        function: Some(f),
        realm,
        script_or_module,
    });
    if skip {
        // (`FunctionCode::skips_function_env`; callers tell by the result
        // being the closure's environment.)
        return Ok(outer);
    }
    Ok(Environment::new_function(
        agent, f, new_target, outer, lexical,
    ))
}

/// Whether `env`, from `prepare_for_ordinary_call`, is a function
/// environment of its own for the call of `f`.
fn own_function_env(agent: &Agent, f: Object, env: EnvId) -> bool {
    ecma(agent, f).environment != env
}

/// OrdinaryCallBindThis(F, calleeContext, thisArgument) (§10.2.1.2)
fn ordinary_call_bind_this(agent: &mut Agent, f: Object, env: EnvId, this: Value) -> JsResult<()> {
    let (this_mode, realm) = {
        let e = ecma(agent, f);
        (e.this_mode, e.realm)
    };
    // 2. If thisMode is lexical, return unused.
    //    (Nor is there anything to bind for code that skips its function
    //    environment.)
    if this_mode == ThisMode::Lexical || !own_function_env(agent, f, env) {
        return Ok(());
    }
    let this_value = if this_mode == ThisMode::Strict {
        // 6. If thisMode is strict, let thisValue be thisArgument.
        this
    } else if this.is_nullish() {
        // 7.a.i-iii The global environment's GlobalThisValue.
        let global_env = agent.realm(realm).global_env;
        global_env.get_this_binding(agent)?
    } else {
        // 7.b Let thisValue be ! ToObject(thisArgument).
        Value::Object(ops::to_object(agent, this)?)
    };
    env.bind_this_value(agent, this_value)
}

/// Compiles the lazy function `f` on its first call (`FunctionCode::lazy`):
/// reparses it from its script's source, keeps the code for every closure
/// made from the same function, and gives it to `f`.
#[inline(never)]
fn compile_lazy(
    agent: &mut Agent,
    f: Object,
    stub: &Arc<FunctionCode>,
) -> JsResult<Arc<FunctionCode>> {
    let lazy = stub.lazy.as_ref().expect("lazy code");
    let code = match lazy.compiled.get() {
        Some(code) => code.clone(),
        None => {
            let cache_key = agent.scripts[stub.script as usize].cache_key;
            let cached = cache_key.as_ref().and_then(|key| {
                let bytes = crate::code_cache::function(key, stub.span.0)?;
                crate::compiler::decode_lazy_function(agent, &bytes, stub)
            });
            if let Some(code) = cached {
                let code = lazy.compiled.get_or_init(|| code).clone();
                stub.constants.add_lazy(code.clone());
                ecma_mut(agent, f).code = code.clone();
                return Ok(code);
            }
            let preparse = agent.scripts[stub.script as usize].preparse.clone();
            let ast = match kahvi_parser::parse_lazy_function_preparsed(
                &stub.source,
                stub.span.0 as usize,
                lazy.function,
                preparse,
            ) {
                Ok(ast) => ast,
                // The first parse found no early errors in this source.
                Err(e) => return Err(agent.throw_syntax_error(&e.message)),
            };
            debug_assert_eq!((ast.span.start, ast.span.end), stub.span);
            let code = crate::compiler::compile_lazy_function(agent, &ast, stub, lazy);
            debug_assert!(
                code.lazy.is_none(),
                "a reparsed function is compiled eagerly"
            );
            if let Some(key) = &cache_key
                && let Some(bytes) = crate::compiler::codec::encode(agent, &code)
            {
                crate::code_cache::store_function(key, stub.span.0, bytes);
            }
            let code = match crate::code_cache::stress()
                .then(|| crate::compiler::codec::encode(agent, &code))
                .flatten()
            {
                Some(bytes) => crate::compiler::decode_lazy_function(agent, &bytes, stub)
                    .expect("code cache stress: the encoding of compiled code decodes"),
                None => code,
            };
            let code = lazy.compiled.get_or_init(|| code).clone();
            stub.constants.add_lazy(code.clone());
            code
        }
    };
    ecma_mut(agent, f).code = code.clone();
    Ok(code)
}

/// A frame for the body of `f`.
fn body_frame(
    code: Arc<FunctionCode>,
    env: EnvId,
    private_env: Option<Arc<PrivateEnv>>,
    f: Object,
    args: &[Value],
) -> Frame {
    let mut frame = Frame::new(code, env, private_env, Some(f));
    init_locals(&mut frame);
    frame.args.extend_from_slice(args);
    frame
}

/// Gives a new frame its locals: `var` ones start undefined, the rest
/// uninitialized.
#[inline(always)]
fn init_locals(frame: &mut Frame) {
    let Frame { code, locals, .. } = frame;
    if let Some(infos) = &code.locals {
        locals.extend(
            infos
                .iter()
                .map(|l| (l.kind == LocalKind::Var).then_some(Value::Undefined)),
        );
    }
}

/// OrdinaryCallEvaluateBody: runs the body in a new frame. Returns the
/// frame's result; generator functions return their generator object.
fn evaluate_body(
    agent: &mut Agent,
    f: Object,
    env: EnvId,
    args: &[Value],
) -> JsResult<FrameResult> {
    let (code, private_env) = {
        let e = ecma(agent, f);
        (e.code.clone(), e.private_environment.clone())
    };
    let code = if code.lazy.is_some() {
        compile_lazy(agent, f, &code)?
    } else {
        code
    };
    if code.is_async || code.is_generator {
        return evaluate_suspendable_body(agent, f, code, env, private_env, args);
    }
    // A pooled frame: no allocation, and no large frame on the native stack.
    let mut frame = match agent.spare_frames.pop() {
        Some(mut frame) => {
            frame.reinit(code, env, private_env, Some(f));
            frame
        }
        None => Box::new(Frame::new(code, env, private_env, Some(f))),
    };
    init_locals(&mut frame);
    frame.args.extend_from_slice(args);
    // FunctionDeclarationInstantiation up to the parameter bindings.
    instantiate::function_declaration_instantiation(agent, &mut frame, f, args)?;
    let result = run_frame(agent, &mut frame);
    if frame.code.envs_die_with_call {
        free_scopes_between(agent, frame.env, env);
    }
    let result = result?;
    if agent.spare_frames.len() < 256 && frame.empty_for_pool() {
        agent.spare_frames.push(frame);
    }
    Ok(result)
}

/// Frees the environments a call created inside its function environment
/// `function_env` that are still on the chain from `env` (blocks it left
/// by returning or throwing, and a separate var environment), for code whose
/// environments die with its call.
fn free_scopes_between(agent: &mut Agent, env: EnvId, function_env: EnvId) {
    // The chain must lead to the function environment; anything else is
    // left to the collector.
    let mut e = env;
    while e != function_env {
        match e.outer(agent) {
            Some(outer) => e = outer,
            None => return,
        }
    }
    let mut e = env;
    while e != function_env {
        let outer = e.outer(agent).expect("checked above");
        agent.free_env(e);
        e = outer;
    }
}

/// The rest of OrdinaryCallEvaluateBody for async and generator functions,
/// kept out of `evaluate_body` so its locals take no native stack in every
/// JavaScript call.
#[inline(never)]
fn evaluate_suspendable_body(
    agent: &mut Agent,
    f: Object,
    code: Arc<FunctionCode>,
    env: EnvId,
    private_env: Option<Arc<PrivateEnv>>,
    args: &[Value],
) -> JsResult<FrameResult> {
    let mut frame = body_frame(code.clone(), env, private_env, f, args);
    // FunctionDeclarationInstantiation up to the parameter bindings.
    let decl = instantiate::function_declaration_instantiation(agent, &mut frame, f, args);
    if !code.is_generator {
        // EvaluateAsyncFunctionBody: errors reject the returned promise.
        let promise = super::async_function::start_async_function(agent, f, frame, decl)?;
        return Ok(FrameResult::Return(promise));
    }
    decl?;
    run_frame(agent, &mut frame)?;
    // EvaluateGeneratorBody / EvaluateAsyncGeneratorBody: the frame stopped at GeneratorStart.
    let g = if code.is_async {
        super::async_generator::create_async_generator(agent, f, frame)?
    } else {
        generator::create_generator(agent, f, frame)?
    };
    Ok(FrameResult::Return(Value::Object(g)))
}

/// [[Call]] of an ECMAScript function object (§10.2.1).
pub fn call_ecma_function(
    agent: &mut Agent,
    f: Object,
    this: Value,
    args: &[Value],
) -> JsResult<Value> {
    // 2. Let calleeContext be PrepareForOrdinaryCall(F, undefined).
    let env = prepare_for_ordinary_call(agent, f, Value::Undefined)?;
    // 4. If F.[[IsClassConstructor]] is true, then throw a TypeError (in the callee's realm).
    if ecma(agent, f).is_class_constructor {
        let err = agent.throw_type_error("Class constructor cannot be invoked without 'new'");
        agent.contexts.pop();
        return Err(err);
    }
    // 5. Perform OrdinaryCallBindThis(F, calleeContext, thisArgument).
    let result = ordinary_call_bind_this(agent, f, env, this)
        .and_then(|_| evaluate_body(agent, f, env, args));
    // 7. Remove calleeContext from the execution context stack.
    agent.contexts.pop();
    if ecma(agent, f).code.envs_die_with_call && own_function_env(agent, f, env) {
        agent.free_env(env);
    }
    match result? {
        FrameResult::Return(v) => Ok(v),
        FrameResult::Yield(_) | FrameResult::YieldDelegate(_) | FrameResult::Await(_) => {
            Ok(Value::Undefined)
        }
    }
}

/// A [[Call]] (or base [[Construct]]) of an ECMAScript function that the
/// interpreter runs in its own loop (`run_frame`) instead of a nested Rust
/// call: the callee's frame and what ending the call must undo.
pub(crate) struct InlineCall {
    pub(crate) frame: Box<Frame>,
    /// The frame's code (a handle of the run loop's own, see
    /// `run_frame_loop`).
    pub(crate) code: Arc<FunctionCode>,
    /// A [[Construct]]: its thisArgument is in the frame's `completion`
    /// (unused by function code), which keeps it rooted.
    pub(crate) construct: bool,
    /// The callee's function environment (the function object is the
    /// frame's).
    env: EnvId,
}

/// A frame for code `code` of function `f`, running in `env`.
#[inline(always)]
fn inline_frame(agent: &mut Agent, f: Object, code: &Arc<FunctionCode>, env: EnvId) -> Box<Frame> {
    let private_env = ecma(agent, f).private_environment.clone();
    let mut frame = match agent.spare_frames.pop() {
        Some(mut frame) => {
            frame.reinit(code.clone(), env, private_env, Some(f));
            frame
        }
        None => Box::new(Frame::new(code.clone(), env, private_env, Some(f))),
    };
    init_locals(&mut frame);
    frame
}

/// Undoes `begin_inline_call` / `begin_inline_construct` from
/// PrepareForOrdinaryCall on, for a call that threw before its body ran.
#[cold]
fn abandon_inline_call(agent: &mut Agent, f: Object, env: EnvId) {
    // Remove calleeContext from the execution context stack.
    agent.contexts.pop();
    if ecma(agent, f).code.envs_die_with_call && own_function_env(agent, f, env) {
        agent.free_env(env);
    }
    agent.call_depth -= 1;
}

/// [[Construct]](argumentsList, F) of `f` (§10.2.2) for Op::New, begun as an
/// [`InlineCall`] when `f` is an ECMAScript function of kind base; `None`
/// (with nothing done) for every other constructor.
pub(crate) fn begin_inline_construct(
    agent: &mut Agent,
    f: Object,
    args: &[Value],
) -> JsResult<Option<InlineCall>> {
    let ObjectKind::Function(e) = &agent.obj(f).kind else {
        return Ok(None);
    };
    // 2. Let kind be F.[[ConstructorKind]].
    if !e.is_constructor() || e.constructor_kind != ConstructorKind::Base {
        return Ok(None);
    }
    let code = e.code.clone();
    let code = if code.lazy.is_some() {
        compile_lazy(agent, f, &code)?
    } else {
        code
    };
    crate::function::enter_inline_call(agent)?;
    // 3. If kind is base, then
    // a. Let thisArgument be ? OrdinaryCreateFromConstructor(newTarget, "%Object.prototype%").
    let this = match ops::ordinary_create_from_constructor(
        agent,
        f,
        "%Object.prototype%",
        ObjectKind::Ordinary,
    ) {
        Ok(o) => o,
        Err(e) => {
            agent.call_depth -= 1;
            return Err(e);
        }
    };
    // 4. Let calleeContext be PrepareForOrdinaryCall(F, newTarget).
    let env = match prepare_for_ordinary_call(agent, f, Value::Object(f)) {
        Ok(env) => env,
        Err(e) => {
            agent.call_depth -= 1;
            return Err(e);
        }
    };
    let mut frame = inline_frame(agent, f, &code, env);
    frame.completion = Value::Object(this);
    frame.args.extend_from_slice(args);
    let r = (|| {
        // 6. If kind is base, then
        // a. Perform OrdinaryCallBindThis(F, calleeContext, thisArgument).
        ordinary_call_bind_this(agent, f, env, Value::Object(this))?;
        // b. Let initializeResult be Completion(InitializeInstanceElements(thisArgument, F)).
        // (Field initializers run code; thisArgument is not in a running
        // frame yet.)
        agent.with_roots(&[Value::Object(this)], |agent| {
            initialize_instance_elements(agent, this, f)
        })?;
        // 8. Let result be Completion(OrdinaryCallEvaluateBody(F, argumentsList)).
        // (Its FunctionDeclarationInstantiation up to the parameter bindings.)
        instantiate::function_declaration_instantiation(agent, &mut frame, f, args)
    })();
    match r {
        Ok(()) => Ok(Some(InlineCall {
            frame,
            code,
            construct: true,
            env,
        })),
        Err(e) => {
            abandon_inline_call(agent, f, env);
            Err(e)
        }
    }
}

/// `begin_inline_call` through a bound function, Function.prototype.call
/// or Function.prototype.apply whose target is an ordinary ECMAScript
/// function: the steps of those [[Call]]s that come before their own
/// Call(target, ...) are done here, so the target runs inline. `None`
/// (with nothing observable done) for every other callee, and for an
/// apply whose argument list could only be read by running code.
#[inline(never)]
pub(crate) fn begin_indirect_call(
    agent: &mut Agent,
    f: Object,
    this: Value,
    args: &[Value],
) -> JsResult<Option<InlineCall>> {
    let plain = |agent: &Agent, v: Value| match v {
        Value::Object(o) => match &agent.obj(o).kind {
            ObjectKind::Function(e) if !e.is_class_constructor => Some(o),
            _ => None,
        },
        _ => None,
    };
    match &agent.obj(f).kind {
        ObjectKind::BoundFunction(b) => {
            // §10.4.1.1 [[Call]] ( thisArgument, argumentsList )
            // 1. Let target be F.[[BoundTargetFunction]].
            let Some(target) = plain(agent, Value::Object(b.target)) else {
                return Ok(None);
            };
            // 2. Let boundThis be F.[[BoundThis]].
            let bound_this = b.bound_this;
            // 3. Let boundArgs be F.[[BoundArguments]].
            // 4. Let args be the list-concatenation of boundArgs and argumentsList.
            let mut all = b.bound_args.clone();
            all.extend_from_slice(args);
            // 5. Return ? Call(target, boundThis, args).
            begin_inline_call(agent, target, bound_this, &all)
        }
        ObjectKind::Builtin(_) => {
            let [call, apply] = agent.realm(agent.current_realm()).call_apply;
            if Some(f) == call {
                // §20.2.3.3 Function.prototype.call ( thisArg, ...args )
                // 1. Let func be the this value.
                // 2. If IsCallable(func) is false, throw a TypeError exception.
                let Some(func) = plain(agent, this) else {
                    return Ok(None);
                };
                // 4. Return ? Call(func, thisArg, args).
                let this_arg = args.first().copied().unwrap_or(Value::Undefined);
                begin_inline_call(agent, func, this_arg, args.get(1..).unwrap_or(&[]))
            } else if Some(f) == apply {
                // §20.2.3.1 Function.prototype.apply ( thisArg, argArray )
                // 1. Let func be the this value.
                // 2. If IsCallable(func) is false, throw a TypeError exception.
                let Some(func) = plain(agent, this) else {
                    return Ok(None);
                };
                let this_arg = args.first().copied().unwrap_or(Value::Undefined);
                let arg_array = args.get(1).copied().unwrap_or(Value::Undefined);
                // 3. If argArray is either undefined or null, then
                if arg_array.is_nullish() {
                    // b. Return ? Call(func, thisArg).
                    return begin_inline_call(agent, func, this_arg, &[]);
                }
                // 4. Let argList be ? CreateListFromArrayLike(argArray).
                let Some(arg_list) = own_data_elements(agent, arg_array) else {
                    return Ok(None);
                };
                // 6. Return ? Call(func, thisArg, argList).
                begin_inline_call(agent, func, this_arg, &arg_list)
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}

/// CreateListFromArrayLike(obj) for an array or unmapped arguments object
/// whose `length` and elements are all own data properties, read without
/// running any code; `None` for any other value.
fn own_data_elements(agent: &Agent, v: Value) -> Option<Vec<Value>> {
    let Value::Object(o) = v else {
        return None;
    };
    let data = agent.obj(o);
    if !matches!(data.kind, ObjectKind::Array | ObjectKind::Arguments(None)) {
        return None;
    }
    // 3. Let len be ? LengthOfArrayLike(obj).
    let len = match data
        .properties
        .get(&PropertyKey::String(agent.names.length))?
    {
        Property::Data {
            value: Value::Number(n),
            ..
        } if *n >= 0.0 && *n <= u32::MAX as f64 && n.fract() == 0.0 => *n as u32,
        _ => return None,
    };
    // 5-6. For each index < len, append ? Get(obj, ! ToString(𝔽(index))).
    (0..len)
        .map(|i| match data.properties.get(&PropertyKey::Index(i)) {
            Some(Property::Data { value, .. }) => Some(*value),
            _ => None,
        })
        .collect()
}

/// [[Call]] of `f` (§10.2.1) up to running its body, as an [`InlineCall`],
/// when `f` is an ECMAScript function whose body runs to completion in one
/// frame (not async, a generator or a class constructor), directly or
/// through `begin_indirect_call`; `None` for any other callee, which the
/// caller calls the usual way.
pub(crate) fn begin_inline_call(
    agent: &mut Agent,
    f: Object,
    this: Value,
    args: &[Value],
) -> JsResult<Option<InlineCall>> {
    let ObjectKind::Function(e) = &agent.obj(f).kind else {
        return begin_indirect_call(agent, f, this, args);
    };
    if e.is_class_constructor {
        return Ok(None);
    }
    let code = e.code.clone();
    let code = if code.lazy.is_some() {
        compile_lazy(agent, f, &code)?
    } else {
        code
    };
    if code.is_async || code.is_generator {
        return Ok(None);
    }
    crate::function::enter_inline_call(agent)?;
    // 2. Let calleeContext be PrepareForOrdinaryCall(F, undefined).
    let env = match prepare_for_ordinary_call(agent, f, Value::Undefined) {
        Ok(env) => env,
        Err(e) => {
            agent.call_depth -= 1;
            return Err(e);
        }
    };
    let mut frame = inline_frame(agent, f, &code, env);
    frame.args.extend_from_slice(args);
    // 5. Perform OrdinaryCallBindThis(F, calleeContext, thisArgument).
    // (Then OrdinaryCallEvaluateBody's FunctionDeclarationInstantiation up
    // to the parameter bindings.)
    let r = ordinary_call_bind_this(agent, f, env, this)
        .and_then(|()| instantiate::function_declaration_instantiation(agent, &mut frame, f, args));
    match r {
        Ok(()) => Ok(Some(InlineCall {
            frame,
            code,
            construct: false,
            env,
        })),
        Err(e) => {
            // 7. Remove calleeContext from the execution context stack.
            abandon_inline_call(agent, f, env);
            Err(e)
        }
    }
}

/// Ends an [`InlineCall`] whose frame has returned `value` (or thrown,
/// for which the value is unused): what `call_ecma_function` and
/// `construct_ecma_function` do after the body. The call's result.
pub(crate) fn finish_inline_call(agent: &mut Agent, call: InlineCall, value: Value) -> Value {
    let InlineCall {
        mut frame,
        env,
        construct,
        ..
    } = call;
    let f = frame.function.expect("function code");
    // [[Construct]] 10. If result is a return completion, then
    // a. If result.[[Value]] is an Object, return result.[[Value]].
    // b. If kind is base, return thisArgument.
    let value = match value {
        Value::Object(_) => value,
        _ if construct => frame.completion,
        _ => value,
    };
    if frame.code.envs_die_with_call {
        free_scopes_between(agent, frame.env, env);
    }
    // 7. Remove calleeContext from the execution context stack.
    agent.contexts.pop();
    if frame.code.envs_die_with_call && own_function_env(agent, f, env) {
        agent.free_env(env);
    }
    agent.call_depth -= 1;
    if agent.spare_frames.len() < 256 && frame.empty_for_pool() {
        agent.spare_frames.push(frame);
    }
    value
}

/// [[Construct]] of an ECMAScript function object (§10.2.2).
pub fn construct_ecma_function(
    agent: &mut Agent,
    f: Object,
    args: &[Value],
    new_target: Object,
) -> JsResult<Object> {
    // 2. Let kind be F.[[ConstructorKind]].
    let kind = ecma(agent, f).constructor_kind;
    // 3. If kind is base, then let thisArgument be ? OrdinaryCreateFromConstructor(newTarget, "%Object.prototype%").
    let this_argument = if kind == ConstructorKind::Base {
        Some(ops::ordinary_create_from_constructor(
            agent,
            new_target,
            "%Object.prototype%",
            ObjectKind::Ordinary,
        )?)
    } else {
        None
    };
    // 4. Let calleeContext be PrepareForOrdinaryCall(F, newTarget).
    let env = prepare_for_ordinary_call(agent, f, Value::Object(new_target))?;
    // (thisArgument is rooted for the call: code that skips its function
    // environment has no this binding to hold it.)
    let roots = this_argument.map(Value::Object);
    let result = agent.with_roots(roots.as_slice(), |agent| {
        construct_in_env(agent, f, args, this_argument, env)
    });
    if ecma(agent, f).code.envs_die_with_call && own_function_env(agent, f, env) {
        agent.free_env(env);
    }
    result
}

/// [[Construct]] steps 5-12, in the callee's function environment `env`.
fn construct_in_env(
    agent: &mut Agent,
    f: Object,
    args: &[Value],
    this_argument: Option<Object>,
    env: EnvId,
) -> JsResult<Object> {
    let result = (|| {
        // 6. If kind is base, then
        if let Some(this) = this_argument {
            // a. Perform OrdinaryCallBindThis(F, calleeContext, thisArgument).
            ordinary_call_bind_this(agent, f, env, Value::Object(this))?;
            // b. Let initializeResult be Completion(InitializeInstanceElements(thisArgument, F)).
            initialize_instance_elements(agent, this, f)?;
        }
        // 8. Let result be Completion(OrdinaryCallEvaluateBody(F, argumentsList)).
        evaluate_body(agent, f, env, args)
    })();
    // 9. Remove calleeContext from the execution context stack.
    agent.contexts.pop();
    let FrameResult::Return(value) = result? else {
        unreachable!("constructor frames return")
    };
    // 10. If result is a return completion, then
    // a. If result.[[Value]] is an Object, return result.[[Value]].
    if let Value::Object(o) = value {
        return Ok(o);
    }
    // b. If kind is base, return thisArgument.
    if let Some(this) = this_argument {
        return Ok(this);
    }
    // c. If result.[[Value]] is not undefined, throw a TypeError exception.
    if !value.is_undefined() {
        return Err(
            agent.throw_type_error("Derived constructors may only return object or undefined")
        );
    }
    // 12. Let thisBinding be ? constructorEnv.GetThisBinding().
    match env.get_this_binding(agent)? {
        Value::Object(o) => Ok(o),
        _ => Err(agent.throw_type_error("constructor did not initialize this")),
    }
}

/// SuperCall evaluation after the arguments (§13.3.7.1 steps 5-11).
pub(super) fn super_call(
    agent: &mut Agent,
    env: EnvId,
    func: Value,
    args: &[Value],
) -> JsResult<Value> {
    let this_env = agent.get_this_environment(env);
    let (new_target, active) = match this_env.function_data(agent) {
        Some(d) => (d.new_target, d.function_object),
        None => return Err(agent.throw_syntax_error("'super' keyword unexpected here")),
    };
    // 5. If IsConstructor(func) is false, throw a TypeError exception.
    let Value::Object(func) = func else {
        return Err(agent.throw_type_error("Super constructor is not a constructor"));
    };
    if !func.is_constructor(agent) {
        return Err(agent.throw_type_error("Super constructor is not a constructor"));
    }
    let Value::Object(new_target) = new_target else {
        return Err(agent.throw_syntax_error("'super' keyword unexpected here"));
    };
    // 6. Let result be ? Construct(func, argList, newTarget).
    // (The caller's frame roots func and the arguments; see docs/GC.md.)
    let result = func.internal_construct_rooted(agent, args, new_target)?;
    // 8. Perform ? thisER.BindThisValue(result).
    this_env.bind_this_value(agent, Value::Object(result))?;
    // 10. Perform ? InitializeInstanceElements(result, F).
    initialize_instance_elements(agent, result, active)?;
    Ok(Value::Object(result))
}

// ---- private elements (§7.3.27-32) ------------------------------------------------

fn private_element_find(agent: &Agent, o: Object, p: PrivateName) -> Option<usize> {
    agent
        .obj(o)
        .private_elements
        .iter()
        .position(|e| e.key == p)
}

fn private_name_error(agent: &mut Agent, p: PrivateName, what: &str) -> crate::error::JsError {
    let msg = format!("{} {what}", agent.str_lossy(p.description));
    agent.throw_type_error(&msg)
}

/// PrivateFieldAdd(O, P, value)
pub fn private_field_add(
    agent: &mut Agent,
    o: Object,
    p: PrivateName,
    value: Value,
) -> JsResult<()> {
    if private_element_find(agent, o, p).is_some() {
        return Err(private_name_error(
            agent,
            p,
            "has already been declared on this object",
        ));
    }
    agent.obj_mut(o).private_elements.push(PrivateElement {
        key: p,
        kind: PrivateElementKind::Field,
        value,
        get: Value::Undefined,
        set: Value::Undefined,
    });
    Ok(())
}

/// PrivateMethodOrAccessorAdd(O, method)
pub fn private_method_or_accessor_add(
    agent: &mut Agent,
    o: Object,
    method: PrivateElement,
) -> JsResult<()> {
    if private_element_find(agent, o, method.key).is_some() {
        return Err(private_name_error(
            agent,
            method.key,
            "has already been declared on this object",
        ));
    }
    agent.obj_mut(o).private_elements.push(method);
    Ok(())
}

/// PrivateGet(O, P)
pub fn private_get(agent: &mut Agent, o: Object, p: PrivateName) -> JsResult<Value> {
    let Some(i) = private_element_find(agent, o, p) else {
        return Err(private_name_error(
            agent,
            p,
            "is not present on this object",
        ));
    };
    let e = agent.obj(o).private_elements[i];
    match e.kind {
        PrivateElementKind::Field | PrivateElementKind::Method => Ok(e.value),
        PrivateElementKind::Accessor => {
            if e.get.is_undefined() {
                return Err(private_name_error(agent, p, "was defined without a getter"));
            }
            ops::call(agent, e.get, Value::Object(o), &[])
        }
    }
}

/// PrivateSet(O, P, value)
pub fn private_set(agent: &mut Agent, o: Object, p: PrivateName, value: Value) -> JsResult<()> {
    let Some(i) = private_element_find(agent, o, p) else {
        return Err(private_name_error(
            agent,
            p,
            "is not present on this object",
        ));
    };
    let e = agent.obj(o).private_elements[i];
    match e.kind {
        PrivateElementKind::Field => {
            agent.obj_mut(o).private_elements[i].value = value;
            Ok(())
        }
        PrivateElementKind::Method => Err(private_name_error(
            agent,
            p,
            "is a method and cannot be assigned",
        )),
        PrivateElementKind::Accessor => {
            if e.set.is_undefined() {
                return Err(private_name_error(agent, p, "was defined without a setter"));
            }
            ops::call(agent, e.set, Value::Object(o), &[value])?;
            Ok(())
        }
    }
}

/// DefineField(receiver, fieldRecord) (§7.3.33)
pub fn define_field(
    agent: &mut Agent,
    receiver: Object,
    field: &ClassFieldDefinition,
) -> JsResult<()> {
    // 3. If initializer is not empty, let initValue be ? Call(initializer, receiver).
    let value = match field.initializer {
        Some(init) => ops::call(agent, Value::Object(init), Value::Object(receiver), &[])?,
        None => Value::Undefined,
    };
    match field.name {
        // 5. If fieldName is a Private Name, perform ? PrivateFieldAdd(receiver, fieldName, initValue).
        FieldName::Private(p) => private_field_add(agent, receiver, p, value),
        // 6. Else, perform ? CreateDataPropertyOrThrow(receiver, fieldName, initValue).
        FieldName::Key(k) => ops::create_data_property_or_throw(agent, receiver, k, value),
    }
}

/// InitializeInstanceElements(O, constructor) (§7.3.34)
pub fn initialize_instance_elements(
    agent: &mut Agent,
    o: Object,
    constructor: Object,
) -> JsResult<()> {
    let (methods, fields) = match &agent.obj(constructor).kind {
        ObjectKind::Function(e) => (e.private_methods.clone(), e.fields.clone()),
        _ => return Ok(()),
    };
    // 2. For each PrivateElement method of methods, perform ? PrivateMethodOrAccessorAdd(O, method).
    for m in methods {
        private_method_or_accessor_add(agent, o, m)?;
    }
    // 4. For each element fieldRecord of fields, perform ? DefineField(O, fieldRecord).
    for f in &fields {
        define_field(agent, o, f)?;
    }
    Ok(())
}

// ---- object literal and class elements -------------------------------------------------

/// MakeMethod plus DefineMethodProperty / the getter and setter cases of
/// MethodDefinitionEvaluation (§15.4.4).
pub(super) fn define_method_property(
    agent: &mut Agent,
    target: Object,
    key: PropertyKey,
    f: Object,
    def: MethodDef,
) -> JsResult<()> {
    // MakeMethod(closure, object).
    ecma_mut(agent, f).home_object = Some(target);
    let desc = match def.kind {
        MethodKind::Method => {
            set_function_name(agent, f, key, None);
            PropertyDescriptor::data(Value::Object(f), true, def.enumerable, true)
        }
        MethodKind::Getter => {
            set_function_name(agent, f, key, Some("get"));
            PropertyDescriptor {
                get: Some(Value::Object(f)),
                enumerable: Some(def.enumerable),
                configurable: Some(true),
                ..Default::default()
            }
        }
        MethodKind::Setter => {
            set_function_name(agent, f, key, Some("set"));
            PropertyDescriptor {
                set: Some(Value::Object(f)),
                enumerable: Some(def.enumerable),
                configurable: Some(true),
                ..Default::default()
            }
        }
    };
    ops::define_property_or_throw(agent, target, key, desc)
}

/// ClassDefinitionEvaluation steps 8.a-h: the heritage. Returns
/// (protoParent, constructorParent).
pub(super) fn class_heritage(
    agent: &mut Agent,
    superclass: Value,
) -> JsResult<(Option<Object>, Object)> {
    match superclass {
        // g. If superclass is null, then protoParent = null, constructorParent = %Function.prototype%.
        Value::Null => Ok((None, agent.intrinsic_c("%Function.prototype%"))),
        Value::Object(s) if s.is_constructor(agent) => {
            // i. Let protoParent be ? Get(superclass, "prototype").
            let key = PropertyKey::String(agent.names.prototype);
            let proto_parent = ops::get(agent, s, key)?;
            // ii. If protoParent is not an Object and not null, throw a TypeError exception.
            match proto_parent {
                Value::Object(p) => Ok((Some(p), s)),
                Value::Null => Ok((None, s)),
                _ => Err(agent.throw_type_error(
                    "Class extends value does not have valid prototype property",
                )),
            }
        }
        // h. Else if IsConstructor(superclass) is false, throw a TypeError exception.
        _ => Err(agent.throw_type_error("Class extends value is not a constructor or null")),
    }
}

/// ClassDefinitionEvaluation steps 9-20: the prototype and constructor.
pub(super) fn create_class(
    agent: &mut Agent,
    frame: &Frame,
    ctor_code: Arc<FunctionCode>,
    class: &ClassStatic,
    name: Value,
    proto_parent: Value,
    ctor_parent: Value,
) -> JsResult<(Object, Object)> {
    // 9. Let proto be OrdinaryObjectCreate(protoParent).
    let proto = agent.ordinary_object_create(proto_parent.as_object());
    let Value::Object(ctor_parent) = ctor_parent else {
        unreachable!("constructor parent")
    };
    let derived = ctor_code.flavor == FunctionFlavor::DerivedConstructor;
    // 14-15. F is the constructor's closure: OrdinaryFunctionCreate(constructorParent, ...), MakeMethod(F, proto).
    let f = ordinary_function_create(
        agent,
        ctor_parent,
        ctor_code,
        frame.env,
        frame.private_env.clone(),
    );
    {
        let e = ecma_mut(agent, f);
        e.home_object = Some(proto);
        // 16. Perform MakeClassConstructor(F).
        e.is_class_constructor = true;
        e.this_mode = ThisMode::Strict;
        // 19. If ClassHeritage is present, set F.[[ConstructorKind]] to derived.
        if derived {
            e.constructor_kind = ConstructorKind::Derived;
        }
        e.class_pending = Some(Box::default());
    }
    let source = frame.code.source.clone();
    ecma_mut(agent, f).source_override = Some((source, class.span.0, class.span.1));
    // 17. Perform SetFunctionName(F, className).
    let key = ops::to_property_key(agent, name)?;
    set_function_name(agent, f, key, None);
    // 18. Perform MakeConstructor(F, false, proto).
    make_constructor(agent, f, false, Some(proto));
    // 20. Perform CreateMethodProperty(proto, "constructor", F).
    let key = PropertyKey::String(agent.names.constructor);
    agent.obj_mut(proto).properties.insert(
        key,
        Property::Data {
            value: Value::Object(f),
            writable: true,
            enumerable: false,
            configurable: true,
        },
    );
    Ok((f, proto))
}

/// ClassFieldDefinitionEvaluation / ClassStaticBlockDefinitionEvaluation:
/// records a field on the class. Stack: [F proto key init] or [F proto block].
pub(super) fn define_class_field(
    agent: &mut Agent,
    frame: &mut Frame,
    def: FieldDef,
) -> JsResult<()> {
    let init = frame.pop();
    let key = if def.static_block {
        None
    } else {
        Some(frame.pop())
    };
    let n = frame.stack.len();
    let (Value::Object(f), Value::Object(proto)) = (frame.stack[n - 2], frame.stack[n - 1]) else {
        unreachable!("class under construction")
    };
    let home = if def.is_static { f } else { proto };
    let name = match key {
        None => FieldName::Key(PropertyKey::String(agent.names.empty)),
        Some(Value::String(s)) if def.private => {
            FieldName::Private(super::resolve_private(agent, frame, s)?)
        }
        Some(k) => FieldName::Key(ops::to_property_key(agent, k)?),
    };
    let initializer = init.as_object();
    if let Some(i) = initializer {
        let e = ecma_mut(agent, i);
        e.home_object = Some(home);
        e.class_field_initializer_name = Some(name);
    }
    let field = ClassFieldDefinition {
        name,
        initializer,
        is_static_block: def.static_block,
    };
    let e = ecma_mut(agent, f);
    if def.is_static {
        e.class_pending
            .get_or_insert_with(Box::default)
            .static_elements
            .push(field);
    } else {
        e.fields.push(field);
    }
    Ok(())
}

/// A private method or accessor (ClassElementEvaluation for private
/// MethodDefinitions). Stack: [F proto closure] -> [F proto].
pub(super) fn define_private_method(
    agent: &mut Agent,
    frame: &mut Frame,
    def: PrivateMethodDef,
) -> JsResult<()> {
    let Value::Object(closure) = frame.pop() else {
        unreachable!("method closure")
    };
    let n = frame.stack.len();
    let (Value::Object(f), Value::Object(proto)) = (frame.stack[n - 2], frame.stack[n - 1]) else {
        unreachable!("class under construction")
    };
    let p = super::resolve_private(agent, frame, def.name)?;
    ecma_mut(agent, closure).home_object = Some(if def.is_static { f } else { proto });
    let name = PropertyKey::String(def.name);
    let element = match def.kind {
        MethodKind::Method => {
            set_function_name(agent, closure, name, None);
            PrivateElement {
                key: p,
                kind: PrivateElementKind::Method,
                value: Value::Object(closure),
                get: Value::Undefined,
                set: Value::Undefined,
            }
        }
        MethodKind::Getter => {
            set_function_name(agent, closure, name, Some("get"));
            PrivateElement {
                key: p,
                kind: PrivateElementKind::Accessor,
                value: Value::Undefined,
                get: Value::Object(closure),
                set: Value::Undefined,
            }
        }
        MethodKind::Setter => {
            set_function_name(agent, closure, name, Some("set"));
            PrivateElement {
                key: p,
                kind: PrivateElementKind::Accessor,
                value: Value::Undefined,
                get: Value::Undefined,
                set: Value::Object(closure),
            }
        }
    };
    let e = ecma_mut(agent, f);
    let list = if def.is_static {
        &mut e
            .class_pending
            .get_or_insert_with(Box::default)
            .static_private_methods
    } else {
        &mut e.private_methods
    };
    // A getter and setter with the same name combine into one accessor.
    match list
        .iter_mut()
        .find(|m| m.key == p && m.kind == PrivateElementKind::Accessor)
    {
        Some(existing) if element.kind == PrivateElementKind::Accessor => {
            if !element.get.is_undefined() {
                existing.get = element.get;
            }
            if !element.set.is_undefined() {
                existing.set = element.set;
            }
        }
        _ => list.push(element),
    }
    Ok(())
}

/// ClassDefinitionEvaluation steps 31-34: static private methods, then
/// static fields and blocks in order.
pub(super) fn finish_class(agent: &mut Agent, f: Object) -> JsResult<()> {
    let pending: ClassPending = ecma_mut(agent, f)
        .class_pending
        .take()
        .map(|b| *b)
        .unwrap_or_default();
    for m in pending.static_private_methods {
        private_method_or_accessor_add(agent, f, m)?;
    }
    for element in &pending.static_elements {
        if element.is_static_block {
            if let Some(init) = element.initializer {
                ops::call(agent, Value::Object(init), Value::Object(f), &[])?;
            }
        } else {
            define_field(agent, f, element)?;
        }
    }
    Ok(())
}

/// GetTemplateObject(templateLiteral) (§13.2.8.4)
pub(super) fn get_template_object(
    agent: &mut Agent,
    code: &FunctionCode,
    t: &TemplateInfo,
) -> JsResult<Object> {
    let realm = agent.current_realm();
    let site = (code.script, t.site);
    // 3. If templateRegistry contains an entry for this site, return it.
    if let Some(&o) = agent.realm(realm).template_map.get(&site) {
        return Ok(o);
    }
    let count = t.raw.len();
    // 7-8. template and rawObj are arrays of length count.
    let template = crate::object::array::array_create(agent, count as f64, None)?;
    let raw = crate::object::array::array_create(agent, count as f64, None)?;
    for i in 0..count {
        let key = PropertyKey::Index(i as u32);
        let cooked = t.cooked[i].map(Value::String).unwrap_or(Value::Undefined);
        ops::define_property_or_throw(
            agent,
            template,
            key,
            PropertyDescriptor::data(cooked, false, true, false),
        )?;
        ops::define_property_or_throw(
            agent,
            raw,
            key,
            PropertyDescriptor::data(Value::String(t.raw[i]), false, true, false),
        )?;
    }
    // 11. Perform ! SetIntegrityLevel(rawObj, frozen).
    ops::set_integrity_level(agent, raw, ops::IntegrityLevel::Frozen)?;
    // 12. Perform ! DefinePropertyOrThrow(template, "raw", ...).
    let raw_key = agent.key("raw");
    ops::define_property_or_throw(
        agent,
        template,
        raw_key,
        PropertyDescriptor::data(Value::Object(raw), false, false, false),
    )?;
    // 13. Perform ! SetIntegrityLevel(template, frozen).
    ops::set_integrity_level(agent, template, ops::IntegrityLevel::Frozen)?;
    agent.realm_mut(realm).template_map.insert(site, template);
    Ok(template)
}
