//! String.prototype.indexOf ( _searchString_ [ , _position_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.indexof

use kahvi_core::prelude::*;

/// String.prototype.indexOf
pub(crate) fn prototype_index_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // If _searchString_ appears as a substring of the result of converting this object to a String, at one or more indices that are greater than or equal to _position_, then the smallest such index is returned; otherwise, *-1*_𝔽 is returned. If _position_ is *undefined*, *+0*_𝔽 is assumed, so as to search all of the String.
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Let _string_ be ? ToString(_thisValue_).
    // 4. Set _searchString_ to ? ToString(_searchString_).
    // 5. Let _length_ be the length of _string_.
    // 6. Let _start_ be the result of clamping ? ToIntegerOrInfinity(_position_) between 0 and _length_.
    // 7. Assert: If _position_ is *undefined*, then _start_ is 0.
    // 8. Let _result_ be StringIndexOf(_string_, _searchString_, _start_).
    // 9. If _result_ is ~not-found~, return *-1*_𝔽.
    // 10. Return 𝔽(_result_).
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.indexOf"))
}
