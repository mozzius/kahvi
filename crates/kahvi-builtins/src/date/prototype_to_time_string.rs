//! Date.prototype.toTimeString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.totimestring

use crate::support::date;
use kahvi_core::prelude::*;

/// Date.prototype.toTimeString
pub(crate) fn prototype_to_time_string(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let dateObj be the this value.
    // 2. Perform ? RequireInternalSlot(dateObj, [[DateValue]]).
    //    (thisTimeValue performs the RequireInternalSlot check: it throws a TypeError
    //    unless dateObj is an Object with a [[DateValue]] internal slot.)
    // 3. Let tv be dateObj.[[DateValue]].
    let tv = date::this_time_value(agent, this)?;
    // 4. If tv is NaN, return "Invalid Date".
    if tv.is_nan() {
        return Ok(Value::String(agent.string("Invalid Date")));
    }
    // 5. Let t be LocalTime(tv).
    let t = date::local_time(agent, tv);
    // 6. Return the string-concatenation of TimeString(t) and TimeZoneString(tv).
    let s = format!(
        "{}{}",
        date::time_string(t),
        date::time_zone_string(agent, tv)
    );
    Ok(Value::String(agent.string(&s)))
}
