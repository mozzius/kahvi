//! Object.prototype.valueOf ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.prototype.valueof

use kahvi_core::prelude::*;

/// Object.prototype.valueOf
pub(crate) fn prototype_value_of(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? ToObject(this value).
    Ok(Value::Object(ops::to_object(agent, this)?))
}
