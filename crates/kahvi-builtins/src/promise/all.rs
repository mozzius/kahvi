//! Promise.all ( _iterable_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise.all

use kahvi_core::function::builtin_slots;
use kahvi_core::ops::promise::{self, PromiseCapability};
use kahvi_core::ops::{IteratorKind, IteratorRecord};
use kahvi_core::prelude::*;

/// The shared state of one Promise.all call: the [[values]] list, the
/// remainingElementsCount record, and the capability's [[Resolve]].
#[derive(Debug)]
struct PromiseAllState {
    values: Vec<Value>,
    remaining_elements_count: usize,
    resolve: Value,
}

/// The [[AlreadyCalled]] and [[Index]] slots of a resolve element function.
#[derive(Debug)]
struct ElementRecord {
    already_called: bool,
    index: usize,
}

/// Promise.all
pub(crate) fn all(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let C be the this value.
    let c = this;
    // 2. Let promiseCapability be ? NewPromiseCapability(C).
    let promise_capability = promise::new_promise_capability(agent, c)?;
    let Value::Object(ctor) = c else {
        return Err(agent.throw_type_error("Promise.all called on a non-object"));
    };
    // 3. Let promiseResolve be Completion(GetPromiseResolve(C)).
    let promise_resolve = get_promise_resolve(agent, ctor);
    // 4. IfAbruptRejectPromise(promiseResolve, promiseCapability).
    let promise_resolve = match promise_resolve {
        Ok(f) => f,
        Err(e) => return reject_abrupt(agent, &promise_capability, e),
    };
    // 5. Let iteratorRecord be Completion(GetIterator(iterable, sync)).
    let iterable = args.get_or_undefined(0);
    let iterator_record = ops::get_iterator(agent, iterable, IteratorKind::Sync);
    // 6. IfAbruptRejectPromise(iteratorRecord, promiseCapability).
    let mut iterator_record = match iterator_record {
        Ok(r) => r,
        Err(e) => return reject_abrupt(agent, &promise_capability, e),
    };
    // 7. Let result be Completion(PerformPromiseAll(iteratorRecord, C, promiseCapability, promiseResolve)).
    let result = perform_promise_all(
        agent,
        &mut iterator_record,
        ctor,
        &promise_capability,
        promise_resolve,
    );
    let result = match result {
        Ok(v) => Ok(v),
        Err(e) => {
            // 8. If result is an abrupt completion, then
            // a. If iteratorRecord.[[Done]] is false, set result to Completion(IteratorClose(iteratorRecord, result)).
            if iterator_record.done {
                Err(e)
            } else {
                ops::iterator_close(agent, &iterator_record, Err::<Value, _>(e))
            }
        }
    };
    // b. IfAbruptRejectPromise(result, promiseCapability).
    let result = match result {
        Ok(v) => Ok(v),
        Err(e) => return reject_abrupt(agent, &promise_capability, e),
    };
    // 9. Return ! result.
    result
}

/// IfAbruptRejectPromise for a throw completion: reject the capability with
/// the thrown value and return its promise. Other abrupt completions propagate.
fn reject_abrupt(
    agent: &mut Agent,
    capability: &PromiseCapability,
    error: JsError,
) -> JsResult<Value> {
    match error {
        // 1. Perform ? Call(capability.[[Reject]], undefined, « error.[[Value]] »).
        JsError::Throw(e) => {
            ops::call(agent, capability.reject, Value::Undefined, &[e])?;
            // 2. Return capability.[[Promise]].
            Ok(capability.promise)
        }
        other => Err(other),
    }
}

/// GetPromiseResolve ( _promiseCtor_ ) (§27.2.4.7.2)
fn get_promise_resolve(agent: &mut Agent, promise_ctor: Object) -> JsResult<Value> {
    // 1. Let promiseResolve be ? Get(promiseCtor, "resolve").
    let key = agent.key("resolve");
    let promise_resolve = ops::get(agent, promise_ctor, key)?;
    // 2. If IsCallable(promiseResolve) is false, throw a TypeError exception.
    if !ops::is_callable(agent, promise_resolve) {
        return Err(agent.throw_type_error("Promise.resolve is not a function"));
    }
    // 3. Return promiseResolve.
    Ok(promise_resolve)
}

/// PerformPromiseAll ( _iteratorRecord_, _ctor_, _resultCapability_, _promiseResolve_ ) (§27.2.4.1.1)
fn perform_promise_all(
    agent: &mut Agent,
    iterator_record: &mut IteratorRecord,
    ctor: Object,
    result_capability: &PromiseCapability,
    promise_resolve: Value,
) -> JsResult<Value> {
    // 1. Let values be a new empty List.
    // 2. NOTE: remainingElementsCount starts at 1 instead of 0 to ensure
    //    resultCapability.[[Resolve]] is only called once, even in the presence of a
    //    misbehaving "then" which calls the passed callback before the input iterator is exhausted.
    // 3. Let remainingElementsCount be the Record { [[Value]]: 1 }.
    let proto = agent.intrinsic("%Object.prototype%");
    let state = agent.object_with_kind(
        Some(proto),
        ObjectKind::Native(Box::new(PromiseAllState {
            values: Vec::new(),
            remaining_elements_count: 1,
            resolve: result_capability.resolve,
        })),
    );
    // 4. Let index be 0.
    let mut index: usize = 0;
    // 5. Repeat,
    loop {
        // a. Let next be ? IteratorStepValue(iteratorRecord).
        let next = ops::iterator_step_value(agent, iterator_record)?;
        match next {
            // b. If next is done, then
            None => {
                let Some(state_record) = agent.native_mut::<PromiseAllState>(state) else {
                    return Err(agent.not_implemented("Promise.all state record"));
                };
                // i. Set remainingElementsCount.[[Value]] to remainingElementsCount.[[Value]] - 1.
                state_record.remaining_elements_count =
                    state_record.remaining_elements_count.saturating_sub(1);
                // ii. If remainingElementsCount.[[Value]] = 0, then
                if state_record.remaining_elements_count == 0 {
                    // 1. Let valuesArray be CreateArrayFromList(values).
                    let values = state_record.values.clone();
                    let resolve = state_record.resolve;
                    let values_array = ops::create_array_from_list(agent, &values);
                    // 2. Perform ? Call(resultCapability.[[Resolve]], undefined, « valuesArray »).
                    ops::call(
                        agent,
                        resolve,
                        Value::Undefined,
                        &[Value::Object(values_array)],
                    )?;
                }
                // iii. Return resultCapability.[[Promise]].
                return Ok(result_capability.promise);
            }
            Some(next) => {
                // c. Append undefined to values.
                let Some(state_record) = agent.native_mut::<PromiseAllState>(state) else {
                    return Err(agent.not_implemented("Promise.all state record"));
                };
                state_record.values.push(Value::Undefined);
                // d. Let nextPromise be ? Call(promiseResolve, ctor, « next »).
                let next_promise = ops::call(agent, promise_resolve, Value::Object(ctor), &[next])?;
                // e-f. Let onFulfilled be CreateBuiltinFunction(fulfilledSteps, 1, "", « [[AlreadyCalled]], [[Index]] »).
                let proto = agent.intrinsic("%Object.prototype%");
                let element_record = agent.object_with_kind(
                    Some(proto),
                    ObjectKind::Native(Box::new(ElementRecord {
                        // g. Set onFulfilled.[[AlreadyCalled]] to false.
                        already_called: false,
                        // h. Set onFulfilled.[[Index]] to index.
                        index,
                    })),
                );
                let empty = PropertyKey::String(agent.names.empty);
                let on_fulfilled = create_builtin_function(
                    agent,
                    promise_all_resolve_element,
                    1,
                    empty,
                    vec![Value::Object(state), Value::Object(element_record)],
                    None,
                    None,
                    None,
                    false,
                );
                // i. Set index to index + 1.
                index += 1;
                // j. Set remainingElementsCount.[[Value]] to remainingElementsCount.[[Value]] + 1.
                let Some(state_record) = agent.native_mut::<PromiseAllState>(state) else {
                    return Err(agent.not_implemented("Promise.all state record"));
                };
                state_record.remaining_elements_count += 1;
                // k. Perform ? Invoke(nextPromise, "then", « onFulfilled, resultCapability.[[Reject]] »).
                let then_key = PropertyKey::String(agent.names.then);
                ops::invoke(
                    agent,
                    next_promise,
                    then_key,
                    &[Value::Object(on_fulfilled), result_capability.reject],
                )?;
            }
        }
    }
}

/// Promise.all Resolve Element Functions (§27.2.4.1.2)
fn promise_all_resolve_element(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // a. Let F be the active function object.
    let Some(active) = agent.active_function() else {
        return Err(agent.not_implemented("Promise.all resolve element: no active function"));
    };
    let slots = builtin_slots(agent, active).to_vec();
    let (Some(&Value::Object(state)), Some(&Value::Object(element))) =
        (slots.first(), slots.get(1))
    else {
        return Err(agent.not_implemented("Promise.all resolve element: missing slots"));
    };
    // b. Let alreadyCalled be F.[[AlreadyCalled]].
    // c. If alreadyCalled.[[Value]] is true, return undefined.
    // d. Set alreadyCalled.[[Value]] to true.
    let Some(element_record) = agent.native_mut::<ElementRecord>(element) else {
        return Err(agent.not_implemented("Promise.all resolve element: missing record"));
    };
    if element_record.already_called {
        return Ok(Value::Undefined);
    }
    element_record.already_called = true;
    // e. Let index be F.[[Index]].
    let index = element_record.index;
    // f. Let values be F.[[Values]].
    // g. Let remainingElementsCount be F.[[RemainingElements]].
    // h. Set values[index] to x.
    let value = args.get_or_undefined(0);
    let Some(state_record) = agent.native_mut::<PromiseAllState>(state) else {
        return Err(agent.not_implemented("Promise.all resolve element: missing state"));
    };
    if let Some(slot) = state_record.values.get_mut(index) {
        *slot = value;
    }
    // i. Set remainingElementsCount.[[Value]] to remainingElementsCount.[[Value]] - 1.
    state_record.remaining_elements_count = state_record.remaining_elements_count.saturating_sub(1);
    // j. If remainingElementsCount.[[Value]] = 0, then
    if state_record.remaining_elements_count == 0 {
        // i. Let valuesArray be CreateArrayFromList(values).
        // ii. Return ? Call(resultCapability.[[Resolve]], undefined, « valuesArray »).
        let values = state_record.values.clone();
        let resolve = state_record.resolve;
        let values_array = ops::create_array_from_list(agent, &values);
        return ops::call(
            agent,
            resolve,
            Value::Undefined,
            &[Value::Object(values_array)],
        );
    }
    // k. Return undefined.
    Ok(Value::Undefined)
}
