//! String.prototype.codePointAt ( _position_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.codepointat

use kahvi_core::prelude::*;

/// String.prototype.codePointAt
pub(crate) fn prototype_code_point_at(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method returns a non-negative integral Number less than or equal to *0x10FFFF*_𝔽 that is the numeric value of the UTF-16 encoded code point ([sec-ecmascript-language-types-string-type]) starting at the string element at index _position_ within the String resulting from converting this object to a String. If there is no element at that index, the result is *undefined*. If a valid UTF-16 surrogate pair does not begin at _position_, the result is the code unit at _position_.
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Let _string_ be ? ToString(_thisValue_).
    // 4. Set _position_ to ? ToIntegerOrInfinity(_position_).
    // 5. Let _size_ be the length of _string_.
    // 6. If _position_ < 0 or _position_ ≥ _size_, return *undefined*.
    // 7. Let _codePoint_ be CodePointAt(_string_, _position_).
    // 8. Return 𝔽(_codePoint_.[[CodePoint]]).
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.codePointAt"))
}
