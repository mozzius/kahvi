//! DisposableStack.prototype.defer ( _onDispose_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-disposablestack.prototype.defer

use kahvi_core::prelude::*;

/// DisposableStack.prototype.defer
pub(crate) fn prototype_defer(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _disposableStack_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_disposableStack_, [[DisposableState]]).
    // 3. If _disposableStack_.[[DisposableState]] is ~disposed~, throw a *ReferenceError* exception.
    // 4. If IsCallable(_onDispose_) is *false*, throw a *TypeError* exception.
    // 5. Perform ? AddDisposableResource(_disposableStack_.[[DisposableResourceStack]], *undefined*, ~sync-dispose~, _onDispose_).
    // 6. Return *undefined*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("DisposableStack.prototype.defer"))
}
