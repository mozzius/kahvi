//! Number.prototype.valueOf ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-number.prototype.valueof

use crate::support::this_value::this_number_value;
use kahvi_core::prelude::*;

/// Number.prototype.valueOf
pub(crate) fn prototype_value_of(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? ThisNumberValue(this value).
    Ok(Value::Number(this_number_value(agent, this)?))
}
