//! Uint8Array.prototype.toHex ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-uint8array.prototype.tohex

use kahvi_core::prelude::*;

/// Uint8Array.prototype.toHex
pub(crate) fn prototype_to_hex(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? ValidateUint8Array(_obj_).
    // 3. Let _toEncode_ be ? GetUint8ArrayBytes(_obj_).
    // 4. Let _out_ be the empty String.
    // 5. For each byte _byte_ of _toEncode_, do
    //    a. Let _hex_ be Number::toString(𝔽(_byte_), 16).
    //    b. Set _hex_ to StringPad(_hex_, 2, *"0"*, ~start~).
    //    c. Set _out_ to the string-concatenation of _out_ and _hex_.
    // 6. Return _out_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Uint8Array.prototype.toHex"))
}
