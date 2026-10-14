//! Set.prototype.difference ( _other_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.difference

use kahvi_core::prelude::*;

/// Set.prototype.difference
pub(crate) fn prototype_difference(
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
    // 4. Let _resultSetData_ be a copy of _set_.[[SetData]].
    // 5. If SetDataSize(_set_.[[SetData]]) ≤ _otherRecord_.[[Size]], then
    //    a. Let _thisSize_ be the number of elements in _set_.[[SetData]].
    //    b. Let _index_ be 0.
    //    c. Repeat, while _index_ < _thisSize_,
    //       i. Let _entry_ be _resultSetData_[_index_].
    //       ii. If _entry_ is not ~empty~, then
    //          1. Let _inOther_ be ToBoolean(? Call(_otherRecord_.[[Has]], _otherRecord_.[[SetObject]], « _entry_ »)).
    //          2. If _inOther_ is *true*, then
    //             a. Set _resultSetData_[_index_] to ~empty~.
    //       iii. Set _index_ to _index_ + 1.
    // 6. Else,
    //    a. Let _keysIterator_ be ? GetIteratorFromMethod(_otherRecord_.[[SetObject]], _otherRecord_.[[Keys]]).
    //    b. Let _next_ be ~not-started~.
    //    c. Repeat, while _next_ is not ~done~,
    //       i. Set _next_ to ? IteratorStepValue(_keysIterator_).
    //       ii. If _next_ is not ~done~, then
    //          1. Set _next_ to CanonicalizeKeyedCollectionKey(_next_).
    //          2. Let _valueIndex_ be SetDataIndex(_resultSetData_, _next_).
    //          3. If _valueIndex_ is not ~not-found~, then
    //             a. Set _resultSetData_[_valueIndex_] to ~empty~.
    // 7. Let _result_ be OrdinaryObjectCreate(%Set.prototype%, « [[SetData]] »).
    // 8. Set _result_.[[SetData]] to _resultSetData_.
    // 9. Return _result_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Set.prototype.difference"))
}
