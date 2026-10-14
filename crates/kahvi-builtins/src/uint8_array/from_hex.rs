//! Uint8Array.fromHex ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-uint8array.fromhex

use kahvi_core::prelude::*;

/// Uint8Array.fromHex
pub(crate) fn from_hex(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If _string_ is not a String, throw a *TypeError* exception.
    // 2. Let _result_ be FromHex(_string_).
    // 3. If _result_.[[Error]] is not ~none~, then
    //    a. Throw _result_.[[Error]].
    // 4. Let _resultLength_ be the number of elements in _result_.[[Bytes]].
    // 5. Let _ta_ be ? AllocateTypedArray(*"Uint8Array"*, %Uint8Array%, *"%Uint8Array.prototype%"*, _resultLength_).
    // 6. Assert: _ta_.[[ViewedArrayBuffer]].[[ArrayBufferByteLength]] is the number of elements in _result_.[[Bytes]].
    // 7. Set the value at each index of _ta_.[[ViewedArrayBuffer]].[[ArrayBufferData]] to the value at the corresponding index of _result_.[[Bytes]].
    // 8. Return _ta_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Uint8Array.fromHex"))
}
