//! RegExp.prototype [ %Symbol.split% ] ( _string_, _limit_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp.prototype-%symbol.split%

use kahvi_core::prelude::*;

/// RegExp.prototype[Symbol.split]
pub(crate) fn prototype_symbol_split(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method returns an Array into which substrings of the result of converting _string_ to a String have been stored. The substrings are determined by searching from left to right for matches of the *this* value regular expression; these occurrences are not part of any String in the returned array, but serve to divide up the String value.
    // The *this* value may be an empty regular expression or a regular expression that can match an empty String. In this case, the regular expression does not match the empty substring at the beginning or end of the input String, nor does it match the empty substring at the end of the previous separator match. (For example, if the regular expression matches the empty String, the String is split up into individual code unit elements; the length of the result array equals the length of the String, and each substring contains one code unit.) Only the first match at a given index of the String is considered, even if backtracking could yield a non-empty substring match at that index. (For example, `/a*?/[Symbol.split]("ab")` evaluates to the array `["a", "b"]`, while `/a*/[Symbol.split]("ab")` evaluates to the array `["","b"]`.)
    // If _string_ is (or converts to) the empty String, the result depends on whether the regular expression can match the empty String. If it can, the result array contains no elements. Otherwise, the result array contains one element, which is the empty String.
    // If the regular expression contains capturing parentheses, then each time _separator_ is matched the results (including any *undefined* results) of the capturing parentheses are spliced into the output array. For example,
    // /<(\/)?([^<>]+)>/[Symbol.split]("A<B>bold</B>and<CODE>coded</CODE>")
    // evaluates to the array
    // ["A", undefined, "B", "bold", "/", "B", "and", undefined, "CODE", "coded", "/", "CODE", ""]
    // If _limit_ is not *undefined*, then the output array is truncated so that it contains no more than _limit_ elements.
    // This method performs the following steps when called:
    //
    // 1. Let _regexp_ be the *this* value.
    // 2. If _regexp_ is not an Object, throw a *TypeError* exception.
    // 3. Set _string_ to ? ToString(_string_).
    // 4. Let _speciesCtor_ be ? SpeciesConstructor(_regexp_, %RegExp%).
    // 5. Let _flags_ be ? ToString(? Get(_regexp_, *"flags"*)).
    // 6. If _flags_ contains *"u"* or _flags_ contains *"v"*, let _unicodeMatching_ be *true*.
    // 7. Else, let _unicodeMatching_ be *false*.
    // 8. If _flags_ contains *"y"*, let _newFlags_ be _flags_.
    // 9. Else, let _newFlags_ be the string-concatenation of _flags_ and *"y"*.
    // 10. Let _splitter_ be ? Construct(_speciesCtor_, « _regexp_, _newFlags_ »).
    // 11. Let _array_ be ! ArrayCreate(0).
    // 12. Let _lengthA_ be 0.
    // 13. If _limit_ is *undefined*, let _lim_ be 2^32 - 1; else let _lim_ be ℝ(? ToUint32(_limit_)).
    // 14. If _lim_ = 0, return _array_.
    // 15. If _string_ is the empty String, then
    //    a. Let _matchResult_ be ? RegExpExec(_splitter_, _string_).
    //    b. If _matchResult_ is not *null*, return _array_.
    //    c. Perform ! CreateDataPropertyOrThrow(_array_, *"0"*, _string_).
    //    d. Return _array_.
    // 16. Let _size_ be the length of _string_.
    // 17. Let _lastMatchEnd_ be 0.
    // 18. Let _searchIndex_ be _lastMatchEnd_.
    // 19. Repeat, while _searchIndex_ < _size_,
    //    a. Perform ? Set(_splitter_, *"lastIndex"*, 𝔽(_searchIndex_), *true*).
    //    b. Let _matchResult_ be ? RegExpExec(_splitter_, _string_).
    //    c. If _matchResult_ is *null*, then
    //       i. Set _searchIndex_ to AdvanceStringIndex(_string_, _searchIndex_, _unicodeMatching_).
    //    d. Else,
    //       i. Let _matchEnd_ be ℝ(? ToLength(? Get(_splitter_, *"lastIndex"*))).
    //       ii. Set _matchEnd_ to min(_matchEnd_, _size_).
    //       iii. If _matchEnd_ = _lastMatchEnd_, then
    //          1. Set _searchIndex_ to AdvanceStringIndex(_string_, _searchIndex_, _unicodeMatching_).
    //       iv. Else,
    //          1. Let _substring_ be the substring of _string_ from _lastMatchEnd_ to _searchIndex_.
    //          2. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_lengthA_)), _substring_).
    //          3. Set _lengthA_ to _lengthA_ + 1.
    //          4. If _lengthA_ = _lim_, return _array_.
    //          5. Set _lastMatchEnd_ to _matchEnd_.
    //          6. Let _numberOfCaptures_ be ? LengthOfArrayLike(_matchResult_).
    //          7. Set _numberOfCaptures_ to max(_numberOfCaptures_ - 1, 0).
    //          8. Let _captureIndex_ be 1.
    //          9. Repeat, while _captureIndex_ ≤ _numberOfCaptures_,
    //             a. Let _nextCapture_ be ? Get(_matchResult_, ! ToString(𝔽(_captureIndex_))).
    //             b. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_lengthA_)), _nextCapture_).
    //             c. Set _captureIndex_ to _captureIndex_ + 1.
    //             d. Set _lengthA_ to _lengthA_ + 1.
    //             e. If _lengthA_ = _lim_, return _array_.
    //          10. Set _searchIndex_ to _lastMatchEnd_.
    // 20. Let _substring_ be the substring of _string_ from _lastMatchEnd_ to _size_.
    // 21. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_lengthA_)), _substring_).
    // 22. Return _array_.
    //
    // The value of the *"name"* property of this method is *"[Symbol.split]"*.
    // NOTE:
    // This method ignores the value of the *"global"* and *"sticky"* properties of this RegExp object.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("RegExp.prototype[Symbol.split]"))
}
