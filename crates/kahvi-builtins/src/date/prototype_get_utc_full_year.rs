//! Date.prototype.getUTCFullYear ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.getutcfullyear

use crate::support::date;
use kahvi_core::prelude::*;

/// Date.prototype.getUTCFullYear
pub(crate) fn prototype_get_utc_full_year(
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
    // 5. Return 𝔽(YearFromTime(_tv_)).
    Ok(Value::Number(date::year_from_time(tv)))
}
