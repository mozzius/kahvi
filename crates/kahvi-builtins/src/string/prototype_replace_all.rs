//! String.prototype.replaceAll ( _searchValue_, _replaceValue_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.replaceall

use kahvi_core::prelude::*;

/// String.prototype.replaceAll
pub(crate) fn prototype_replace_all(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. If _searchValue_ is an Object, then
    //    a. Let _isRegexp_ be ? IsRegExp(_searchValue_).
    //    b. If _isRegexp_ is *true*, then
    //       i. Let _flags_ be ? Get(_searchValue_, *"flags"*).
    //       ii. Perform ? RequireObjectCoercible(_flags_).
    //       iii. If ? ToString(_flags_) does not contain *"g"*, throw a *TypeError* exception.
    //    c. Let _replacer_ be ? GetMethod(_searchValue_, %Symbol.replace%).
    //    d. If _replacer_ is not *undefined*, then
    //       i. Return ? Call(_replacer_, _searchValue_, « _thisValue_, _replaceValue_ »).
    // 4. Let _string_ be ? ToString(_thisValue_).
    // 5. Let _searchString_ be ? ToString(_searchValue_).
    // 6. Let _functionalReplace_ be IsCallable(_replaceValue_).
    // 7. If _functionalReplace_ is *false*, then
    //    a. Set _replaceValue_ to ? ToString(_replaceValue_).
    // 8. Let _searchLength_ be the length of _searchString_.
    // 9. Let _advanceBy_ be max(1, _searchLength_).
    // 10. Let _matchPositions_ be a new empty List.
    // 11. Let _position_ be StringIndexOf(_string_, _searchString_, 0).
    // 12. Repeat, while _position_ is not ~not-found~,
    //    a. Append _position_ to _matchPositions_.
    //    b. Set _position_ to StringIndexOf(_string_, _searchString_, _position_ + _advanceBy_).
    // 13. Let _endOfLastMatch_ be 0.
    // 14. Let _result_ be the empty String.
    // 15. For each element _matchPosition_ of _matchPositions_, do
    //    a. Let _preserved_ be the substring of _string_ from _endOfLastMatch_ to _matchPosition_.
    //    b. If _functionalReplace_ is *true*, then
    //       i. Let _replacement_ be ? ToString(? Call(_replaceValue_, *undefined*, « _searchString_, 𝔽(_matchPosition_), _string_ »)).
    //    c. Else,
    //       i. Assert: _replaceValue_ is a String.
    //       ii. Let _captures_ be a new empty List.
    //       iii. Let _replacement_ be ! GetSubstitution(_searchString_, _string_, _matchPosition_, _captures_, *undefined*, _replaceValue_).
    //    d. Set _result_ to the string-concatenation of _result_, _preserved_, and _replacement_.
    //    e. Set _endOfLastMatch_ to _matchPosition_ + _searchLength_.
    // 16. If _endOfLastMatch_ < the length of _string_, then
    //    a. Set _result_ to the string-concatenation of _result_ and the substring of _string_ from _endOfLastMatch_.
    // 17. Return _result_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.replaceAll"))
}
