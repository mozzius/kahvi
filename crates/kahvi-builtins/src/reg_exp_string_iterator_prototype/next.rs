//! %RegExpStringIteratorPrototype%.next ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%regexpstringiteratorprototype%.next

use kahvi_core::prelude::*;

/// RegExpStringIteratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _iteratorObj_ be the *this* value.
    // 2. If _iteratorObj_ is not an Object, throw a *TypeError* exception.
    // 3. If _iteratorObj_ does not have all of the internal slots of a RegExp String Iterator Object Instance (see [sec-properties-of-regexp-string-iterator-instances]), throw a *TypeError* exception.
    // 4. If _iteratorObj_.[[Done]] is *true*, then
    //    a. Return CreateIteratorResultObject(*undefined*, *true*).
    // 5. Let _regexp_ be _iteratorObj_.[[IteratingRegExp]].
    // 6. Let _string_ be _iteratorObj_.[[IteratedString]].
    // 7. Let _global_ be _iteratorObj_.[[Global]].
    // 8. Let _fullUnicode_ be _iteratorObj_.[[Unicode]].
    // 9. Let _match_ be ? RegExpExec(_regexp_, _string_).
    // 10. If _match_ is *null*, then
    //    a. Set _iteratorObj_.[[Done]] to *true*.
    //    b. Return CreateIteratorResultObject(*undefined*, *true*).
    // 11. If _global_ is *false*, then
    //    a. Set _iteratorObj_.[[Done]] to *true*.
    //    b. Return CreateIteratorResultObject(_match_, *false*).
    // 12. Let _matchString_ be ? ToString(? Get(_match_, *"0"*)).
    // 13. If _matchString_ is the empty String, then
    //    a. Let _thisIndex_ be ℝ(? ToLength(? Get(_regexp_, *"lastIndex"*))).
    //    b. Let _nextIndex_ be AdvanceStringIndex(_string_, _thisIndex_, _fullUnicode_).
    //    c. Perform ? Set(_regexp_, *"lastIndex"*, 𝔽(_nextIndex_), *true*).
    // 14. Return CreateIteratorResultObject(_match_, *false*).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("RegExpStringIteratorPrototype.next"))
}
