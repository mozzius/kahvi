//! String.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.tostring

use kahvi_core::prelude::*;

/// String.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Return ? ThisStringValue(*this* value).
    //
    // NOTE:
    // For a String object, this method happens to return the same thing as the `valueOf` method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.toString"))
}
