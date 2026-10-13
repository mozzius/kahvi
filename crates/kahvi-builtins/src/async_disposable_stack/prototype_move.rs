//! AsyncDisposableStack.prototype.move ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncdisposablestack.prototype.move

use kahvi_core::prelude::*;

/// AsyncDisposableStack.prototype.move
pub(crate) fn prototype_move(
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
    // 4. Let _newAsyncDisposableStack_ be ? OrdinaryCreateFromConstructor(%AsyncDisposableStack%, *"%AsyncDisposableStack.prototype%"*, « [[AsyncDisposableState]], [[DisposableResourceStack]] »).
    // 5. Set _newAsyncDisposableStack_.[[AsyncDisposableState]] to ~pending~.
    // 6. Set _newAsyncDisposableStack_.[[DisposableResourceStack]] to _asyncDisposableStack_.[[DisposableResourceStack]].
    // 7. Set _asyncDisposableStack_.[[DisposableResourceStack]] to a new empty List.
    // 8. Set _asyncDisposableStack_.[[AsyncDisposableState]] to ~disposed~.
    // 9. Return _newAsyncDisposableStack_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("AsyncDisposableStack.prototype.move"))
}
