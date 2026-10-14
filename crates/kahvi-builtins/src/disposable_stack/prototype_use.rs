//! DisposableStack.prototype.use ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-disposablestack.prototype.use

use kahvi_core::ops::dispose::DisposeKind;
use kahvi_core::prelude::*;

use crate::support::disposable_stack::{
    add_disposable_resource, disposable_stack, require_disposable_stack,
};

/// DisposableStack.prototype.use
pub(crate) fn prototype_use(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let value = args.get_or_undefined(0);
    // 1. Let _disposableStack_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_disposableStack_, [[DisposableState]]).
    let o = require_disposable_stack(agent, this)?;
    // 3. If _disposableStack_.[[DisposableState]] is ~disposed~, throw a *ReferenceError* exception.
    if disposable_stack(agent, o).disposed {
        return Err(agent.throw_reference_error("the DisposableStack is already disposed"));
    }
    // 4. Perform ? AddDisposableResource(_disposableStack_.[[DisposableResourceStack]], _value_, ~sync-dispose~).
    add_disposable_resource(agent, o, value, DisposeKind::Sync, None)?;
    // 5. Return _value_.
    Ok(value)
}
