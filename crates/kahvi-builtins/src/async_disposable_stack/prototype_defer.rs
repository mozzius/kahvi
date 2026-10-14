//! AsyncDisposableStack.prototype.defer ( _onDisposeAsync_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncdisposablestack.prototype.defer

use kahvi_core::ops::dispose::DisposeKind;
use kahvi_core::prelude::*;

use crate::support::disposable_stack::{
    add_disposable_resource, async_disposable_stack, require_async_disposable_stack,
};

/// AsyncDisposableStack.prototype.defer
pub(crate) fn prototype_defer(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let on_dispose_async = args.get_or_undefined(0);
    // 1. Let _asyncDisposableStack_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_asyncDisposableStack_, [[AsyncDisposableState]]).
    let o = require_async_disposable_stack(agent, this)?;
    // 3. If _asyncDisposableStack_.[[AsyncDisposableState]] is ~disposed~, throw a *ReferenceError* exception.
    if async_disposable_stack(agent, o).disposed {
        return Err(agent.throw_reference_error("the AsyncDisposableStack is already disposed"));
    }
    // 4. If IsCallable(_onDisposeAsync_) is *false*, throw a *TypeError* exception.
    if !ops::is_callable(agent, on_dispose_async) {
        return Err(
            agent.throw_type_error("AsyncDisposableStack.prototype.defer requires a callable")
        );
    }
    // 5. Perform ? AddDisposableResource(_asyncDisposableStack_.[[DisposableResourceStack]], *undefined*, ~async-dispose~, _onDisposeAsync_).
    add_disposable_resource(
        agent,
        o,
        Value::Undefined,
        DisposeKind::Async,
        Some(on_dispose_async),
    )?;
    // 6. Return *undefined*.
    Ok(Value::Undefined)
}
