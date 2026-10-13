//! String.prototype.lastIndexOf ( _searchString_ [ , _position_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.lastindexof

use kahvi_core::prelude::*;

/// String.prototype.lastIndexOf
pub(crate) fn prototype_last_index_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // If _searchString_ appears as a substring of the result of converting this object to a String at one or more indices that are smaller than or equal to _position_, then the greatest such index is returned; otherwise, *-1*_𝔽 is returned. If _position_ is *undefined*, the length of the String value is assumed, so as to search all of the String.
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. Let _string_ be ? ToString(_thisValue_).
    // 4. Set _searchString_ to ? ToString(_searchString_).
    // 5. Let _numberPosition_ be ? ToNumber(_position_).
    // 6. Let _length_ be the length of _string_.
    // 7. Let _searchLength_ be the length of _searchString_.
    // 8. Let _maxStart_ be _length_ - _searchLength_.
    // 9. If _maxStart_ < 0, return *-1*_𝔽.
    // 10. If _numberPosition_ is *NaN*, let _start_ be _maxStart_; else let _start_ be the result of clamping ! ToIntegerOrInfinity(_numberPosition_) between 0 and _maxStart_.
    // 11. Assert: If _position_ is *undefined*, then _start_ is _maxStart_.
    // 12. Let _result_ be StringLastIndexOf(_string_, _searchString_, _start_).
    // 13. If _result_ is ~not-found~, return *-1*_𝔽.
    // 14. Return 𝔽(_result_).
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.lastIndexOf"))
}
