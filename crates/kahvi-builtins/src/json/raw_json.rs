//! JSON.rawJSON ( _text_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-json.rawjson

use kahvi_core::prelude::*;

/// JSON.rawJSON
pub(crate) fn raw_json(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns an object representing raw JSON text of a string, number, boolean, or null value.
    //
    // 1. Let _jsonString_ be ? ToString(_text_).
    // 2. If _jsonString_ is the empty String, throw a *SyntaxError* exception.
    // 3. If the first code unit of _jsonString_ is not either an ASCII lowercase letter code unit (0x0061 through 0x007A, inclusive), an ASCII digit code unit (0x0030 through 0x0039, inclusive), 0x0022 (QUOTATION MARK), or 0x002D (HYPHEN-MINUS), throw a *SyntaxError* exception.
    // 4. If the last code unit of _jsonString_ is not either an ASCII lowercase letter code unit (0x0061 through 0x007A, inclusive), an ASCII digit code unit (0x0030 through 0x0039, inclusive), or 0x0022 (QUOTATION MARK), throw a *SyntaxError* exception.
    // 5. Let _parseResult_ be ? ParseJSON(_jsonString_).
    // 6. Assert: _parseResult_.[[Value]] is either a String, a Number, a Boolean, or *null*.
    // 7. Let _internalSlotsList_ be « [[IsRawJSON]] ».
    // 8. Let _obj_ be OrdinaryObjectCreate(*null*, _internalSlotsList_).
    // 9. Perform ! CreateDataPropertyOrThrow(_obj_, *"rawJSON"*, _jsonString_).
    // 10. Perform ! SetIntegrityLevel(_obj_, ~frozen~).
    // 11. Return _obj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("JSON.rawJSON"))
}
