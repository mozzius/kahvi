//! FinalizationRegistry.prototype.unregister ( _unregisterToken_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-finalization-registry.prototype.unregister

use kahvi_core::prelude::*;

/// FinalizationRegistry.prototype.unregister
pub(crate) fn prototype_unregister(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _finalizationRegistry_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_finalizationRegistry_, [[Cells]]).
    // 3. If CanBeHeldWeakly(_unregisterToken_) is *false*, throw a *TypeError* exception.
    // 4. Let _removed_ be *false*.
    // 5. For each Record { [[WeakRefTarget]], [[HeldValue]], [[UnregisterToken]] } _cell_ of _finalizationRegistry_.[[Cells]], do
    //    a. If _cell_.[[UnregisterToken]] is not ~empty~ and SameValue(_cell_.[[UnregisterToken]], _unregisterToken_) is *true*, then
    //       i. Remove _cell_ from _finalizationRegistry_.[[Cells]].
    //       ii. Set _removed_ to *true*.
    // 6. Return _removed_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("FinalizationRegistry.prototype.unregister"))
}
