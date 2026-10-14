//! AsyncDisposableStack.prototype.move ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncdisposablestack.prototype.move

use kahvi_core::prelude::*;

use crate::support::disposable_stack::{
    AsyncDisposableStackData, async_disposable_stack, require_async_disposable_stack,
};

/// AsyncDisposableStack.prototype.move
pub(crate) fn prototype_move(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _asyncDisposableStack_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_asyncDisposableStack_, [[AsyncDisposableState]]).
    let o = require_async_disposable_stack(agent, this)?;
    // 3. If _asyncDisposableStack_.[[AsyncDisposableState]] is ~disposed~, throw a *ReferenceError* exception.
    if async_disposable_stack(agent, o).disposed {
        return Err(agent.throw_reference_error("the AsyncDisposableStack is already disposed"));
    }
    // 4. Let _newAsyncDisposableStack_ be ? OrdinaryCreateFromConstructor(%AsyncDisposableStack%, *"%AsyncDisposableStack.prototype%"*, « [[AsyncDisposableState]], [[DisposableResourceStack]] »).
    let constructor = agent.intrinsic("%AsyncDisposableStack%");
    let new_stack = ops::ordinary_create_from_constructor(
        agent,
        constructor,
        "%AsyncDisposableStack.prototype%",
        ObjectKind::Native(Box::new(AsyncDisposableStackData {
            disposed: false,
            stack: Vec::new(),
        })),
    )?;
    // 5. Set _newAsyncDisposableStack_.[[AsyncDisposableState]] to ~pending~.
    // 6. Set _newAsyncDisposableStack_.[[DisposableResourceStack]] to _asyncDisposableStack_.[[DisposableResourceStack]].
    let resources = std::mem::take(&mut async_disposable_stack(agent, o).stack);
    async_disposable_stack(agent, new_stack).stack = resources;
    // 7. Set _asyncDisposableStack_.[[DisposableResourceStack]] to a new empty List.
    // (taken above)
    // 8. Set _asyncDisposableStack_.[[AsyncDisposableState]] to ~disposed~.
    async_disposable_stack(agent, o).disposed = true;
    // 9. Return _newAsyncDisposableStack_.
    Ok(Value::Object(new_stack))
}
