//! String.prototype.split ( _separator_, _limit_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.split

use kahvi_core::prelude::*;

/// String.prototype.split
pub(crate) fn prototype_split(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method returns an Array into which substrings of the result of converting this object to a String have been stored. The substrings are determined by searching from left to right for occurrences of _separator_; these occurrences are not part of any String in the returned array, but serve to divide up the String value. The value of _separator_ may be a String of any length or it may be an object, such as a RegExp, that has a %Symbol.split% method.
    // It performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. If _separator_ is an Object, then
    //    a. Let _splitter_ be ? GetMethod(_separator_, %Symbol.split%).
    //    b. If _splitter_ is not *undefined*, then
    //       i. Return ? Call(_splitter_, _separator_, « _thisValue_, _limit_ »).
    // 4. Let _string_ be ? ToString(_thisValue_).
    // 5. If _limit_ is *undefined*, let _lim_ be 2^32 - 1; else let _lim_ be ℝ(? ToUint32(_limit_)).
    // 6. Let _separatorString_ be ? ToString(_separator_).
    // 7. If _lim_ = 0, then
    //    a. Return CreateArrayFromList(« »).
    // 8. If _separator_ is *undefined*, then
    //    a. Return CreateArrayFromList(« _string_ »).
    // 9. Let _separatorLength_ be the length of _separatorString_.
    // 10. If _separatorLength_ = 0, then
    //    a. Let _stringLength_ be the length of _string_.
    //    b. Let _outLength_ be the result of clamping _lim_ between 0 and _stringLength_.
    //    c. Let _head_ be the substring of _string_ from 0 to _outLength_.
    //    d. Let _codeUnits_ be a List consisting of the sequence of code units that are the elements of _head_.
    //    e. Return CreateArrayFromList(_codeUnits_).
    // 11. If _string_ is the empty String, return CreateArrayFromList(« _string_ »).
    // 12. Let _substrings_ be a new empty List.
    // 13. Let _searchStart_ be 0.
    // 14. Let _matchIndex_ be StringIndexOf(_string_, _separatorString_, 0).
    // 15. Repeat, while _matchIndex_ is not ~not-found~,
    //    a. Let _substring_ be the substring of _string_ from _searchStart_ to _matchIndex_.
    //    b. Append _substring_ to _substrings_.
    //    c. If the number of elements in _substrings_ is _lim_, return CreateArrayFromList(_substrings_).
    //    d. Set _searchStart_ to _matchIndex_ + _separatorLength_.
    //    e. Set _matchIndex_ to StringIndexOf(_string_, _separatorString_, _searchStart_).
    // 16. Let _substring_ be the substring of _string_ from _searchStart_.
    // 17. Append _substring_ to _substrings_.
    // 18. Return CreateArrayFromList(_substrings_).
    //
    // NOTE:
    // The value of _separator_ may be an empty String. In this case, _separator_ does not match the empty substring at the beginning or end of the input String, nor does it match the empty substring at the end of the previous separator match. If _separator_ is the empty String, the String is split up into individual code unit elements; the length of the result array equals the length of the String, and each substring contains one code unit.
    // If the *this* value is (or converts to) the empty String, the result depends on whether _separator_ can match the empty String. If it can, the result array contains no elements. Otherwise, the result array contains one element, which is the empty String.
    // If _separator_ is *undefined*, then the result array contains just one String, which is the *this* value (converted to a String). If _limit_ is not *undefined*, then the output array is truncated so that it contains no more than _limit_ elements.
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.split"))
}
