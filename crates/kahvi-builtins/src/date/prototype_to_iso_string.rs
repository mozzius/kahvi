//! Date.prototype.toISOString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.toisostring

use crate::support::date;
use kahvi_core::prelude::*;

/// Date.prototype.toISOString
pub(crate) fn prototype_to_iso_string(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1-3. Let tv be dateObject.[[DateValue]] (RequireInternalSlot first).
    let tv = date::this_time_value(agent, this)?;
    // 4. If tv is not finite, throw a RangeError exception.
    if !tv.is_finite() {
        return Err(agent.throw_range_error("Invalid time value"));
    }
    // 5-6. Return the Date Time String Format representation of tv in UTC.
    let s = date::iso_string(tv);
    Ok(Value::String(agent.string(&s)))
}
