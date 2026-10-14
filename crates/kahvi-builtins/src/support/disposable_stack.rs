//! DisposableStack and AsyncDisposableStack internal slots, and running
//! DisposeResources from a built-in (AsyncDisposableStack.prototype.disposeAsync
//! is an async method: its Awaits are promise reactions).
//!
//! The abstract operations themselves (AddDisposableResource,
//! DisposeResources, ...) are in `kahvi_core::ops::dispose`.
#![allow(dead_code)] // helpers for the DisposableStack tickets

use kahvi_core::function::{builtin_slots, create_builtin_function};
use kahvi_core::ops::dispose::{DisposableResource, DisposeResources, DisposeStep};
use kahvi_core::ops::promise::PromiseCapability;
use kahvi_core::prelude::*;

/// [[DisposableState]] and [[DisposableResourceStack]] of a DisposableStack.
#[derive(Debug, Default)]
pub(crate) struct DisposableStackData {
    /// [[DisposableState]] is ~disposed~ (otherwise ~pending~).
    pub disposed: bool,
    /// [[DisposableResourceStack]]
    pub stack: Vec<DisposableResource>,
}

/// [[AsyncDisposableState]] and [[DisposableResourceStack]] of an AsyncDisposableStack.
#[derive(Debug, Default)]
pub(crate) struct AsyncDisposableStackData {
    /// [[AsyncDisposableState]] is ~disposed~ (otherwise ~pending~).
    pub disposed: bool,
    /// [[DisposableResourceStack]]
    pub stack: Vec<DisposableResource>,
}

/// RequireInternalSlot(O, [[DisposableState]]): O as a DisposableStack.
pub(crate) fn require_disposable_stack(agent: &mut Agent, o: Value) -> JsResult<Object> {
    match o {
        Value::Object(o) if agent.native::<DisposableStackData>(o).is_some() => Ok(o),
        _ => Err(agent.throw_type_error("not a DisposableStack")),
    }
}

/// RequireInternalSlot(O, [[AsyncDisposableState]]): O as an AsyncDisposableStack.
pub(crate) fn require_async_disposable_stack(agent: &mut Agent, o: Value) -> JsResult<Object> {
    match o {
        Value::Object(o) if agent.native::<AsyncDisposableStackData>(o).is_some() => Ok(o),
        _ => Err(agent.throw_type_error("not an AsyncDisposableStack")),
    }
}

/// The slots of a DisposableStack (after `require_disposable_stack`).
pub(crate) fn disposable_stack(agent: &mut Agent, o: Object) -> &mut DisposableStackData {
    agent
        .native_mut::<DisposableStackData>(o)
        .expect("a DisposableStack")
}

/// The slots of an AsyncDisposableStack (after `require_async_disposable_stack`).
pub(crate) fn async_disposable_stack(
    agent: &mut Agent,
    o: Object,
) -> &mut AsyncDisposableStackData {
    agent
        .native_mut::<AsyncDisposableStackData>(o)
        .expect("an AsyncDisposableStack")
}

/// ? AddDisposableResource(stack.[[DisposableResourceStack]], value, kind [, method])
/// for the DisposableStack or AsyncDisposableStack `o`.
pub(crate) fn add_disposable_resource(
    agent: &mut Agent,
    o: Object,
    value: Value,
    kind: kahvi_core::ops::dispose::DisposeKind,
    method: Option<Value>,
) -> JsResult<()> {
    let is_async = agent.native::<AsyncDisposableStackData>(o).is_some();
    let mut stack = if is_async {
        std::mem::take(&mut async_disposable_stack(agent, o).stack)
    } else {
        std::mem::take(&mut disposable_stack(agent, o).stack)
    };
    let result =
        kahvi_core::ops::dispose::add_disposable_resource(agent, &mut stack, value, kind, method);
    if is_async {
        async_disposable_stack(agent, o).stack = stack;
    } else {
        disposable_stack(agent, o).stack = stack;
    }
    result
}

/// DisposeResources run to completion by a built-in that can await: each
/// Await is a reaction on PromiseResolve(%Promise%, value), and the outcome
/// settles `capability`.
#[derive(Debug)]
struct AsyncDispose {
    dispose: Option<DisposeResources>,
    capability: PromiseCapability,
}

/// Runs `dispose`, then resolves `capability` with undefined or rejects it
/// with the thrown value.
pub(crate) fn dispose_resources_async(
    agent: &mut Agent,
    dispose: DisposeResources,
    capability: PromiseCapability,
) -> JsResult<()> {
    let state = agent.object_with_kind(
        None,
        ObjectKind::Native(Box::new(AsyncDispose {
            dispose: Some(dispose),
            capability,
        })),
    );
    continue_async_dispose(agent, state, None)
}

/// Resumes DisposeResources after an Await (`threw` is the rejection reason
/// of the awaited value, if it was rejected).
fn continue_async_dispose(agent: &mut Agent, state: Object, threw: Option<Value>) -> JsResult<()> {
    let mut threw = threw;
    loop {
        let mut dispose = agent
            .native_mut::<AsyncDispose>(state)
            .and_then(|s| s.dispose.take())
            .expect("an async DisposeResources");
        let step = match threw.take() {
            Some(e) => dispose
                .await_threw(agent, e)
                .and_then(|_| dispose.step(agent)),
            None => dispose.step(agent),
        };
        let output = dispose.output();
        let s = agent
            .native_mut::<AsyncDispose>(state)
            .expect("an async DisposeResources");
        s.dispose = Some(dispose);
        let capability = s.capability;
        match step? {
            DisposeStep::Done => {
                match output {
                    Ok(_) => ops::call(
                        agent,
                        capability.resolve,
                        Value::Undefined,
                        &[Value::Undefined],
                    )?,
                    Err(e) => ops::call(agent, capability.reject, Value::Undefined, &[e])?,
                };
                return Ok(());
            }
            DisposeStep::Await(v) => {
                // Await(value): Let promise be ? PromiseResolve(%Promise%, value).
                let promise_ctor = agent.intrinsic("%Promise%");
                let promise = match ops::promise::promise_resolve(agent, promise_ctor, v) {
                    Ok(p) => p,
                    // The Await completion is that throw.
                    Err(JsError::Throw(e)) => {
                        threw = Some(e);
                        continue;
                    }
                    Err(e) => return Err(e),
                };
                // PerformPromiseThen(promise, onFulfilled, onRejected): resume here.
                let empty = PropertyKey::String(agent.string(""));
                let on_fulfilled = create_builtin_function(
                    agent,
                    async_dispose_fulfilled,
                    1,
                    empty,
                    vec![Value::Object(state)],
                    None,
                    None,
                    None,
                    false,
                );
                let on_rejected = create_builtin_function(
                    agent,
                    async_dispose_rejected,
                    1,
                    empty,
                    vec![Value::Object(state)],
                    None,
                    None,
                    None,
                    false,
                );
                ops::promise::perform_promise_then(
                    agent,
                    promise,
                    Value::Object(on_fulfilled),
                    Value::Object(on_rejected),
                    None,
                );
                return Ok(());
            }
        }
    }
}

fn reaction_state(agent: &Agent) -> Object {
    let f = agent.active_function().expect("a running built-in");
    match builtin_slots(agent, f)[0] {
        Value::Object(state) => state,
        _ => unreachable!("an async DisposeResources reaction without its state"),
    }
}

/// The awaited value fulfilled: DisposeResources continues.
fn async_dispose_fulfilled(
    agent: &mut Agent,
    _this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let state = reaction_state(agent);
    continue_async_dispose(agent, state, None)?;
    Ok(Value::Undefined)
}

/// The awaited value rejected: the Await is a throw completion.
fn async_dispose_rejected(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let state = reaction_state(agent);
    let reason = args.get_or_undefined(0);
    continue_async_dispose(agent, state, Some(reason))?;
    Ok(Value::Undefined)
}
