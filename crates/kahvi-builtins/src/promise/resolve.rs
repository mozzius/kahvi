//! Promise.resolve ( _resolution_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise.resolve

use kahvi_core::ops::promise;
use kahvi_core::prelude::*;

/// Promise.resolve
pub(crate) fn resolve(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let C be the this value.
    // 2. If C is not an Object, throw a TypeError exception.
    let Value::Object(c) = this else {
        return Err(agent.throw_type_error("Promise.resolve called on a non-object"));
    };
    // 3. Return ? PromiseResolve(C, x).
    Ok(Value::Object(promise::promise_resolve(
        agent,
        c,
        args.get_or_undefined(0),
    )?))
}
