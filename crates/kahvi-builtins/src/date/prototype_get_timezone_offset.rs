//! Date.prototype.getTimezoneOffset ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.gettimezoneoffset

use crate::support::date;
use kahvi_core::prelude::*;

/// Date.prototype.getTimezoneOffset
pub(crate) fn prototype_get_timezone_offset(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _dateObj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
    // 3. Let _tv_ be _dateObj_.[[DateValue]].
    let tv = date::this_time_value(agent, this)?;
    // 4. If _tv_ is *NaN*, return *NaN*.
    if tv.is_nan() {
        return Ok(Value::Number(f64::NAN));
    }
    // 5. Return (_tv_ - LocalTime(_tv_)) / 𝔽(MillisecondsPerMinute).
    let offset = (tv - date::local_time(agent, tv)) / date::MS_PER_MINUTE;
    Ok(Value::Number(offset))
}
