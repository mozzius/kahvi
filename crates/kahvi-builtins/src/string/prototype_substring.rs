//! String.prototype.substring ( _start_, _end_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.substring

use kahvi_core::prelude::*;

/// String.prototype.substring
pub(crate) fn prototype_substring(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method returns a substring of the result of converting this object to a String, starting from index _start_ and running to, but not including, index _end_ of the String (or through the end of the String if _end_ is *undefined*). The result is a String value, not a String object.
    // If either argument is *NaN* or negative, it is replaced with zero; if either argument is strictly greater than the length of the String, it is replaced with the length of the String.
    // If _start_ is strictly greater than _end_, they are swapped.
    // It performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Let _string_ be ? ToString(_thisValue_).
    // 4. Let _length_ be the length of _string_.
    // 5. Let _finalStart_ be the result of clamping ? ToIntegerOrInfinity(_start_) between 0 and _length_.
    // 6. Assert: If _start_ is *undefined*, then _finalStart_ is 0.
    // 7. If _end_ is *undefined*, let _finalEnd_ be _length_; else let _finalEnd_ be the result of clamping ? ToIntegerOrInfinity(_end_) between 0 and _length_.
    // 8. Let _from_ be min(_finalStart_, _finalEnd_).
    // 9. Let _to_ be max(_finalStart_, _finalEnd_).
    // 10. Return the substring of _string_ from _from_ to _to_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.substring"))
}
