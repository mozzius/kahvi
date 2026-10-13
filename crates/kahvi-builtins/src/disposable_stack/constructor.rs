//! DisposableStack ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-disposablestack

use kahvi_core::prelude::*;

/// DisposableStack
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. Let _disposableStack_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%DisposableStack.prototype%"*, « [[DisposableState]], [[DisposableResourceStack]] »).
    // 3. Set _disposableStack_.[[DisposableState]] to ~pending~.
    // 4. Set _disposableStack_.[[DisposableResourceStack]] to a new empty List.
    // 5. Return _disposableStack_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("DisposableStack"))
}
