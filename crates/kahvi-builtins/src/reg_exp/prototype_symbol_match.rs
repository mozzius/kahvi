//! RegExp.prototype [ %Symbol.match% ] ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-regexp.prototype-%symbol.match%

use kahvi_core::prelude::*;

/// RegExp.prototype[Symbol.match]
pub(crate) fn prototype_symbol_match(
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
    // 4. Let _flags_ be ? ToString(? Get(_regexp_, *"flags"*)).
    // 5. If _flags_ does not contain *"g"*, return ? RegExpExec(_regexp_, _string_).
    // 6. If _flags_ contains *"u"* or _flags_ contains *"v"*, let _fullUnicode_ be *true*; else let _fullUnicode_ be *false*.
    // 7. Perform ? Set(_regexp_, *"lastIndex"*, *+0*_𝔽, *true*).
    // 8. Let _array_ be ! ArrayCreate(0).
    // 9. Let _matchCount_ be 0.
    // 10. Repeat,
    //    a. Let _result_ be ? RegExpExec(_regexp_, _string_).
    //    b. If _result_ is *null*, then
    //       i. If _matchCount_ = 0, return *null*.
    //       ii. Return _array_.
    //    c. Let _matchString_ be ? ToString(? Get(_result_, *"0"*)).
    //    d. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_matchCount_)), _matchString_).
    //    e. If _matchString_ is the empty String, then
    //       i. Let _thisIndex_ be ℝ(? ToLength(? Get(_regexp_, *"lastIndex"*))).
    //       ii. Let _nextIndex_ be AdvanceStringIndex(_string_, _thisIndex_, _fullUnicode_).
    //       iii. Perform ? Set(_regexp_, *"lastIndex"*, 𝔽(_nextIndex_), *true*).
    //    f. Set _matchCount_ to _matchCount_ + 1.
    //
    // The value of the *"name"* property of this method is *"[Symbol.match]"*.
    // NOTE:
    // The %Symbol.match% property is used by the IsRegExp abstract operation to identify objects that have the basic behaviour of regular expressions. The absence of a %Symbol.match% property or the existence of such a property whose value does not Boolean coerce to *true* indicates that the object is not intended to be used as a regular expression object.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("RegExp.prototype[Symbol.match]"))
}
