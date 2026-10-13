//! Date.prototype.getTime ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.gettime

use crate::support::date;
use kahvi_core::prelude::*;

/// Date.prototype.getTime
pub(crate) fn prototype_get_time(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let dateObject be the this value.
    // 2. Perform ? RequireInternalSlot(dateObject, [[DateValue]]).
    // 3. Return dateObject.[[DateValue]].
    Ok(Value::Number(date::this_time_value(agent, this)?))
}
