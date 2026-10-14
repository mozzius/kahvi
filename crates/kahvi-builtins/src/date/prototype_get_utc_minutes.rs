//! Date.prototype.getUTCMinutes ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.getutcminutes

use crate::support::date;
use kahvi_core::prelude::*;

/// Date.prototype.getUTCMinutes
pub(crate) fn prototype_get_utc_minutes(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let dateObj be the this value.
    // 2. Perform ? RequireInternalSlot(dateObj, [[DateValue]]).
    // 3. Let tv be dateObj.[[DateValue]].
    let tv = date::this_time_value(agent, this)?;
    // 4. If tv is NaN, return NaN.
    if tv.is_nan() {
        return Ok(Value::Number(f64::NAN));
    }
    // 5. Return 𝔽(MinuteFromTime(tv)).
    Ok(Value::Number(date::min_from_time(tv)))
}
