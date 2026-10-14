//! DisposableStack.prototype.defer ( _onDispose_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-disposablestack.prototype.defer

use kahvi_core::ops::dispose::DisposeKind;
use kahvi_core::prelude::*;

use crate::support::disposable_stack::{
    add_disposable_resource, disposable_stack, require_disposable_stack,
};

/// DisposableStack.prototype.defer
pub(crate) fn prototype_defer(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let on_dispose = args.get_or_undefined(0);
    // 1. Let _disposableStack_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_disposableStack_, [[DisposableState]]).
    let o = require_disposable_stack(agent, this)?;
    // 3. If _disposableStack_.[[DisposableState]] is ~disposed~, throw a *ReferenceError* exception.
    if disposable_stack(agent, o).disposed {
        return Err(agent.throw_reference_error("the DisposableStack is already disposed"));
    }
    // 4. If IsCallable(_onDispose_) is *false*, throw a *TypeError* exception.
    if !ops::is_callable(agent, on_dispose) {
        return Err(
            agent.throw_type_error("DisposableStack.prototype.defer argument is not a function")
        );
    }
    // 5. Perform ? AddDisposableResource(_disposableStack_.[[DisposableResourceStack]], *undefined*, ~sync-dispose~, _onDispose_).
    add_disposable_resource(
        agent,
        o,
        Value::Undefined,
        DisposeKind::Sync,
        Some(on_dispose),
    )?;
    // 6. Return *undefined*.
    Ok(Value::Undefined)
}
