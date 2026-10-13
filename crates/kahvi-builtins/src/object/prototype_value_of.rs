//! Object.prototype.valueOf ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.prototype.valueof

use kahvi_core::prelude::*;

/// Object.prototype.valueOf
pub(crate) fn prototype_value_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Return ? ToObject(*this* value).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.prototype.valueOf"))
}
