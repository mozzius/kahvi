//! Atomics.wait ( _ta_, _index_, _value_, _timeout_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.wait

use kahvi_core::prelude::*;

/// Atomics.wait
pub(crate) fn wait(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function puts the surrounding agent in a wait queue and suspends it until notified or until the wait times out, returning a String differentiating those cases.
    // It performs the following steps when called:
    //
    // 1. Return ? DoWait(~sync~, _ta_, _index_, _value_, _timeout_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Atomics.wait"))
}
