//! get AsyncDisposableStack.prototype.disposed
//!
//! Spec: https://tc39.es/ecma262/#sec-get-asyncdisposablestack.prototype.disposed

use kahvi_core::prelude::*;

/// get AsyncDisposableStack.prototype.disposed
pub(crate) fn prototype_get_disposed(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `AsyncDisposableStack.prototype.disposed` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _asyncDisposableStack_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_asyncDisposableStack_, [[AsyncDisposableState]]).
    // 3. If _asyncDisposableStack_.[[AsyncDisposableState]] is ~disposed~, return *true*.
    // 4. Return *false*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get AsyncDisposableStack.prototype.disposed"))
}
