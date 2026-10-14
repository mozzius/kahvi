//! Boolean.prototype.valueOf ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-boolean.prototype.valueof

use crate::support::this_value::this_boolean_value;
use kahvi_core::prelude::*;

/// Boolean.prototype.valueOf
pub(crate) fn prototype_value_of(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? ThisBooleanValue(this value).
    Ok(Value::Boolean(this_boolean_value(agent, this)?))
}
