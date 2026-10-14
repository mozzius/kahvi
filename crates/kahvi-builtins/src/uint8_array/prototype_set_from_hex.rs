//! Uint8Array.prototype.setFromHex ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-uint8array.prototype.setfromhex

use kahvi_core::prelude::*;

/// Uint8Array.prototype.setFromHex
pub(crate) fn prototype_set_from_hex(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _into_ be the *this* value.
    // 2. Perform ? ValidateUint8Array(_into_).
    // 3. If _string_ is not a String, throw a *TypeError* exception.
    // 4. Let _taRecord_ be ? ValidateTypedArrayBounds(_into_, ~seq-cst~).
    // 5. Let _byteLength_ be TypedArrayLength(_taRecord_).
    // 6. Let _result_ be FromHex(_string_, _byteLength_).
    // 7. Let _bytes_ be _result_.[[Bytes]].
    // 8. Let _written_ be the number of elements in _bytes_.
    // 9. NOTE: FromHex does not invoke any user code, so the ArrayBuffer backing _into_ cannot have been detached or shrunk.
    // 10. Assert: _written_ ≤ _byteLength_.
    // 11. Perform SetUint8ArrayBytes(_into_, _bytes_).
    // 12. If _result_.[[Error]] is not ~none~, then
    //    a. Throw _result_.[[Error]].
    // 13. Let _resultObj_ be OrdinaryObjectCreate(%Object.prototype%).
    // 14. Perform ! CreateDataPropertyOrThrow(_resultObj_, *"read"*, 𝔽(_result_.[[Read]])).
    // 15. Perform ! CreateDataPropertyOrThrow(_resultObj_, *"written"*, 𝔽(_written_)).
    // 16. Return _resultObj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Uint8Array.prototype.setFromHex"))
}
