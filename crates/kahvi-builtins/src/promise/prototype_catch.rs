//! Promise.prototype.catch ( _onRejected_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise.prototype.catch

use kahvi_core::prelude::*;

/// Promise.prototype.catch
pub(crate) fn prototype_catch(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let promise be the this value.
    // 2. Return ? Invoke(promise, "then", « undefined, onRejected »).
    let then = PropertyKey::String(agent.names.then);
    ops::invoke(
        agent,
        this,
        then,
        &[Value::Undefined, args.get_or_undefined(0)],
    )
}
