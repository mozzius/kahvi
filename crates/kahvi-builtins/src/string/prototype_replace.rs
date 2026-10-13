//! String.prototype.replace ( _searchValue_, _replaceValue_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.replace

use kahvi_core::prelude::*;

/// String.prototype.replace
pub(crate) fn prototype_replace(
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
    //    a. Let _replacer_ be ? GetMethod(_searchValue_, %Symbol.replace%).
    //    b. If _replacer_ is not *undefined*, then
    //       i. Return ? Call(_replacer_, _searchValue_, « _thisValue_, _replaceValue_ »).
    // 4. Let _string_ be ? ToString(_thisValue_).
    // 5. Let _searchString_ be ? ToString(_searchValue_).
    // 6. Let _functionalReplace_ be IsCallable(_replaceValue_).
    // 7. If _functionalReplace_ is *false*, then
    //    a. Set _replaceValue_ to ? ToString(_replaceValue_).
    // 8. Let _searchLength_ be the length of _searchString_.
    // 9. Let _position_ be StringIndexOf(_string_, _searchString_, 0).
    // 10. If _position_ is ~not-found~, return _string_.
    // 11. Let _preceding_ be the substring of _string_ from 0 to _position_.
    // 12. Let _following_ be the substring of _string_ from _position_ + _searchLength_.
    // 13. If _functionalReplace_ is *true*, then
    //    a. Let _replacement_ be ? ToString(? Call(_replaceValue_, *undefined*, « _searchString_, 𝔽(_position_), _string_ »)).
    // 14. Else,
    //    a. Assert: _replaceValue_ is a String.
    //    b. Let _captures_ be a new empty List.
    //    c. Let _replacement_ be ! GetSubstitution(_searchString_, _string_, _position_, _captures_, *undefined*, _replaceValue_).
    // 15. Return the string-concatenation of _preceding_, _replacement_, and _following_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    // [subclause sec-getsubstitution: GetSubstitution (
    // _matched_: a String,
    // _string_: a String,
    // _position_: a non-negative integer,
    // _captures_: a List of either Strings or *undefined*,
    // _namedCaptures_: an Object or *undefined*,
    // _replacementTemplate_: a String,
    // ): either a normal completion containing a String or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.replace"))
}
