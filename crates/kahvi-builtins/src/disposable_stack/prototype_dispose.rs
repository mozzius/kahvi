//! DisposableStack.prototype.dispose ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-disposablestack.prototype.dispose

use kahvi_core::ops::dispose::{DisposeResources, DisposeStep};
use kahvi_core::prelude::*;

use crate::support::disposable_stack::{disposable_stack, require_disposable_stack};

/// DisposableStack.prototype.dispose
pub(crate) fn prototype_dispose(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _disposableStack_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_disposableStack_, [[DisposableState]]).
    let o = require_disposable_stack(agent, this)?;
    let data = disposable_stack(agent, o);
    // 3. If _disposableStack_.[[DisposableState]] is ~disposed~, return *undefined*.
    if data.disposed {
        return Ok(Value::Undefined);
    }
    // 4. Set _disposableStack_.[[DisposableState]] to ~disposed~.
    data.disposed = true;
    // 5. Return ? DisposeResources(_disposableStack_.[[DisposableResourceStack]], NormalCompletion(*undefined*)).
    let stack = std::mem::take(&mut data.stack);
    let mut dispose = DisposeResources::new(stack, Ok(Value::Undefined));
    // Every resource is ~sync-dispose~, so DisposeResources never awaits.
    match dispose.step(agent)? {
        DisposeStep::Done => {}
        DisposeStep::Await(_) => unreachable!("a DisposableStack awaited"),
    }
    match dispose.output() {
        Ok(v) => Ok(v),
        Err(e) => Err(JsError::Throw(e)),
    }
}
