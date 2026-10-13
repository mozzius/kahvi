//! Promise.allSettled ( _iterable_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise.allsettled

use kahvi_core::prelude::*;

/// Promise.allSettled
pub(crate) fn all_settled(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns a promise that is fulfilled with an array of promise state snapshots, but only after all the original promises have settled, i.e. become either fulfilled or rejected. It resolves all elements of the passed iterable to promises as it runs this algorithm.
    //
    // 1. Let _ctor_ be the *this* value.
    // 2. Let _promiseCapability_ be ? NewPromiseCapability(_ctor_).
    // 3. Let _promiseResolve_ be Completion(GetPromiseResolve(_ctor_)).
    // 4. IfAbruptRejectPromise(_promiseResolve_, _promiseCapability_).
    // 5. Let _iteratorRecord_ be Completion(GetIterator(_iterable_, ~sync~)).
    // 6. IfAbruptRejectPromise(_iteratorRecord_, _promiseCapability_).
    // 7. Let _result_ be Completion(PerformPromiseAllSettled(_iteratorRecord_, _ctor_, _promiseCapability_, _promiseResolve_)).
    // 8. If _result_ is an abrupt completion, then
    //    a. If _iteratorRecord_.[[Done]] is *false*, set _result_ to Completion(IteratorClose(_iteratorRecord_, _result_)).
    //    b. IfAbruptRejectPromise(_result_, _promiseCapability_).
    // 9. Return ! _result_.
    //
    // NOTE:
    // This function requires its *this* value to be a constructor function that supports the parameter conventions of the Promise constructor.
    // [subclause sec-performpromiseallsettled: PerformPromiseAllSettled (
    // _iteratorRecord_: an Iterator Record,
    // _ctor_: a constructor,
    // _resultCapability_: a PromiseCapability Record,
    // _promiseResolve_: a function object,
    // ): either a normal completion containing an Object or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Promise.allSettled"))
}
