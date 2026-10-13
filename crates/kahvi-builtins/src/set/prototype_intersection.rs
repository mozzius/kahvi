//! Set.prototype.intersection ( _other_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.intersection

use kahvi_core::prelude::*;

/// Set.prototype.intersection
pub(crate) fn prototype_intersection(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _set_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_set_, [[SetData]]).
    // 3. Let _otherRecord_ be ? GetSetRecord(_other_).
    // 4. Let _resultSetData_ be a new empty List.
    // 5. If SetDataSize(_set_.[[SetData]]) ≤ _otherRecord_.[[Size]], then
    //    a. Let _thisSize_ be the number of elements in _set_.[[SetData]].
    //    b. Let _index_ be 0.
    //    c. Repeat, while _index_ < _thisSize_,
    //       i. Let _entry_ be _set_.[[SetData]][_index_].
    //       ii. Set _index_ to _index_ + 1.
    //       iii. If _entry_ is not ~empty~, then
    //          1. Let _inOther_ be ToBoolean(? Call(_otherRecord_.[[Has]], _otherRecord_.[[SetObject]], « _entry_ »)).
    //          2. If _inOther_ is *true*, then
    //             a. NOTE: It is possible for earlier calls to _otherRecord_.[[Has]] to remove and re-add an element of _set_.[[SetData]], which can cause the same element to be visited twice during this iteration.
    //             b. If SetDataHas(_resultSetData_, _entry_) is *false*, then
    //                i. Append _entry_ to _resultSetData_.
    //          3. NOTE: The number of elements in _set_.[[SetData]] may have increased during execution of _otherRecord_.[[Has]].
    //          4. Set _thisSize_ to the number of elements in _set_.[[SetData]].
    // 6. Else,
    //    a. Let _keysIterator_ be ? GetIteratorFromMethod(_otherRecord_.[[SetObject]], _otherRecord_.[[Keys]]).
    //    b. Let _next_ be ~not-started~.
    //    c. Repeat, while _next_ is not ~done~,
    //       i. Set _next_ to ? IteratorStepValue(_keysIterator_).
    //       ii. If _next_ is not ~done~, then
    //          1. Set _next_ to CanonicalizeKeyedCollectionKey(_next_).
    //          2. Let _inThis_ be SetDataHas(_set_.[[SetData]], _next_).
    //          3. If _inThis_ is *true*, then
    //             a. NOTE: Because _other_ is an arbitrary object, it is possible for its *"keys"* iterator to produce the same value more than once.
    //             b. If SetDataHas(_resultSetData_, _next_) is *false*, then
    //                i. Append _next_ to _resultSetData_.
    // 7. Let _result_ be OrdinaryObjectCreate(%Set.prototype%, « [[SetData]] »).
    // 8. Set _result_.[[SetData]] to _resultSetData_.
    // 9. Return _result_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Set.prototype.intersection"))
}
