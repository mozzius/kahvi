//! AsyncDisposableStack.prototype.disposeAsync ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncdisposablestack.prototype.disposeAsync

use kahvi_core::prelude::*;

/// AsyncDisposableStack.prototype.disposeAsync
pub(crate) fn prototype_dispose_async(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This async method performs the following steps when called:
    //
    // 1. Let _asyncDisposableStack_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_asyncDisposableStack_, [[AsyncDisposableState]]).
    // 3. If _asyncDisposableStack_.[[AsyncDisposableState]] is ~disposed~, return *undefined*.
    // 4. Set _asyncDisposableStack_.[[AsyncDisposableState]] to ~disposed~.
    // 5. Return ? DisposeResources(_asyncDisposableStack_.[[DisposableResourceStack]], NormalCompletion(*undefined*)).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("AsyncDisposableStack.prototype.disposeAsync"))
}
