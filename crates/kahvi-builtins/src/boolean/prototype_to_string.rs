//! Boolean.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-boolean.prototype.tostring

use crate::support::this_value::this_boolean_value;
use kahvi_core::prelude::*;

/// Boolean.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let b be ? ThisBooleanValue(this value).
    let b = this_boolean_value(agent, this)?;
    // 2. If b is true, return "true"; else return "false".
    Ok(Value::String(agent.string(if b {
        "true"
    } else {
        "false"
    })))
}
