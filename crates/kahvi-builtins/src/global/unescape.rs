//! unescape ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-unescape-string

use kahvi_core::prelude::*;

/// unescape
pub(crate) fn unescape(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function is a property of the global object. It computes a new version of a String value in which each escape sequence of the sort that might be introduced by the `escape` function is replaced with the code unit that it represents.
    // It is the %unescape% intrinsic object.
    // It performs the following steps when called:
    //
    // 1. Set _string_ to ? ToString(_string_).
    // 2. Let _length_ be the length of _string_.
    // 3. Let _result_ be the empty String.
    // 4. Let _k_ be 0.
    // 5. Repeat, while _k_ < _length_,
    //    a. Let _codeUnit_ be the code unit at index _k_ within _string_.
    //    b. If _codeUnit_ is the code unit 0x0025 (PERCENT SIGN), then
    //       i. Let _hexDigits_ be the empty String.
    //       ii. Let _optionalAdvance_ be 0.
    //       iii. If _k_ + 5 < _length_ and the code unit at index _k_ + 1 within _string_ is the code unit 0x0075 (LATIN SMALL LETTER U), then
    //          1. Set _hexDigits_ to the substring of _string_ from _k_ + 2 to _k_ + 6.
    //          2. Set _optionalAdvance_ to 5.
    //       iv. Else if _k_ + 3 ≤ _length_, then
    //          1. Set _hexDigits_ to the substring of _string_ from _k_ + 1 to _k_ + 3.
    //          2. Set _optionalAdvance_ to 2.
    //       v. Let _parseResult_ be ParseText(_hexDigits_, |HexDigits[~Sep]|).
    //       vi. If _parseResult_ is a Parse Node, then
    //          1. Let _codeUnitNumber_ be the MV of _parseResult_.
    //          2. Set _codeUnit_ to the code unit whose numeric value is _codeUnitNumber_.
    //          3. Set _k_ to _k_ + _optionalAdvance_.
    //    c. Set _result_ to the string-concatenation of _result_ and _codeUnit_.
    //    d. Set _k_ to _k_ + 1.
    // 6. Return _result_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("unescape"))
}
