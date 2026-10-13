//! Promise.prototype.catch ( _onRejected_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise.prototype.catch

use kahvi_core::prelude::*;

/// Promise.prototype.catch
pub(crate) fn prototype_catch(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _promise_ be the *this* value.
    // 2. Return ? Invoke(_promise_, *"then"*, « *undefined*, _onRejected_ »).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Promise.prototype.catch"))
}
