//! String.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.tostring

use crate::support::this_value::this_string_value;
use kahvi_core::prelude::*;

/// String.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? ThisStringValue(this value).
    Ok(Value::String(this_string_value(agent, this)?))
}
