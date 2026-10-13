//! Set.prototype.isSupersetOf ( _other_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.issupersetof

use kahvi_core::prelude::*;

/// Set.prototype.isSupersetOf
pub(crate) fn prototype_is_superset_of(
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
    // 4. If SetDataSize(_set_.[[SetData]]) < _otherRecord_.[[Size]], return *false*.
    // 5. Let _keysIterator_ be ? GetIteratorFromMethod(_otherRecord_.[[SetObject]], _otherRecord_.[[Keys]]).
    // 6. Let _next_ be ~not-started~.
    // 7. Repeat, while _next_ is not ~done~,
    //    a. Set _next_ to ? IteratorStepValue(_keysIterator_).
    //    b. If _next_ is not ~done~, then
    //       i. If SetDataHas(_set_.[[SetData]], _next_) is *false*, then
    //          1. Perform ? IteratorClose(_keysIterator_, NormalCompletion(~unused~)).
    //          2. Return *false*.
    // 8. Return *true*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Set.prototype.isSupersetOf"))
}
