//! DisposableStack.prototype.dispose ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-disposablestack.prototype.dispose

use kahvi_core::prelude::*;

/// DisposableStack.prototype.dispose
pub(crate) fn prototype_dispose(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _disposableStack_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_disposableStack_, [[DisposableState]]).
    // 3. If _disposableStack_.[[DisposableState]] is ~disposed~, return *undefined*.
    // 4. Set _disposableStack_.[[DisposableState]] to ~disposed~.
    // 5. Return ? DisposeResources(_disposableStack_.[[DisposableResourceStack]], NormalCompletion(*undefined*)).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("DisposableStack.prototype.dispose"))
}
