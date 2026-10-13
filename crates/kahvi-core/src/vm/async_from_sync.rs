//! Async-from-sync iterator objects (§27.1.6): adapt a sync iterator for
//! `for await` and `yield*` in async generators.

use crate::agent::Agent;
use crate::error::{JsError, JsResult};
use crate::function::{builtin_slots, create_builtin_function};
use crate::object::{ObjectKind, PropertyKey};
use crate::ops::promise::{
    PromiseCapability, new_promise_capability, perform_promise_then, promise_resolve,
};
use crate::ops::{
    IteratorRecord, call, create_iter_result_object, get, get_method, iterator_close,
    iterator_complete, iterator_next, iterator_value,
};
use crate::value::{Args, Object, Value};

/// [[SyncIteratorRecord]]
#[derive(Debug)]
struct AsyncFromSyncIterator {
    sync: IteratorRecord,
}

/// CreateAsyncFromSyncIterator(syncIteratorRecord) (§27.1.6.1)
pub fn create_async_from_sync_iterator(
    agent: &mut Agent,
    sync: IteratorRecord,
) -> JsResult<IteratorRecord> {
    // 1. Let asyncIterator be OrdinaryObjectCreate(%AsyncFromSyncIteratorPrototype%, « [[SyncIteratorRecord]] »).
    let proto = agent.intrinsic("%AsyncFromSyncIteratorPrototype%");
    // 2. Set asyncIterator.[[SyncIteratorRecord]] to syncIteratorRecord.
    let it = agent.object_with_kind(
        Some(proto),
        ObjectKind::Native(Box::new(AsyncFromSyncIterator { sync })),
    );
    // 3. Let nextMethod be ! Get(asyncIterator, "next").
    let next_key = PropertyKey::String(agent.names.next);
    let next_method = get(agent, it, next_key)?;
    // 4. Return the Iterator Record { [[Iterator]]: asyncIterator, [[NextMethod]]: nextMethod, [[Done]]: false }.
    Ok(IteratorRecord {
        iterator: it,
        next_method,
        done: false,
    })
}

/// The method's capability and the receiver's sync iterator record.
fn begin(agent: &mut Agent, this: Value) -> JsResult<(PromiseCapability, IteratorRecord)> {
    // 3. Let promiseCapability be ! NewPromiseCapability(%Promise%).
    let promise_ctor = agent.intrinsic("%Promise%");
    let capability = new_promise_capability(agent, Value::Object(promise_ctor))?;
    // 2. Assert: O is an Object that has a [[SyncIteratorRecord]] internal slot.
    let sync = match this {
        Value::Object(o) => agent.native::<AsyncFromSyncIterator>(o).map(|d| d.sync),
        _ => None,
    };
    match sync {
        Some(s) => Ok((capability, s)),
        None => Err(agent.throw_type_error("not an async-from-sync iterator")),
    }
}

/// IfAbruptRejectPromise(value, capability): `Ok(Err(promise))` means return the promise.
fn reject_if_abrupt<T>(
    agent: &mut Agent,
    r: JsResult<T>,
    capability: PromiseCapability,
) -> JsResult<Result<T, Value>> {
    match r {
        Ok(v) => Ok(Ok(v)),
        Err(JsError::Throw(e)) => {
            call(agent, capability.reject, Value::Undefined, &[e])?;
            Ok(Err(capability.promise))
        }
        Err(e) => Err(e),
    }
}

fn reject_type_error(
    agent: &mut Agent,
    capability: PromiseCapability,
    message: &str,
) -> JsResult<Value> {
    let err = agent.throw_type_error(message);
    let JsError::Throw(e) = err else {
        return Err(err);
    };
    call(agent, capability.reject, Value::Undefined, &[e])?;
    Ok(capability.promise)
}

/// %AsyncFromSyncIteratorPrototype%.next ( [ value ] ) (§27.1.6.2.1)
pub fn async_from_sync_next(agent: &mut Agent, this: Value, args: &[Value]) -> JsResult<Value> {
    let (capability, mut sync) = begin(agent, this)?;
    // 5. If value is present, let result be Completion(IteratorNext(syncIteratorRecord, value)); else without it.
    let result = iterator_next(agent, &mut sync, args.first().copied());
    // 6. IfAbruptRejectPromise(result, promiseCapability).
    let result = match reject_if_abrupt(agent, result, capability)? {
        Ok(r) => r,
        Err(p) => return Ok(p),
    };
    // 7. Return AsyncFromSyncIteratorContinuation(result, promiseCapability, syncIteratorRecord, true).
    continuation(agent, result, capability, sync, true)
}

/// %AsyncFromSyncIteratorPrototype%.return ( [ value ] ) (§27.1.6.2.2)
pub fn async_from_sync_return(agent: &mut Agent, this: Value, args: &[Value]) -> JsResult<Value> {
    let (capability, sync) = begin(agent, this)?;
    let iterator = Value::Object(sync.iterator);
    // 5. Let return be Completion(GetMethod(syncIterator, "return")).
    let return_key = PropertyKey::String(agent.names.r#return);
    let r = get_method(agent, iterator, return_key);
    let r = match reject_if_abrupt(agent, r, capability)? {
        Ok(r) => r,
        Err(p) => return Ok(p),
    };
    // 7. If return is undefined, then resolve with CreateIteratorResultObject(value, true).
    if r.is_undefined() {
        let result = create_iter_result_object(agent, args.get_or_undefined(0), true);
        call(
            agent,
            capability.resolve,
            Value::Undefined,
            &[Value::Object(result)],
        )?;
        return Ok(capability.promise);
    }
    // 8-9. Let result be Completion(Call(return, syncIterator, « value »)) (without value if absent).
    let result = call(agent, r, iterator, args.get(..1).unwrap_or(&[]));
    let result = match reject_if_abrupt(agent, result, capability)? {
        Ok(r) => r,
        Err(p) => return Ok(p),
    };
    // 11. If result is not an Object, then reject with a TypeError.
    let Value::Object(result) = result else {
        return reject_type_error(
            agent,
            capability,
            "iterator.return() did not return an object",
        );
    };
    // 12. Return AsyncFromSyncIteratorContinuation(result, promiseCapability, syncIteratorRecord, false).
    continuation(agent, result, capability, sync, false)
}

/// %AsyncFromSyncIteratorPrototype%.throw ( [ value ] ) (§27.1.6.2.3)
pub fn async_from_sync_throw(agent: &mut Agent, this: Value, args: &[Value]) -> JsResult<Value> {
    let (capability, sync) = begin(agent, this)?;
    let iterator = Value::Object(sync.iterator);
    // 5. Let throw be Completion(GetMethod(syncIterator, "throw")).
    let throw_key = PropertyKey::String(agent.names.throw);
    let t = get_method(agent, iterator, throw_key);
    let t = match reject_if_abrupt(agent, t, capability)? {
        Ok(t) => t,
        Err(p) => return Ok(p),
    };
    // 7. If throw is undefined, then
    if t.is_undefined() {
        // a-c. Close the sync iterator so it can clean up.
        let closed = iterator_close(agent, &sync, Ok(()));
        if let Err(p) = reject_if_abrupt(agent, closed, capability)? {
            return Ok(p);
        }
        // e-g. Reject with a TypeError.
        return reject_type_error(
            agent,
            capability,
            "The iterator does not provide a 'throw' method",
        );
    }
    // 8-9. Let result be Completion(Call(throw, syncIterator, « value »)) (without value if absent).
    let result = call(agent, t, iterator, args.get(..1).unwrap_or(&[]));
    let result = match reject_if_abrupt(agent, result, capability)? {
        Ok(r) => r,
        Err(p) => return Ok(p),
    };
    // 11. If result is not an Object, then reject with a TypeError.
    let Value::Object(result) = result else {
        return reject_type_error(
            agent,
            capability,
            "iterator.throw() did not return an object",
        );
    };
    // 12. Return AsyncFromSyncIteratorContinuation(result, promiseCapability, syncIteratorRecord, true).
    continuation(agent, result, capability, sync, true)
}

/// AsyncFromSyncIteratorContinuation(result, promiseCapability, syncIteratorRecord, closeOnRejection) (§27.1.6.4)
fn continuation(
    agent: &mut Agent,
    result: Object,
    capability: PromiseCapability,
    sync: IteratorRecord,
    close_on_rejection: bool,
) -> JsResult<Value> {
    // 1-2. Let done be Completion(IteratorComplete(result)); IfAbruptRejectPromise(done, promiseCapability).
    let done = iterator_complete(agent, result);
    let done = match reject_if_abrupt(agent, done, capability)? {
        Ok(d) => d,
        Err(p) => return Ok(p),
    };
    // 3-4. Let value be Completion(IteratorValue(result)); IfAbruptRejectPromise(value, promiseCapability).
    let value = iterator_value(agent, result);
    let value = match reject_if_abrupt(agent, value, capability)? {
        Ok(v) => v,
        Err(p) => return Ok(p),
    };
    // 5. Let valueWrapper be Completion(PromiseResolve(%Promise%, value)).
    let promise_ctor = agent.intrinsic("%Promise%");
    let mut wrapper = promise_resolve(agent, promise_ctor, value);
    // 6. If valueWrapper is an abrupt completion, done is false, and closeOnRejection is true, then
    //    set valueWrapper to Completion(IteratorClose(syncIteratorRecord, valueWrapper)).
    if wrapper.is_err() && !done && close_on_rejection {
        wrapper = iterator_close(agent, &sync, wrapper);
    }
    // 7. IfAbruptRejectPromise(valueWrapper, promiseCapability).
    let wrapper = match reject_if_abrupt(agent, wrapper, capability)? {
        Ok(w) => w,
        Err(p) => return Ok(p),
    };
    // 8-9. onFulfilled unwraps the value into an iterator result with done.
    let empty = PropertyKey::String(agent.names.empty);
    let on_fulfilled = create_builtin_function(
        agent,
        unwrap,
        1,
        empty,
        vec![Value::Boolean(done)],
        None,
        None,
        None,
        false,
    );
    // 11-12. onRejected closes the sync iterator, unless done or closeOnRejection is false.
    let on_rejected = if done || !close_on_rejection {
        Value::Undefined
    } else {
        let slots = vec![Value::Object(sync.iterator), sync.next_method];
        Value::Object(create_builtin_function(
            agent,
            close_iterator,
            1,
            empty,
            slots,
            None,
            None,
            None,
            false,
        ))
    };
    // 13. Perform PerformPromiseThen(valueWrapper, onFulfilled, onRejected, promiseCapability).
    perform_promise_then(
        agent,
        wrapper,
        Value::Object(on_fulfilled),
        on_rejected,
        Some(capability),
    );
    // 14. Return promiseCapability.[[Promise]].
    Ok(capability.promise)
}

fn active_slots(agent: &Agent) -> Vec<Value> {
    match agent.active_function() {
        Some(f) => builtin_slots(agent, f).to_vec(),
        None => Vec::new(),
    }
}

/// The unwrap closure: CreateIteratorResultObject(v, done).
fn unwrap(agent: &mut Agent, _this: Value, args: &[Value], _nt: Option<Object>) -> JsResult<Value> {
    let done = matches!(active_slots(agent).first(), Some(Value::Boolean(true)));
    Ok(Value::Object(create_iter_result_object(
        agent,
        args.get_or_undefined(0),
        done,
    )))
}

/// The closeIterator closure: IteratorClose(syncIteratorRecord, ThrowCompletion(error)).
fn close_iterator(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _nt: Option<Object>,
) -> JsResult<Value> {
    let slots = active_slots(agent);
    let error = args.get_or_undefined(0);
    if let (Some(&Value::Object(iterator)), Some(&next_method)) = (slots.first(), slots.get(1)) {
        let record = IteratorRecord {
            iterator,
            next_method,
            done: false,
        };
        return iterator_close(agent, &record, Err(JsError::Throw(error)));
    }
    Err(JsError::Throw(error))
}
