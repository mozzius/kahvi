//! Uint8Array.prototype.toBase64 ( [ _options_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-uint8array.prototype.tobase64

use kahvi_core::prelude::*;

/// Uint8Array.prototype.toBase64
pub(crate) fn prototype_to_base64(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? ValidateUint8Array(_obj_).
    // 3. Let _optionsObj_ be ? GetOptionsObject(_options_).
    // 4. Let _alphabet_ be ? Get(_optionsObj_, *"alphabet"*).
    // 5. If _alphabet_ is *undefined*, set _alphabet_ to *"base64"*.
    // 6. If _alphabet_ is neither *"base64"* nor *"base64url"*, throw a *TypeError* exception.
    // 7. Let _omitPadding_ be ToBoolean(? Get(_optionsObj_, *"omitPadding"*)).
    // 8. Let _toEncode_ be ? GetUint8ArrayBytes(_obj_).
    // 9. If _alphabet_ is *"base64"*, then
    //    a. Let _outAscii_ be the sequence of code points which results from encoding _toEncode_ according to the base64 encoding specified in section 4 of RFC 4648. Padding is included if and only if _omitPadding_ is *false*.
    // 10. Else,
    //    a. Assert: _alphabet_ is *"base64url"*.
    //    b. Let _outAscii_ be the sequence of code points which results from encoding _toEncode_ according to the base64url encoding specified in section 5 of RFC 4648. Padding is included if and only if _omitPadding_ is *false*.
    // 11. Return CodePointsToString(_outAscii_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Uint8Array.prototype.toBase64"))
}
