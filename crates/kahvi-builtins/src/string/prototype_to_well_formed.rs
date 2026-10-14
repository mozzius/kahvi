//! String.prototype.toWellFormed ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.towellformed

use kahvi_core::prelude::*;

/// String.prototype.toWellFormed
pub(crate) fn prototype_to_well_formed(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method returns a String representation of this object with all leading surrogates and trailing surrogates that are not part of a surrogate pair replaced with U+FFFD (REPLACEMENT CHARACTER).
    // It performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Let _string_ be ? ToString(_thisValue_).
    // 4. Let _stringLength_ be the length of _string_.
    // 5. Let _k_ be 0.
    // 6. Let _result_ be the empty String.
    // 7. Repeat, while _k_ < _stringLength_,
    //    a. Let _codePoint_ be CodePointAt(_string_, _k_).
    //    b. If _codePoint_.[[IsUnpairedSurrogate]] is *true*, then
    //       i. Set _result_ to the string-concatenation of _result_ and 0xFFFD (REPLACEMENT CHARACTER).
    //    c. Else,
    //       i. Set _result_ to the string-concatenation of _result_ and UTF16EncodeCodePoint(_codePoint_.[[CodePoint]]).
    //    d. Set _k_ to _k_ + _codePoint_.[[CodeUnitCount]].
    // 8. Return _result_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.toWellFormed"))
}
