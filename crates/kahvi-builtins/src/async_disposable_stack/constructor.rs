//! AsyncDisposableStack ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncdisposablestack

use kahvi_core::prelude::*;

/// AsyncDisposableStack
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. Let _asyncDisposableStack_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%AsyncDisposableStack.prototype%"*, « [[AsyncDisposableState]], [[DisposableResourceStack]] »).
    // 3. Set _asyncDisposableStack_.[[AsyncDisposableState]] to ~pending~.
    // 4. Set _asyncDisposableStack_.[[DisposableResourceStack]] to a new empty List.
    // 5. Return _asyncDisposableStack_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("AsyncDisposableStack"))
}
