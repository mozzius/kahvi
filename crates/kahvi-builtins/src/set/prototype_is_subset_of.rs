//! Set.prototype.isSubsetOf ( _other_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.issubsetof

use kahvi_core::prelude::*;

/// Set.prototype.isSubsetOf
pub(crate) fn prototype_is_subset_of(
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
    // 4. If SetDataSize(_set_.[[SetData]]) > _otherRecord_.[[Size]], return *false*.
    // 5. Let _thisSize_ be the number of elements in _set_.[[SetData]].
    // 6. Let _index_ be 0.
    // 7. Repeat, while _index_ < _thisSize_,
    //    a. Let _entry_ be _set_.[[SetData]][_index_].
    //    b. Set _index_ to _index_ + 1.
    //    c. If _entry_ is not ~empty~, then
    //       i. Let _inOther_ be ToBoolean(? Call(_otherRecord_.[[Has]], _otherRecord_.[[SetObject]], « _entry_ »)).
    //       ii. If _inOther_ is *false*, return *false*.
    //       iii. NOTE: The number of elements in _set_.[[SetData]] may have increased during execution of _otherRecord_.[[Has]].
    //       iv. Set _thisSize_ to the number of elements in _set_.[[SetData]].
    // 8. Return *true*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Set.prototype.isSubsetOf"))
}
