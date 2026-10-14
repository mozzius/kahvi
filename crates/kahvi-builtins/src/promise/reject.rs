//! Promise.reject ( _reason_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise.reject

use kahvi_core::ops::promise;
use kahvi_core::prelude::*;

/// Promise.reject
pub(crate) fn reject(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let C be the this value.
    // 2. Let promiseCapability be ? NewPromiseCapability(C).
    let capability = promise::new_promise_capability(agent, this)?;
    // 3. Perform ? Call(promiseCapability.[[Reject]], undefined, « r »).
    ops::call(
        agent,
        capability.reject,
        Value::Undefined,
        &[args.get_or_undefined(0)],
    )?;
    // 4. Return promiseCapability.[[Promise]].
    Ok(capability.promise)
}
