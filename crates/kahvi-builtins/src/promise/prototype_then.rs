//! Promise.prototype.then ( _onFulfilled_, _onRejected_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise.prototype.then

use kahvi_core::prelude::*;

/// Promise.prototype.then
pub(crate) fn prototype_then(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _promise_ be the *this* value.
    // 2. If IsPromise(_promise_) is *false*, throw a *TypeError* exception.
    // 3. Let _ctor_ be ? SpeciesConstructor(_promise_, %Promise%).
    // 4. Let _resultCapability_ be ? NewPromiseCapability(_ctor_).
    // 5. Return PerformPromiseThen(_promise_, _onFulfilled_, _onRejected_, _resultCapability_).
    //
    // [subclause sec-performpromisethen: PerformPromiseThen (
    // _promise_: a Promise,
    // _onFulfilled_: an ECMAScript language value,
    // _onRejected_: an ECMAScript language value,
    // optional _resultCapability_: a PromiseCapability Record,
    // ): an Object or *undefined*]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Promise.prototype.then"))
}
