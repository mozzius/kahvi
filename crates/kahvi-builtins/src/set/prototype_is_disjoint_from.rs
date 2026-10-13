//! Set.prototype.isDisjointFrom ( _other_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.isdisjointfrom

use kahvi_core::prelude::*;

/// Set.prototype.isDisjointFrom
pub(crate) fn prototype_is_disjoint_from(
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
    // 4. If SetDataSize(_set_.[[SetData]]) ≤ _otherRecord_.[[Size]], then
    //    a. Let _thisSize_ be the number of elements in _set_.[[SetData]].
    //    b. Let _index_ be 0.
    //    c. Repeat, while _index_ < _thisSize_,
    //       i. Let _entry_ be _set_.[[SetData]][_index_].
    //       ii. Set _index_ to _index_ + 1.
    //       iii. If _entry_ is not ~empty~, then
    //          1. Let _inOther_ be ToBoolean(? Call(_otherRecord_.[[Has]], _otherRecord_.[[SetObject]], « _entry_ »)).
    //          2. If _inOther_ is *true*, return *false*.
    //          3. NOTE: The number of elements in _set_.[[SetData]] may have increased during execution of _otherRecord_.[[Has]].
    //          4. Set _thisSize_ to the number of elements in _set_.[[SetData]].
    // 5. Else,
    //    a. Let _keysIterator_ be ? GetIteratorFromMethod(_otherRecord_.[[SetObject]], _otherRecord_.[[Keys]]).
    //    b. Let _next_ be ~not-started~.
    //    c. Repeat, while _next_ is not ~done~,
    //       i. Set _next_ to ? IteratorStepValue(_keysIterator_).
    //       ii. If _next_ is not ~done~, then
    //          1. If SetDataHas(_set_.[[SetData]], _next_) is *true*, then
    //             a. Perform ? IteratorClose(_keysIterator_, NormalCompletion(~unused~)).
    //             b. Return *false*.
    // 6. Return *true*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Set.prototype.isDisjointFrom"))
}
