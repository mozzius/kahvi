//! AsyncDisposableStack.prototype.use ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncdisposablestack.prototype.use

use kahvi_core::prelude::*;

/// AsyncDisposableStack.prototype.use
pub(crate) fn prototype_use(
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
    // 4. Perform ? AddDisposableResource(_asyncDisposableStack_.[[DisposableResourceStack]], _value_, ~async-dispose~).
    // 5. Return _value_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("AsyncDisposableStack.prototype.use"))
}
