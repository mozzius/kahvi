//! Async functions (§27.7): AsyncFunctionStart, AsyncBlockStart and Await.
//!
//! An async function's frame runs until an `await`; the frame is then kept
//! in an async context object and resumed by the promise reaction that
//! settles the awaited value.

use super::{Frame, FrameResult, run_frame};
use crate::agent::{Agent, ExecutionContext};
use crate::error::{JsError, JsResult};
use crate::function::{builtin_slots, create_builtin_function};
use crate::object::{ObjectKind, PropertyKey};
use crate::ops::call;
use crate::ops::promise::{
    PromiseCapability, new_promise_capability, perform_promise_then, promise_resolve,
};
use crate::value::{Args, Object, Value};

/// The suspended asyncContext of a running async function.
#[derive(Debug)]
struct AsyncContext {
    frame: Option<Box<Frame>>,
    capability: PromiseCapability,
    /// The async function, or None for a module's top-level await.
    function: Option<Object>,
    realm: crate::realm::RealmId,
    script_or_module: Option<crate::ScriptOrModule>,
}

/// How the async context continues.
enum Resumption {
    Start,
    Normal(Value),
    Throw(Value),
}

/// EvaluateAsyncFunctionBody (§15.8.4) for a frame whose
/// FunctionDeclarationInstantiation has been set up: returns the promise.
pub(super) fn start_async_function(
    agent: &mut Agent,
    f: Object,
    frame: Frame,
    decl_result: JsResult<()>,
) -> JsResult<Value> {
    // 1. Let promiseCapability be ! NewPromiseCapability(%Promise%).
    let promise_ctor = agent.intrinsic("%Promise%");
    let capability = new_promise_capability(agent, Value::Object(promise_ctor))?;
    match decl_result {
        // 3. If declResult is an abrupt completion, then perform ! Call(promiseCapability.[[Reject]], undefined, « declResult.[[Value]] »).
        Err(JsError::Throw(e)) => {
            call(agent, capability.reject, Value::Undefined, &[e])?;
        }
        Err(e) => return Err(e),
        // 4. Else, perform AsyncFunctionStart(promiseCapability, FunctionBody).
        Ok(()) => {
            let proto = agent.intrinsic("%Object.prototype%");
            let (realm, script_or_module) = match &agent.obj(f).kind {
                ObjectKind::Function(e) => (e.realm, e.script_or_module),
                _ => (agent.current_realm(), None),
            };
            let ctx = AsyncContext {
                frame: Some(Box::new(frame)),
                capability,
                function: Some(f),
                realm,
                script_or_module,
            };
            let ctx = agent.object_with_kind(Some(proto), ObjectKind::Native(Box::new(ctx)));
            resume(agent, ctx, Resumption::Start)?;
        }
    }
    // 5. Return Completion Record { [[Type]]: return, [[Value]]: promiseCapability.[[Promise]] }.
    Ok(capability.promise)
}

/// AsyncBlockStart(promiseCapability, module body, moduleContext) for a
/// module with top-level await (ExecuteModule step 3 of §16.2.1.6.5).
pub(super) fn start_async_module(
    agent: &mut Agent,
    module: u32,
    frame: Frame,
    capability: PromiseCapability,
) -> JsResult<()> {
    let proto = agent.intrinsic("%Object.prototype%");
    let realm = agent.modules[module as usize].realm;
    let ctx = AsyncContext {
        frame: Some(Box::new(frame)),
        capability,
        function: None,
        realm,
        script_or_module: Some(crate::ScriptOrModule::Module(module)),
    };
    let ctx = agent.object_with_kind(Some(proto), ObjectKind::Native(Box::new(ctx)));
    resume(agent, ctx, Resumption::Start)
}

/// Runs the async context until it awaits, returns or throws (AsyncBlockStart).
fn resume(agent: &mut Agent, ctx: Object, mut how: Resumption) -> JsResult<()> {
    loop {
        let (mut frame, capability, function, realm, script_or_module) = {
            let c = agent
                .native_mut::<AsyncContext>(ctx)
                .expect("async context");
            (
                c.frame.take().expect("suspended async frame"),
                c.capability,
                c.function,
                c.realm,
                c.script_or_module,
            )
        };
        let result = match how {
            Resumption::Start => None,
            Resumption::Normal(v) => {
                frame.stack.push(v);
                None
            }
            // A throw completion resumes at the await, where handlers may catch it.
            Resumption::Throw(e) => (!frame.handle_throw(e)).then_some(Err(JsError::Throw(e))),
        };
        let result = match result {
            Some(r) => r,
            None => {
                agent.contexts.push(ExecutionContext {
                    function,
                    realm,
                    script_or_module,
                });
                agent.call_depth += 1;
                let r = if agent.call_depth > crate::agent::MAX_CALL_DEPTH {
                    Err(agent.throw_range_error("Maximum call stack size exceeded"))
                } else {
                    run_frame(agent, &mut frame)
                };
                agent.call_depth -= 1;
                agent.contexts.pop();
                r
            }
        };
        match result {
            // Await(value): suspend and continue when the promise settles.
            Ok(FrameResult::Await(v)) => {
                agent
                    .native_mut::<AsyncContext>(ctx)
                    .expect("async context")
                    .frame = Some(frame);
                match await_value(agent, ctx, v) {
                    Ok(()) => return Ok(()),
                    // Await step 2 (? PromiseResolve) threw: the throw happens at the await.
                    Err(JsError::Throw(e)) => {
                        how = Resumption::Throw(e);
                        continue;
                    }
                    Err(e) => return Err(e),
                }
            }
            // AsyncBlockStart 2.e-f: a normal or return completion resolves the promise.
            Ok(FrameResult::Return(v))
            | Ok(FrameResult::Yield(v))
            | Ok(FrameResult::YieldDelegate(v)) => {
                call(agent, capability.resolve, Value::Undefined, &[v])?;
                return Ok(());
            }
            // 2.g Else, a throw completion rejects it.
            Err(JsError::Throw(e)) => {
                call(agent, capability.reject, Value::Undefined, &[e])?;
                return Ok(());
            }
            Err(e) => return Err(e),
        }
    }
}

/// Await(value) (§27.7.5.3) steps 2-9, after the frame has been suspended.
/// `ctx` is the suspended async function context or async generator.
pub(super) fn await_value(agent: &mut Agent, ctx: Object, value: Value) -> JsResult<()> {
    // 2. Let promise be ? PromiseResolve(%Promise%, value).
    let promise_ctor = agent.intrinsic("%Promise%");
    let promise = promise_resolve(agent, promise_ctor, value)?;
    // 3-6. onFulfilled and onRejected resume the async context.
    let empty = PropertyKey::String(agent.names.empty);
    let slots = vec![Value::Object(ctx)];
    let on_fulfilled = create_builtin_function(
        agent,
        await_fulfilled,
        1,
        empty,
        slots.clone(),
        None,
        None,
        None,
        false,
    );
    let on_rejected = create_builtin_function(
        agent,
        await_rejected,
        1,
        empty,
        slots,
        None,
        None,
        None,
        false,
    );
    // 7. Perform PerformPromiseThen(promise, onFulfilled, onRejected).
    perform_promise_then(
        agent,
        promise,
        Value::Object(on_fulfilled),
        Value::Object(on_rejected),
        None,
    );
    Ok(())
}

fn context_of_active(agent: &Agent) -> Option<Object> {
    let f = agent.active_function()?;
    builtin_slots(agent, f).first().and_then(|v| v.as_object())
}

/// Await's fulfilledClosure: resume with a normal completion.
fn await_fulfilled(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _nt: Option<Object>,
) -> JsResult<Value> {
    let Some(ctx) = context_of_active(agent) else {
        return Ok(Value::Undefined);
    };
    let v = args.get_or_undefined(0);
    if super::async_generator::is_async_generator(agent, ctx) {
        super::async_generator::resume(agent, ctx, super::async_generator::Completion::Normal(v))?;
    } else {
        resume(agent, ctx, Resumption::Normal(v))?;
    }
    Ok(Value::Undefined)
}

/// Await's rejectedClosure: resume with a throw completion.
fn await_rejected(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _nt: Option<Object>,
) -> JsResult<Value> {
    let Some(ctx) = context_of_active(agent) else {
        return Ok(Value::Undefined);
    };
    let e = args.get_or_undefined(0);
    if super::async_generator::is_async_generator(agent, ctx) {
        super::async_generator::resume(agent, ctx, super::async_generator::Completion::Throw(e))?;
    } else {
        resume(agent, ctx, Resumption::Throw(e))?;
    }
    Ok(Value::Undefined)
}

impl crate::gc::Trace for AsyncContext {
    fn trace(&self, t: &mut crate::gc::Tracer) {
        self.frame.trace(t);
        self.capability.trace(t);
        self.function.trace(t);
    }
}
