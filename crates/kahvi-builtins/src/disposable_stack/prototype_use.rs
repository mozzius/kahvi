//! DisposableStack.prototype.use ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-disposablestack.prototype.use

use kahvi_core::prelude::*;

/// DisposableStack.prototype.use
pub(crate) fn prototype_use(
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
    // 4. Perform ? AddDisposableResource(_disposableStack_.[[DisposableResourceStack]], _value_, ~sync-dispose~).
    // 5. Return _value_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("DisposableStack.prototype.use"))
}
