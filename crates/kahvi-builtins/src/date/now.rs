//! Date.now ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.now

use kahvi_core::prelude::*;

/// Date.now
pub(crate) fn now(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns the time value designating the UTC date and time of the occurrence of the call to it.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.now"))
}
