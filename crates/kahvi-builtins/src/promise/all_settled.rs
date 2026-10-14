//! Promise.allSettled ( _iterable_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise.allsettled

use kahvi_core::function::builtin_slots;
use kahvi_core::ops::iteration::{IteratorKind, IteratorRecord};
use kahvi_core::ops::promise::{self, PromiseCapability};
use kahvi_core::prelude::*;

/// The [[AlreadyCalled]] record shared by a pair of element functions.
#[derive(Debug)]
struct AlreadyCalled(bool);

/// The [[RemainingElements]] record: the count of elements not yet settled.
#[derive(Debug)]
struct RemainingElements(f64);

/// Which of the two element functions is running.
#[derive(Clone, Copy)]
enum Settlement {
    Fulfilled,
    Rejected,
}

/// IfAbruptRejectPromise for a throw completion: reject the capability and
/// return its promise. Other abrupt completions (not-implemented) propagate.
fn reject_on_throw(
    agent: &mut Agent,
    capability: PromiseCapability,
    err: JsError,
) -> JsResult<Value> {
    match err {
        JsError::Throw(reason) => {
            ops::call(agent, capability.reject, Value::Undefined, &[reason])?;
            Ok(capability.promise)
        }
        other => Err(other),
    }
}

/// GetPromiseResolve(promiseCtor) (§27.2.4.1.1 uses it; not yet in kahvi_core::ops).
fn get_promise_resolve(agent: &mut Agent, promise_ctor: Value) -> JsResult<Value> {
    // 1. Let promiseResolve be ? Get(promiseCtor, "resolve").
    let key = agent.key("resolve");
    let promise_resolve = ops::get_v(agent, promise_ctor, key)?;
    // 2. If IsCallable(promiseResolve) is false, throw a TypeError exception.
    if !ops::is_callable(agent, promise_resolve) {
        return Err(agent.throw_type_error("Promise.resolve is not a function"));
    }
    // 3. Return promiseResolve.
    Ok(promise_resolve)
}

/// Adds `delta` to a [[RemainingElements]] record and returns the new count.
fn add_remaining(agent: &mut Agent, remaining: Object, delta: f64) -> JsResult<f64> {
    let Some(record) = agent.native_mut::<RemainingElements>(remaining) else {
        return Err(agent.throw_type_error("Promise.allSettled missing RemainingElements"));
    };
    record.0 += delta;
    Ok(record.0)
}

/// The steps shared by Promise.allSettled Resolve and Reject Element Functions.
fn settle_element(agent: &mut Agent, args: &[Value], settlement: Settlement) -> JsResult<Value> {
    let x = args.get_or_undefined(0);
    // 1. Let F be the active function object.
    let Some(f) = agent.active_function() else {
        return Err(agent.throw_type_error("Promise.allSettled element function has no function"));
    };
    let slots = builtin_slots(agent, f).to_vec();
    let (
        Some(&Value::Object(already_called)),
        Some(&Value::Number(index)),
        Some(&Value::Object(values)),
        Some(&resolve),
        Some(&Value::Object(remaining)),
    ) = (
        slots.first(),
        slots.get(1),
        slots.get(2),
        slots.get(3),
        slots.get(4),
    )
    else {
        return Err(agent.throw_type_error("Promise.allSettled element function has invalid slots"));
    };
    // 2. If F.[[AlreadyCalled]] is true, return undefined.
    let Some(record) = agent.native_mut::<AlreadyCalled>(already_called) else {
        return Err(agent.throw_type_error("Promise.allSettled element function has no record"));
    };
    if record.0 {
        return Ok(Value::Undefined);
    }
    // 3. Set F.[[AlreadyCalled]] to true.
    record.0 = true;
    // 4. Let index be F.[[Index]].
    // 5. Let values be F.[[Values]].
    // 6. Let promiseCapability be F.[[Capability]].
    // 7. Let remainingElementsCount be F.[[RemainingElements]].
    // 8. Let obj be OrdinaryObjectCreate(%Object.prototype%).
    let proto = agent.intrinsic("%Object.prototype%");
    let obj = agent.ordinary_object_create(Some(proto));
    // 9. Perform ! CreateDataPropertyOrThrow(obj, "status", "fulfilled" or "rejected").
    let status_key = agent.key("status");
    let status = match settlement {
        Settlement::Fulfilled => agent.string("fulfilled"),
        Settlement::Rejected => agent.string("rejected"),
    };
    ops::create_data_property_or_throw(agent, obj, status_key, Value::String(status))?;
    // 10. Perform ! CreateDataPropertyOrThrow(obj, "value" or "reason", x).
    let value_key = match settlement {
        Settlement::Fulfilled => agent.key("value"),
        Settlement::Rejected => agent.key("reason"),
    };
    ops::create_data_property_or_throw(agent, obj, value_key, x)?;
    // 11. Set values[index] to obj.
    let index_key = agent.key_from_index(index);
    ops::create_data_property_or_throw(agent, values, index_key, Value::Object(obj))?;
    // 12. Set remainingElementsCount.[[Value]] to remainingElementsCount.[[Value]] - 1.
    let left = add_remaining(agent, remaining, -1.0)?;
    // 13. If remainingElementsCount.[[Value]] = 0, then
    if left == 0.0 {
        // a. Let valuesArray be CreateArrayFromList(values).
        // b. Return ? Call(promiseCapability.[[Resolve]], undefined, « valuesArray »).
        return ops::call(agent, resolve, Value::Undefined, &[Value::Object(values)]);
    }
    // 14. Return undefined.
    Ok(Value::Undefined)
}

/// Promise.allSettled Resolve Element Functions (§27.2.4.1.3)
fn resolve_element(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    settle_element(agent, args, Settlement::Fulfilled)
}

/// Promise.allSettled Reject Element Functions (§27.2.4.1.2)
fn reject_element(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    settle_element(agent, args, Settlement::Rejected)
}

/// PerformPromiseAllSettled(iteratorRecord, ctor, resultCapability, promiseResolve) (§27.2.4.3.1)
fn perform_promise_all_settled(
    agent: &mut Agent,
    iterator_record: &mut IteratorRecord,
    ctor: Value,
    capability: PromiseCapability,
    promise_resolve: Value,
) -> JsResult<Value> {
    // 1. Let values be a new empty List.
    // (Kept as the array that CreateArrayFromList would build; elements are added as they settle.)
    let values = ops::create_array_from_list(agent, &[]);
    // 2. Let remainingElementsCount be the Record { [[Value]]: 1 }.
    let remaining =
        agent.object_with_kind(None, ObjectKind::Native(Box::new(RemainingElements(1.0))));
    // 3. Let index be 0.
    let mut index = 0.0;
    // 4. Repeat,
    loop {
        // a. Let next be ? IteratorStepValue(iteratorRecord).
        let next = ops::iterator_step_value(agent, iterator_record)?;
        let Some(next) = next else {
            // b. If next is DONE, then
            // i. Set remainingElementsCount.[[Value]] to remainingElementsCount.[[Value]] - 1.
            let left = add_remaining(agent, remaining, -1.0)?;
            // ii. If remainingElementsCount.[[Value]] = 0, then
            if left == 0.0 {
                // 1. Let valuesArray be CreateArrayFromList(values).
                // 2. Perform ? Call(resultCapability.[[Resolve]], undefined, « valuesArray »).
                ops::call(
                    agent,
                    capability.resolve,
                    Value::Undefined,
                    &[Value::Object(values)],
                )?;
            }
            // iii. Return resultCapability.[[Promise]].
            return Ok(capability.promise);
        };
        // c. Append undefined to values.
        // d. Let nextPromise be ? Call(promiseResolve, ctor, « next »).
        let next_promise = ops::call(agent, promise_resolve, ctor, &[next])?;
        // e. Let alreadyCalled be the Record { [[Value]]: false }.
        let already_called =
            agent.object_with_kind(None, ObjectKind::Native(Box::new(AlreadyCalled(false))));
        // f. Let stepsFulfilled be the algorithm steps defined in Promise.allSettled Resolve Element Functions.
        // g. Let lengthFulfilled be 1.
        // h. Let onFulfilled be CreateBuiltinFunction(stepsFulfilled, 1, "", « [[AlreadyCalled]], [[Index]], [[Values]], [[Capability]], [[RemainingElements]] »).
        let slots = vec![
            Value::Object(already_called),
            Value::Number(index),
            Value::Object(values),
            capability.resolve,
            Value::Object(remaining),
        ];
        let empty = PropertyKey::String(agent.names.empty);
        let on_fulfilled = create_builtin_function(
            agent,
            resolve_element,
            1,
            empty,
            slots.clone(),
            None,
            None,
            None,
            false,
        );
        // i. Let stepsRejected be the algorithm steps defined in Promise.allSettled Reject Element Functions.
        // j. Let onRejected be CreateBuiltinFunction(stepsRejected, 1, "", « [[AlreadyCalled]], [[Index]], [[Values]], [[Capability]], [[RemainingElements]] »).
        let on_rejected = create_builtin_function(
            agent,
            reject_element,
            1,
            empty,
            slots,
            None,
            None,
            None,
            false,
        );
        // k. Set remainingElementsCount.[[Value]] to remainingElementsCount.[[Value]] + 1.
        add_remaining(agent, remaining, 1.0)?;
        // l. Perform ? Invoke(nextPromise, "then", « onFulfilled, onRejected »).
        let then_key = agent.key("then");
        ops::invoke(
            agent,
            next_promise,
            then_key,
            &[Value::Object(on_fulfilled), Value::Object(on_rejected)],
        )?;
        // m. Set index to index + 1.
        index += 1.0;
    }
}

/// Promise.allSettled
pub(crate) fn all_settled(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let C be the this value.
    let c = this;
    // 2. Let promiseCapability be ? NewPromiseCapability(C).
    let capability = promise::new_promise_capability(agent, c)?;
    // 3. Let promiseResolve be Completion(GetPromiseResolve(C)).
    // 4. IfAbruptRejectPromise(promiseResolve, promiseCapability).
    let promise_resolve = match get_promise_resolve(agent, c) {
        Ok(f) => f,
        Err(e) => return reject_on_throw(agent, capability, e),
    };
    // 5. Let iteratorRecord be Completion(GetIterator(iterable, sync)).
    // 6. IfAbruptRejectPromise(iteratorRecord, promiseCapability).
    let iterable = args.get_or_undefined(0);
    let mut iterator_record = match ops::get_iterator(agent, iterable, IteratorKind::Sync) {
        Ok(r) => r,
        Err(e) => return reject_on_throw(agent, capability, e),
    };
    // 7. Let result be Completion(PerformPromiseAllSettled(iteratorRecord, C, promiseCapability, promiseResolve)).
    let result =
        perform_promise_all_settled(agent, &mut iterator_record, c, capability, promise_resolve);
    // 8. If result is an abrupt completion, then
    let result = match result {
        Err(e) => {
            // a. If iteratorRecord.[[Done]] is false, set result to Completion(IteratorClose(iteratorRecord, result)).
            if iterator_record.done {
                Err(e)
            } else {
                ops::iterator_close(agent, &iterator_record, Err::<Value, _>(e))
            }
        }
        ok => ok,
    };
    match result {
        // 9. Return ! result.
        Ok(v) => Ok(v),
        // b. IfAbruptRejectPromise(result, promiseCapability).
        Err(e) => reject_on_throw(agent, capability, e),
    }
}

impl kahvi_core::gc::Trace for AlreadyCalled {
    fn trace(&self, _t: &mut kahvi_core::gc::Tracer) {}
}

impl kahvi_core::gc::Trace for RemainingElements {
    fn trace(&self, _t: &mut kahvi_core::gc::Tracer) {}
}
