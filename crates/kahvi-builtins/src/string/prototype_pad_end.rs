//! String.prototype.padEnd ( _maxLength_ [ , _fillString_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.padend

use kahvi_core::prelude::*;

/// String.prototype.padEnd
pub(crate) fn prototype_pad_end(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Return ? StringPaddingBuiltinsImpl(_thisValue_, _maxLength_, _fillString_, ~end~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.padEnd"))
}
