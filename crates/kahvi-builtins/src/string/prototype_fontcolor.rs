//! String.prototype.fontcolor ( _colour_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.fontcolor

use kahvi_core::prelude::*;

/// String.prototype.fontcolor
pub(crate) fn prototype_fontcolor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _string_ be the *this* value.
    // 2. Return ? CreateHTML(_string_, *"font"*, *"color"*, _colour_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.fontcolor"))
}
