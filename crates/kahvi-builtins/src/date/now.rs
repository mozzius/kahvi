//! Date.now ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.now

use crate::support::date;
use kahvi_core::prelude::*;

/// Date.now
pub(crate) fn now(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return the time value (UTC) identifying the current time.
    let _ = args;
    Ok(Value::Number(date::now(agent)))
}
