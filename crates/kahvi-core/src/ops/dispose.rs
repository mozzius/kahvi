//! Operations on Disposable Objects: DisposableResource Records,
//! AddDisposableResource, GetDisposeMethod and DisposeResources.
//!
//! DisposeResources may Await, so it is a resumable state machine:
//! [`DisposeResources::step`] runs up to the next Await and hands back the
//! value to await; whoever drives it (the VM's dispose ops for `using`
//! blocks, a promise reaction for AsyncDisposableStack.prototype.disposeAsync)
//! awaits it and calls `step` again, reporting a rejection with
//! [`DisposeResources::await_threw`].

use crate::agent::Agent;
use crate::error::{JsError, JsResult};
use crate::function::{builtin_slots, create_builtin_function};
use crate::object::{ObjectKind, PropertyKey};
use crate::ops;
use crate::value::{Object, Value};

/// [[Kind]] of a DisposableResource Record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisposeKind {
    /// ~sync-dispose~: a `using` declaration or a DisposableStack.
    Sync,
    /// ~async-dispose~: an `await using` declaration or an AsyncDisposableStack.
    Async,
}

/// A DisposableResource Record.
#[derive(Debug, Clone, Copy)]
pub struct DisposableResource {
    /// [[ResourceValue]]: an Object or undefined.
    pub value: Value,
    /// [[Kind]]
    pub kind: DisposeKind,
    /// [[DisposeMethod]]: a function object or undefined.
    pub method: Value,
}

/// AddDisposableResource(disposableResourceStack, value, kind [, method])
pub fn add_disposable_resource(
    agent: &mut Agent,
    stack: &mut Vec<DisposableResource>,
    value: Value,
    kind: DisposeKind,
    method: Option<Value>,
) -> JsResult<()> {
    let resource = match method {
        // 1. If method is present, then
        Some(method) => {
            // a. Assert: value is undefined.
            debug_assert!(matches!(value, Value::Undefined));
            // b. Let resource be ? CreateDisposableResource(undefined, kind, method).
            create_disposable_resource(agent, Value::Undefined, kind, Some(method))?
        }
        // 2. Else,
        None => {
            // a. If value is either null or undefined and kind is sync-dispose, return unused.
            if value.is_nullish() && kind == DisposeKind::Sync {
                return Ok(());
            }
            // b. NOTE: For async-dispose a null or undefined value is recorded,
            //    so disposal still performs an Await.
            // c. Let resource be ? CreateDisposableResource(value, kind).
            create_disposable_resource(agent, value, kind, None)?
        }
    };
    // 3. Append resource to disposableResourceStack.
    stack.push(resource);
    // 4. Return unused.
    Ok(())
}

/// CreateDisposableResource(value, kind [, method])
pub fn create_disposable_resource(
    agent: &mut Agent,
    value: Value,
    kind: DisposeKind,
    method: Option<Value>,
) -> JsResult<DisposableResource> {
    let (value, method) = match method {
        Some(method) => (value, method),
        // 1. If method is not present, then
        None => {
            if value.is_nullish() {
                // a. If value is either null or undefined, then
                //    i. Set value to undefined. ii. Set method to undefined.
                (Value::Undefined, Value::Undefined)
            } else {
                // b. Else,
                //    i. Set method to ? GetDisposeMethod(value, kind).
                let method = get_dispose_method(agent, value, kind)?;
                //    ii. If method is undefined, throw a TypeError exception.
                if matches!(method, Value::Undefined) {
                    return Err(agent.throw_type_error(match kind {
                        DisposeKind::Sync => "the value has no [Symbol.dispose] method",
                        DisposeKind::Async => {
                            "the value has no [Symbol.asyncDispose] or [Symbol.dispose] method"
                        }
                    }));
                }
                (value, method)
            }
        }
    };
    // 2. Return the DisposableResource Record { [[ResourceValue]]: value, [[Kind]]: kind, [[DisposeMethod]]: method }.
    Ok(DisposableResource {
        value,
        kind,
        method,
    })
}

/// GetDisposeMethod(value, kind)
pub fn get_dispose_method(agent: &mut Agent, value: Value, kind: DisposeKind) -> JsResult<Value> {
    // 1. If value is not an Object, throw a TypeError exception.
    if !matches!(value, Value::Object(_)) {
        return Err(agent.throw_type_error("a disposable resource must be an object"));
    }
    let dispose = PropertyKey::Symbol(agent.symbols_wk.dispose);
    // 2. If kind is sync-dispose, return ? GetMethod(value, %Symbol.dispose%).
    if kind == DisposeKind::Sync {
        return ops::get_method(agent, value, dispose);
    }
    // 3. Assert: kind is async-dispose.
    // 4. Let asyncMethod be ? GetMethod(value, %Symbol.asyncDispose%).
    let async_dispose = PropertyKey::Symbol(agent.symbols_wk.async_dispose);
    let async_method = ops::get_method(agent, value, async_dispose)?;
    // 5. If asyncMethod is not undefined, return asyncMethod.
    if !matches!(async_method, Value::Undefined) {
        return Ok(async_method);
    }
    // 6. Let syncMethod be ? GetMethod(value, %Symbol.dispose%).
    let sync_method = ops::get_method(agent, value, dispose)?;
    // 7. If syncMethod is undefined, return undefined.
    if matches!(sync_method, Value::Undefined) {
        return Ok(Value::Undefined);
    }
    // 8-10. Return CreateBuiltinFunction(closure, 0, "", « ») where closure
    //       captures syncMethod (see `sync_dispose_as_async`).
    let empty = agent.string("");
    Ok(Value::Object(create_builtin_function(
        agent,
        sync_dispose_as_async,
        0,
        PropertyKey::String(empty),
        vec![sync_method],
        None,
        None,
        None,
        false,
    )))
}

/// The closure of GetDisposeMethod step 8: calls the captured syncMethod
/// and turns its outcome into a promise (a synchronous throw becomes a
/// rejection; a returned promise is not awaited).
fn sync_dispose_as_async(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let f = agent.active_function().expect("a running built-in");
    let sync_method = builtin_slots(agent, f)[0];
    // a. Let obj be the this value.
    // b. Let promiseCapability be ! NewPromiseCapability(%Promise%).
    let promise_ctor = Value::Object(agent.intrinsic_c("%Promise%"));
    let capability = ops::promise::new_promise_capability(agent, promise_ctor)?;
    // c. Let result be Completion(Call(syncMethod, obj)).
    match ops::call(agent, sync_method, this, &[]) {
        // d. IfAbruptRejectPromise(result, promiseCapability).
        Err(JsError::Throw(e)) => {
            ops::call(agent, capability.reject, Value::Undefined, &[e])?;
            return Ok(capability.promise);
        }
        Err(e) => return Err(e),
        Ok(_) => {}
    }
    // e. Perform ! Call(promiseCapability.[[Resolve]], undefined, « undefined »).
    ops::call(
        agent,
        capability.resolve,
        Value::Undefined,
        &[Value::Undefined],
    )?;
    // f. Return promiseCapability.[[Promise]].
    Ok(capability.promise)
}

/// What DisposeResources needs next.
#[derive(Debug, Clone, Copy)]
pub enum DisposeStep {
    /// Await(value), then call [`DisposeResources::step`] again (after
    /// [`DisposeResources::await_threw`] if the await threw).
    Await(Value),
    /// All resources are disposed: the result is [`DisposeResources::output`].
    Done,
}

/// DisposeResources(disposableResourceStack, completion), suspended
/// at its Awaits.
#[derive(Debug)]
pub struct DisposeResources {
    stack: Vec<DisposableResource>,
    /// outputCompletion: `Ok` for a normal completion (or any non-throw
    /// abrupt completion the caller carries alongside), `Err` for a throw.
    output: Result<Value, Value>,
    needs_await: bool,
    has_awaited: bool,
}

impl DisposeResources {
    /// Steps 1-3.
    pub fn new(stack: Vec<DisposableResource>, completion: Result<Value, Value>) -> Self {
        DisposeResources {
            stack,
            // 3. Let outputCompletion be completion.
            output: completion,
            // 1. Let needsAwait be false.
            needs_await: false,
            // 2. Let hasAwaited be false.
            has_awaited: false,
        }
    }

    /// Runs the remaining steps up to the next Await or the end.
    /// Only a not-implemented error escapes; throws are folded into the output.
    pub fn step(&mut self, agent: &mut Agent) -> JsResult<DisposeStep> {
        // 4. For each element resource of disposableResourceStack, in reverse List order, do
        while let Some(&resource) = self.stack.last() {
            // a-c. Let value, kind and method be resource's fields.
            // d. If kind is sync-dispose, needsAwait is true, and hasAwaited is false, then
            if resource.kind == DisposeKind::Sync && self.needs_await && !self.has_awaited {
                // ii. Set needsAwait to false.
                self.needs_await = false;
                // i. Perform ! Await(undefined). (The resource is processed on resumption.)
                return Ok(DisposeStep::Await(Value::Undefined));
            }
            self.stack.pop();
            // e. If method is not undefined, then
            if !matches!(resource.method, Value::Undefined) {
                // i. Let result be Completion(Call(method, value)).
                match ops::call(agent, resource.method, resource.value, &[]) {
                    // ii. If result is a normal completion and kind is async-dispose, then
                    Ok(v) if resource.kind == DisposeKind::Async => {
                        // 2. Set hasAwaited to true.
                        self.has_awaited = true;
                        // 1. Set result to Completion(Await(result.[[Value]])).
                        return Ok(DisposeStep::Await(v));
                    }
                    Ok(_) => {}
                    // iii. If result is a throw completion, then ...
                    Err(JsError::Throw(e)) => self.record_throw(agent, e)?,
                    Err(e) => return Err(e),
                }
            } else {
                // f. Else,
                //    i. Assert: kind is async-dispose.
                debug_assert_eq!(resource.kind, DisposeKind::Async);
                //    ii. Set needsAwait to true.
                self.needs_await = true;
            }
        }
        // 5. If needsAwait is true and hasAwaited is false, then
        if self.needs_await && !self.has_awaited {
            self.needs_await = false;
            // a. Perform ! Await(undefined).
            return Ok(DisposeStep::Await(Value::Undefined));
        }
        Ok(DisposeStep::Done)
    }

    /// The Await of step 4.e.ii.1 threw `e`: step 4.e.iii with that throw completion.
    pub fn await_threw(&mut self, agent: &mut Agent, e: Value) -> JsResult<()> {
        self.record_throw(agent, e)
    }

    /// Step 4.e.iii: result is a throw completion carrying `result`.
    fn record_throw(&mut self, agent: &mut Agent, result: Value) -> JsResult<()> {
        match self.output {
            // 1. If outputCompletion is a throw completion, then
            Err(suppressed) => {
                // c. Let error be a newly created SuppressedError object.
                let proto = agent.intrinsic_c("%SuppressedError.prototype%");
                let error = agent.object_with_kind(Some(proto), ObjectKind::Error);
                // d. Perform CreateNonEnumerableDataPropertyOrThrow(error, "error", result).
                let key = agent.key("error");
                ops::create_non_enumerable_data_property_or_throw(agent, error, key, result)?;
                // e. Perform CreateNonEnumerableDataPropertyOrThrow(error, "suppressed", suppressed).
                let key = agent.key("suppressed");
                ops::create_non_enumerable_data_property_or_throw(agent, error, key, suppressed)?;
                // f. Set outputCompletion to ThrowCompletion(error).
                self.output = Err(Value::Object(error));
            }
            // 2. Else, a. Set outputCompletion to result.
            Ok(_) => self.output = Err(result),
        }
        Ok(())
    }

    /// 7. Return ? outputCompletion.
    pub fn output(&self) -> Result<Value, Value> {
        self.output
    }
}

// ---- garbage collection ----------------------------------------------------

use crate::gc::{Trace, Tracer};

impl Trace for DisposableResource {
    fn trace(&self, t: &mut Tracer) {
        t.value(self.value);
        t.value(self.method);
    }
}

impl Trace for DisposeResources {
    fn trace(&self, t: &mut Tracer) {
        self.stack.trace(t);
        match self.output {
            Ok(v) | Err(v) => t.value(v),
        }
    }
}
