//! Atomics.waitAsync ( _ta_, _index_, _value_, _timeout_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.waitasync

use kahvi_core::prelude::*;

/// Atomics.waitAsync
pub(crate) fn wait_async(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns a Promise that is resolved when the calling agent is notified or the timeout is reached.
    // It performs the following steps when called:
    //
    // 1. Return ? DoWait(~async~, _ta_, _index_, _value_, _timeout_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Atomics.waitAsync"))
}
