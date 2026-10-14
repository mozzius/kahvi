//! Date.prototype.getHours ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.gethours

use crate::support::date;
use kahvi_core::prelude::*;

/// Date.prototype.getHours
pub(crate) fn prototype_get_hours(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _dateObj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
    // 3. Let _tv_ be _dateObj_.[[DateValue]].
    let tv = date::this_time_value(agent, this)?;
    // 4. If _tv_ is *NaN*, return *NaN*.
    if tv.is_nan() {
        return Ok(Value::Number(f64::NAN));
    }
    // 5. Return 𝔽(HourFromTime(LocalTime(_tv_))).
    Ok(Value::Number(date::hour_from_time(date::local_time(
        agent, tv,
    ))))
}
