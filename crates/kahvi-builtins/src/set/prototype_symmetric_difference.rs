//! Set.prototype.symmetricDifference ( _other_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.symmetricdifference

use kahvi_core::prelude::*;

/// Set.prototype.symmetricDifference
pub(crate) fn prototype_symmetric_difference(
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
    // 4. Let _keysIterator_ be ? GetIteratorFromMethod(_otherRecord_.[[SetObject]], _otherRecord_.[[Keys]]).
    // 5. Let _resultSetData_ be a copy of _set_.[[SetData]].
    // 6. Let _next_ be ~not-started~.
    // 7. Repeat, while _next_ is not ~done~,
    //    a. Set _next_ to ? IteratorStepValue(_keysIterator_).
    //    b. If _next_ is not ~done~, then
    //       i. Set _next_ to CanonicalizeKeyedCollectionKey(_next_).
    //       ii. Let _resultIndex_ be SetDataIndex(_resultSetData_, _next_).
    //       iii. If _resultIndex_ is ~not-found~, let _alreadyInResult_ be *false*; else let _alreadyInResult_ be *true*.
    //       iv. If SetDataHas(_set_.[[SetData]], _next_) is *true*, then
    //          1. If _alreadyInResult_ is *true*, set _resultSetData_[_resultIndex_] to ~empty~.
    //       v. Else,
    //          1. If _alreadyInResult_ is *false*, append _next_ to _resultSetData_.
    // 8. Let _result_ be OrdinaryObjectCreate(%Set.prototype%, « [[SetData]] »).
    // 9. Set _result_.[[SetData]] to _resultSetData_.
    // 10. Return _result_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Set.prototype.symmetricDifference"))
}
