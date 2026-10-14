//! Uint8Array.fromBase64 ( _string_ [ , _options_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-uint8array.frombase64

use kahvi_core::prelude::*;

/// Uint8Array.fromBase64
pub(crate) fn from_base64(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If _string_ is not a String, throw a *TypeError* exception.
    // 2. Let _optionsObj_ be ? GetOptionsObject(_options_).
    // 3. Let _alphabet_ be ? Get(_optionsObj_, *"alphabet"*).
    // 4. If _alphabet_ is *undefined*, set _alphabet_ to *"base64"*.
    // 5. If _alphabet_ is neither *"base64"* nor *"base64url"*, throw a *TypeError* exception.
    // 6. Let _lastChunkHandling_ be ? Get(_optionsObj_, *"lastChunkHandling"*).
    // 7. If _lastChunkHandling_ is *undefined*, set _lastChunkHandling_ to *"loose"*.
    // 8. If _lastChunkHandling_ is not one of *"loose"*, *"strict"*, or *"stop-before-partial"*, throw a *TypeError* exception.
    // 9. Let _result_ be FromBase64(_string_, _alphabet_, _lastChunkHandling_).
    // 10. If _result_.[[Error]] is not ~none~, then
    //    a. Throw _result_.[[Error]].
    // 11. Let _resultLength_ be the number of elements in _result_.[[Bytes]].
    // 12. Let _ta_ be ? AllocateTypedArray(*"Uint8Array"*, %Uint8Array%, *"%Uint8Array.prototype%"*, _resultLength_).
    // 13. Assert: _ta_.[[ViewedArrayBuffer]].[[ArrayBufferByteLength]] is the number of elements in _result_.[[Bytes]].
    // 14. Set the value at each index of _ta_.[[ViewedArrayBuffer]].[[ArrayBufferData]] to the value at the corresponding index of _result_.[[Bytes]].
    // 15. Return _ta_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Uint8Array.fromBase64"))
}
