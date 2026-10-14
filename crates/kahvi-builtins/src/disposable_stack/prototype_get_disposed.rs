//! get DisposableStack.prototype.disposed
//!
//! Spec: https://tc39.es/ecma262/#sec-get-disposablestack.prototype.disposed

use kahvi_core::prelude::*;

use crate::support::disposable_stack::{disposable_stack, require_disposable_stack};

/// get DisposableStack.prototype.disposed
pub(crate) fn prototype_get_disposed(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `DisposableStack.prototype.disposed` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _disposableStack_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_disposableStack_, [[DisposableState]]).
    // 3. If _disposableStack_.[[DisposableState]] is ~disposed~, return *true*.
    // 4. Return *false*.
    let _ = (args, new_target);
    // 1. Let _disposableStack_ be the *this* value.
    let disposable_stack_value = this;
    // 2. Perform ? RequireInternalSlot(_disposableStack_, [[DisposableState]]).
    let o = require_disposable_stack(agent, disposable_stack_value)?;
    // 3. If _disposableStack_.[[DisposableState]] is ~disposed~, return *true*.
    if disposable_stack(agent, o).disposed {
        return Ok(Value::Boolean(true));
    }
    // 4. Return *false*.
    Ok(Value::Boolean(false))
}
