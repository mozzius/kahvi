//! String.prototype.bold ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.bold

use kahvi_core::prelude::*;

/// String.prototype.bold
pub(crate) fn prototype_bold(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _string_ be the *this* value.
    // 2. Return ? CreateHTML(_string_, *"b"*, *""*, *""*).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.bold"))
}
