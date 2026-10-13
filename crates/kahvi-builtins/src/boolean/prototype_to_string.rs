//! Boolean.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-boolean.prototype.tostring

use kahvi_core::prelude::*;

/// Boolean.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _bool_ be ? ThisBooleanValue(*this* value).
    // 2. If _bool_ is *true*, return *"true"*.
    // 3. Return *"false"*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Boolean.prototype.toString"))
}
