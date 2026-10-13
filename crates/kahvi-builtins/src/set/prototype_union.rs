//! Set.prototype.union ( _other_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.union

use kahvi_core::prelude::*;

/// Set.prototype.union
pub(crate) fn prototype_union(
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
    //       ii. If SetDataHas(_resultSetData_, _next_) is *false*, then
    //          1. Append _next_ to _resultSetData_.
    // 8. Let _result_ be OrdinaryObjectCreate(%Set.prototype%, « [[SetData]] »).
    // 9. Set _result_.[[SetData]] to _resultSetData_.
    // 10. Return _result_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Set.prototype.union"))
}
