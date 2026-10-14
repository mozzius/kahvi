//! String.prototype.charAt ( _position_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.charat

use kahvi_core::prelude::*;

/// String.prototype.charAt
pub(crate) fn prototype_char_at(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method returns a single element String containing the code unit at index _position_ within the String value resulting from converting this object to a String. If there is no element at that index, the result is the empty String. The result is a String value, not a String object.
    // If `pos` is an integral Number, then the result of `x.charAt(pos)` is equivalent to the result of `x.substring(pos, pos + 1)`.
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Let _string_ be ? ToString(_thisValue_).
    // 4. Set _position_ to ? ToIntegerOrInfinity(_position_).
    // 5. Let _size_ be the length of _string_.
    // 6. If _position_ < 0 or _position_ ≥ _size_, return the empty String.
    // 7. Return the substring of _string_ from _position_ to _position_ + 1.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.charAt"))
}
