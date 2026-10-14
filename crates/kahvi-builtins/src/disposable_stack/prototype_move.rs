//! DisposableStack.prototype.move ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-disposablestack.prototype.move

use kahvi_core::prelude::*;

/// DisposableStack.prototype.move
pub(crate) fn prototype_move(
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
    // 4. Let _newDisposableStack_ be ? OrdinaryCreateFromConstructor(%DisposableStack%, *"%DisposableStack.prototype%"*, « [[DisposableState]], [[DisposableResourceStack]] »).
    // 5. Set _newDisposableStack_.[[DisposableState]] to ~pending~.
    // 6. Set _newDisposableStack_.[[DisposableResourceStack]] to _disposableStack_.[[DisposableResourceStack]].
    // 7. Set _disposableStack_.[[DisposableResourceStack]] to a new empty List.
    // 8. Set _disposableStack_.[[DisposableState]] to ~disposed~.
    // 9. Return _newDisposableStack_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("DisposableStack.prototype.move"))
}
