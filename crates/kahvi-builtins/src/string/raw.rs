//! String.raw ( _template_, ..._substitutions_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.raw

use kahvi_core::prelude::*;

/// String.raw
pub(crate) fn raw(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function may be called with a variable number of arguments. The first argument is _template_ and the remainder of the arguments form the List _substitutions_.
    // It performs the following steps when called:
    //
    // 1. Let _substitutionCount_ be the number of elements in _substitutions_.
    // 2. Let _cooked_ be ? ToObject(_template_).
    // 3. Let _literals_ be ? ToObject(? Get(_cooked_, *"raw"*)).
    // 4. Let _literalCount_ be ? LengthOfArrayLike(_literals_).
    // 5. If _literalCount_ ≤ 0, return the empty String.
    // 6. Let _result_ be the empty String.
    // 7. Let _nextIndex_ be 0.
    // 8. Repeat,
    //    a. Let _nextLiteralValue_ be ? Get(_literals_, ! ToString(𝔽(_nextIndex_))).
    //    b. Let _nextLiteral_ be ? ToString(_nextLiteralValue_).
    //    c. Set _result_ to the string-concatenation of _result_ and _nextLiteral_.
    //    d. If _nextIndex_ + 1 = _literalCount_, return _result_.
    //    e. If _nextIndex_ < _substitutionCount_, then
    //       i. Let _nextSubValue_ be _substitutions_[_nextIndex_].
    //       ii. Let _nextSub_ be ? ToString(_nextSubValue_).
    //       iii. Set _result_ to the string-concatenation of _result_ and _nextSub_.
    //    f. Set _nextIndex_ to _nextIndex_ + 1.
    //
    // NOTE:
    // This function is intended for use as a tag function of a Tagged Template ([sec-tagged-templates]). When called as such, the first argument will be a well formed template object and the rest parameter will contain the substitution values.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.raw"))
}
