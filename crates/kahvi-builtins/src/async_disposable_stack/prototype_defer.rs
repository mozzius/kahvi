//! AsyncDisposableStack.prototype.defer ( _onDisposeAsync_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncdisposablestack.prototype.defer

use kahvi_core::prelude::*;

/// AsyncDisposableStack.prototype.defer
pub(crate) fn prototype_defer(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _asyncDisposableStack_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_asyncDisposableStack_, [[AsyncDisposableState]]).
    // 3. If _asyncDisposableStack_.[[AsyncDisposableState]] is ~disposed~, throw a *ReferenceError* exception.
    // 4. If IsCallable(_onDisposeAsync_) is *false*, throw a *TypeError* exception.
    // 5. Perform ? AddDisposableResource(_asyncDisposableStack_.[[DisposableResourceStack]], *undefined*, ~async-dispose~, _onDisposeAsync_).
    // 6. Return *undefined*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("AsyncDisposableStack.prototype.defer"))
}
