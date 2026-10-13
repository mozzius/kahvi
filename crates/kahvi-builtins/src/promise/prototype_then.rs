//! Promise.prototype.then ( _onFulfilled_, _onRejected_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise.prototype.then

use kahvi_core::ops::promise;
use kahvi_core::prelude::*;

/// Promise.prototype.then
pub(crate) fn prototype_then(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let promise be the this value.
    // 2. If IsPromise(promise) is false, throw a TypeError exception.
    let p = match this {
        Value::Object(o) if promise::is_promise(agent, this) => o,
        _ => return Err(agent.throw_type_error("Promise.prototype.then called on a non-promise")),
    };
    // 3. Let C be ? SpeciesConstructor(promise, %Promise%).
    let default = agent.intrinsic("%Promise%");
    let c = ops::species_constructor(agent, p, default)?;
    // 4. Let resultCapability be ? NewPromiseCapability(C).
    let result_capability = promise::new_promise_capability(agent, Value::Object(c))?;
    // 5. Return PerformPromiseThen(promise, onFulfilled, onRejected, resultCapability).
    Ok(promise::perform_promise_then(
        agent,
        p,
        args.get_or_undefined(0),
        args.get_or_undefined(1),
        Some(result_capability),
    ))
}
