//! get AsyncDisposableStack.prototype.disposed
//!
//! Spec: https://tc39.es/ecma262/#sec-get-asyncdisposablestack.prototype.disposed

use kahvi_core::prelude::*;

use crate::support::disposable_stack::{async_disposable_stack, require_async_disposable_stack};

/// get AsyncDisposableStack.prototype.disposed
pub(crate) fn prototype_get_disposed(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (args, new_target);
    // 1. Let _asyncDisposableStack_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_asyncDisposableStack_, [[AsyncDisposableState]]).
    let o = require_async_disposable_stack(agent, this)?;
    // 3. If _asyncDisposableStack_.[[AsyncDisposableState]] is ~disposed~, return *true*.
    if async_disposable_stack(agent, o).disposed {
        return Ok(Value::Boolean(true));
    }
    // 4. Return *false*.
    Ok(Value::Boolean(false))
}
