//! String.prototype.includes ( _searchString_ [ , _position_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.includes

use kahvi_core::prelude::*;

/// String.prototype.includes
pub(crate) fn prototype_includes(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Let _string_ be ? ToString(_thisValue_).
    // 4. Let _isRegexp_ be ? IsRegExp(_searchString_).
    // 5. If _isRegexp_ is *true*, throw a *TypeError* exception.
    // 6. Set _searchString_ to ? ToString(_searchString_).
    // 7. Let _length_ be the length of _string_.
    // 8. Let _start_ be the result of clamping ? ToIntegerOrInfinity(_position_) between 0 and _length_.
    // 9. Assert: If _position_ is *undefined*, then _start_ is 0.
    // 10. Let _index_ be StringIndexOf(_string_, _searchString_, _start_).
    // 11. If _index_ is ~not-found~, return *false*.
    // 12. Return *true*.
    //
    // NOTE:
    // If _searchString_ appears as a substring of the result of converting this object to a String, at one or more indices that are greater than or equal to _position_, this function returns *true*; otherwise, it returns *false*. If _position_ is *undefined*, 0 is assumed, so as to search all of the String.
    // NOTE:
    // Throwing an exception if the first argument is a RegExp is specified in order to allow future editions to define extensions that allow such argument values.
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.includes"))
}
