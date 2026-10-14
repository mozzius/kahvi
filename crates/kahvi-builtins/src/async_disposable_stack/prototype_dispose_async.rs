//! AsyncDisposableStack.prototype.disposeAsync ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncdisposablestack.prototype.disposeAsync

use kahvi_core::ops::dispose::DisposeResources;
use kahvi_core::prelude::*;

use crate::support::disposable_stack::{
    async_disposable_stack, dispose_resources_async, require_async_disposable_stack,
};

/// AsyncDisposableStack.prototype.disposeAsync
///
/// An async method: it returns a promise, and its steps' abrupt completions
/// reject that promise instead of throwing.
pub(crate) fn prototype_dispose_async(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let promise_ctor = Value::Object(agent.intrinsic("%Promise%"));
    let capability = ops::promise::new_promise_capability(agent, promise_ctor)?;
    // 1. Let _asyncDisposableStack_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_asyncDisposableStack_, [[AsyncDisposableState]]).
    let o = match require_async_disposable_stack(agent, this) {
        Ok(o) => o,
        Err(JsError::Throw(e)) => {
            ops::call(agent, capability.reject, Value::Undefined, &[e])?;
            return Ok(capability.promise);
        }
        Err(e) => return Err(e),
    };
    let data = async_disposable_stack(agent, o);
    // 3. If _asyncDisposableStack_.[[AsyncDisposableState]] is ~disposed~, return *undefined*.
    if data.disposed {
        ops::call(
            agent,
            capability.resolve,
            Value::Undefined,
            &[Value::Undefined],
        )?;
        return Ok(capability.promise);
    }
    // 4. Set _asyncDisposableStack_.[[AsyncDisposableState]] to ~disposed~.
    data.disposed = true;
    // 5. Return ? DisposeResources(_asyncDisposableStack_.[[DisposableResourceStack]], NormalCompletion(*undefined*)).
    let stack = std::mem::take(&mut data.stack);
    let dispose = DisposeResources::new(stack, Ok(Value::Undefined));
    dispose_resources_async(agent, dispose, capability)?;
    Ok(capability.promise)
}
