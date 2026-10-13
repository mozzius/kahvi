//! Uint8Array.prototype.setFromBase64 ( _string_ [ , _options_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-uint8array.prototype.setfrombase64

use kahvi_core::prelude::*;

/// Uint8Array.prototype.setFromBase64
pub(crate) fn prototype_set_from_base64(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _into_ be the *this* value.
    // 2. Perform ? ValidateUint8Array(_into_).
    // 3. If _string_ is not a String, throw a *TypeError* exception.
    // 4. Let _optionsObj_ be ? GetOptionsObject(_options_).
    // 5. Let _alphabet_ be ? Get(_optionsObj_, *"alphabet"*).
    // 6. If _alphabet_ is *undefined*, set _alphabet_ to *"base64"*.
    // 7. If _alphabet_ is neither *"base64"* nor *"base64url"*, throw a *TypeError* exception.
    // 8. Let _lastChunkHandling_ be ? Get(_optionsObj_, *"lastChunkHandling"*).
    // 9. If _lastChunkHandling_ is *undefined*, set _lastChunkHandling_ to *"loose"*.
    // 10. If _lastChunkHandling_ is not one of *"loose"*, *"strict"*, or *"stop-before-partial"*, throw a *TypeError* exception.
    // 11. Let _taRecord_ be ? ValidateTypedArrayBounds(_into_, ~seq-cst~).
    // 12. Let _byteLength_ be TypedArrayLength(_taRecord_).
    // 13. Let _result_ be FromBase64(_string_, _alphabet_, _lastChunkHandling_, _byteLength_).
    // 14. Let _bytes_ be _result_.[[Bytes]].
    // 15. Let _written_ be the number of elements in _bytes_.
    // 16. NOTE: FromBase64 does not invoke any user code, so the ArrayBuffer backing _into_ cannot have been detached or shrunk.
    // 17. Assert: _written_ ≤ _byteLength_.
    // 18. Perform SetUint8ArrayBytes(_into_, _bytes_).
    // 19. If _result_.[[Error]] is not ~none~, then
    //    a. Throw _result_.[[Error]].
    // 20. Let _resultObj_ be OrdinaryObjectCreate(%Object.prototype%).
    // 21. Perform ! CreateDataPropertyOrThrow(_resultObj_, *"read"*, 𝔽(_result_.[[Read]])).
    // 22. Perform ! CreateDataPropertyOrThrow(_resultObj_, *"written"*, 𝔽(_written_)).
    // 23. Return _resultObj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Uint8Array.prototype.setFromBase64"))
}
