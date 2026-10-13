//! RegExp.prototype [ %Symbol.replace% ] ( _string_, _replaceValue_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp.prototype-%symbol.replace%

use kahvi_core::prelude::*;

/// RegExp.prototype[Symbol.replace]
pub(crate) fn prototype_symbol_replace(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _regexp_ be the *this* value.
    // 2. If _regexp_ is not an Object, throw a *TypeError* exception.
    // 3. Set _string_ to ? ToString(_string_).
    // 4. Let _stringLength_ be the length of _string_.
    // 5. Let _functionalReplace_ be IsCallable(_replaceValue_).
    // 6. If _functionalReplace_ is *false*, then
    //    a. Set _replaceValue_ to ? ToString(_replaceValue_).
    // 7. Let _flags_ be ? ToString(? Get(_regexp_, *"flags"*)).
    // 8. If _flags_ contains *"g"*, let _global_ be *true*; else let _global_ be *false*.
    // 9. If _global_ is *true*, then
    //    a. Perform ? Set(_regexp_, *"lastIndex"*, *+0*_𝔽, *true*).
    // 10. Let _results_ be a new empty List.
    // 11. Let _done_ be *false*.
    // 12. Repeat, while _done_ is *false*,
    //    a. Let _result_ be ? RegExpExec(_regexp_, _string_).
    //    b. If _result_ is *null*, then
    //       i. Set _done_ to *true*.
    //    c. Else,
    //       i. Append _result_ to _results_.
    //       ii. If _global_ is *false*, then
    //          1. Set _done_ to *true*.
    //       iii. Else,
    //          1. Let _matchString_ be ? ToString(? Get(_result_, *"0"*)).
    //          2. If _matchString_ is the empty String, then
    //             a. Let _thisIndex_ be ℝ(? ToLength(? Get(_regexp_, *"lastIndex"*))).
    //             b. If _flags_ contains *"u"* or _flags_ contains *"v"*, let _fullUnicode_ be *true*; else let _fullUnicode_ be *false*.
    //             c. Let _nextIndex_ be AdvanceStringIndex(_string_, _thisIndex_, _fullUnicode_).
    //             d. Perform ? Set(_regexp_, *"lastIndex"*, 𝔽(_nextIndex_), *true*).
    // 13. Let _accumulatedResult_ be the empty String.
    // 14. Let _nextSourcePosition_ be 0.
    // 15. For each element _result_ of _results_, do
    //    a. Let _resultLength_ be ? LengthOfArrayLike(_result_).
    //    b. Let _capturesCount_ be max(_resultLength_ - 1, 0).
    //    c. Let _matched_ be ? ToString(? Get(_result_, *"0"*)).
    //    d. Let _matchLength_ be the length of _matched_.
    //    e. Let _position_ be ? ToIntegerOrInfinity(? Get(_result_, *"index"*)).
    //    f. Set _position_ to the result of clamping _position_ between 0 and _stringLength_.
    //    g. Let _captures_ be a new empty List.
    //    h. Let _captureNumber_ be 1.
    //    i. Repeat, while _captureNumber_ ≤ _capturesCount_,
    //       i. Let _capture_ be ? Get(_result_, ! ToString(𝔽(_captureNumber_))).
    //       ii. If _capture_ is not *undefined*, then
    //          1. Set _capture_ to ? ToString(_capture_).
    //       iii. Append _capture_ to _captures_.
    //       iv. NOTE: When _captureNumber_ = 1, the preceding step puts the first element into _captures_ (at index 0). More generally, the _captureNumber_^th capture (the characters captured by the _captureNumber_^th set of capturing parentheses) is at _captures_[_captureNumber_ - 1].
    //       v. Set _captureNumber_ to _captureNumber_ + 1.
    //    j. Let _namedCaptures_ be ? Get(_result_, *"groups"*).
    //    k. If _functionalReplace_ is *true*, then
    //       i. Let _replacerArgs_ be the list-concatenation of « _matched_ », _captures_, and « 𝔽(_position_), _string_ ».
    //       ii. If _namedCaptures_ is not *undefined*, then
    //          1. Append _namedCaptures_ to _replacerArgs_.
    //       iii. Let _replacementValue_ be ? Call(_replaceValue_, *undefined*, _replacerArgs_).
    //       iv. Let _replacementString_ be ? ToString(_replacementValue_).
    //    l. Else,
    //       i. If _namedCaptures_ is not *undefined*, then
    //          1. Set _namedCaptures_ to ? ToObject(_namedCaptures_).
    //       ii. Let _replacementString_ be ? GetSubstitution(_matched_, _string_, _position_, _captures_, _namedCaptures_, _replaceValue_).
    //    m. If _position_ ≥ _nextSourcePosition_, then
    //       i. NOTE: _position_ should not normally move backwards. If it does, it is an indication of an ill-behaving RegExp subclass or use of an access triggered side-effect to change the global flag or other characteristics of _regexp_. In such cases, the corresponding substitution is ignored.
    //       ii. Set _accumulatedResult_ to the string-concatenation of _accumulatedResult_, the substring of _string_ from _nextSourcePosition_ to _position_, and _replacementString_.
    //       iii. Set _nextSourcePosition_ to _position_ + _matchLength_.
    // 16. If _nextSourcePosition_ ≥ _stringLength_, return _accumulatedResult_.
    // 17. Return the string-concatenation of _accumulatedResult_ and the substring of _string_ from _nextSourcePosition_.
    //
    // The value of the *"name"* property of this method is *"[Symbol.replace]"*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("RegExp.prototype[Symbol.replace]"))
}
