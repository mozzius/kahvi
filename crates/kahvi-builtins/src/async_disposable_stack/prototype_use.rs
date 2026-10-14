//! AsyncDisposableStack.prototype.use ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncdisposablestack.prototype.use

use kahvi_core::ops::dispose::DisposeKind;
use kahvi_core::prelude::*;

use crate::support::disposable_stack::{
    add_disposable_resource, async_disposable_stack, require_async_disposable_stack,
};

/// AsyncDisposableStack.prototype.use
pub(crate) fn prototype_use(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let value = args.get_or_undefined(0);
    // 1. Let _asyncDisposableStack_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_asyncDisposableStack_, [[AsyncDisposableState]]).
    let o = require_async_disposable_stack(agent, this)?;
    // 3. If _asyncDisposableStack_.[[AsyncDisposableState]] is ~disposed~, throw a *ReferenceError* exception.
    if async_disposable_stack(agent, o).disposed {
        return Err(agent.throw_reference_error("the AsyncDisposableStack is already disposed"));
    }
    // 4. Perform ? AddDisposableResource(_asyncDisposableStack_.[[DisposableResourceStack]], _value_, ~async-dispose~).
    add_disposable_resource(agent, o, value, DisposeKind::Async, None)?;
    // 5. Return _value_.
    Ok(value)
}
