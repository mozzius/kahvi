//! FinalizationRegistry.prototype.register ( _target_, _heldValue_ [ , _unregisterToken_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-finalization-registry.prototype.register

use kahvi_core::prelude::*;

/// FinalizationRegistry.prototype.register
pub(crate) fn prototype_register(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _finalizationRegistry_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_finalizationRegistry_, [[Cells]]).
    // 3. If CanBeHeldWeakly(_target_) is *false*, throw a *TypeError* exception.
    // 4. If SameValue(_target_, _heldValue_) is *true*, throw a *TypeError* exception.
    // 5. If CanBeHeldWeakly(_unregisterToken_) is *false*, then
    //    a. If _unregisterToken_ is not *undefined*, throw a *TypeError* exception.
    //    b. Set _unregisterToken_ to ~empty~.
    // 6. Let _cell_ be the Record { [[WeakRefTarget]]: _target_, [[HeldValue]]: _heldValue_, [[UnregisterToken]]: _unregisterToken_ }.
    // 7. Append _cell_ to _finalizationRegistry_.[[Cells]].
    // 8. Return *undefined*.
    //
    // NOTE:
    // Based on the algorithms and definitions in this specification, _cell_.[[HeldValue]] is live when _finalizationRegistry_.[[Cells]] contains _cell_; however, this does not necessarily mean that _cell_.[[UnregisterToken]] or _cell_.[[Target]] are live. For example, registering an object with itself as its unregister token would not keep the object alive forever.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("FinalizationRegistry.prototype.register"))
}
