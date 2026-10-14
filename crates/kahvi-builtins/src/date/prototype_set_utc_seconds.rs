//! Date.prototype.setUTCSeconds ( _second_ [ , _millisecond_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.setutcseconds

use crate::support::date;
use kahvi_core::prelude::*;

/// Date.prototype.setUTCSeconds
pub(crate) fn prototype_set_utc_seconds(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _dateObj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
    // 3. Let _tv_ be _dateObj_.[[DateValue]].
    let tv = date::this_time_value(agent, this)?;
    let Value::Object(date_obj) = this else {
        return Err(agent.throw_type_error("this is not a Date object"));
    };
    // 4. Let _secondNumber_ be ? ToNumber(_second_).
    let second_number = ops::to_number(agent, args.get_or_undefined(0))?;
    // 5. If _millisecond_ is present, let _millisecondNumber_ be ? ToNumber(_millisecond_).
    let millisecond_number = match args.get(1) {
        Some(&millisecond) => Some(ops::to_number(agent, millisecond)?),
        None => None,
    };
    // 6. If _tv_ is *NaN*, return *NaN*.
    if tv.is_nan() {
        return Ok(Value::Number(f64::NAN));
    }
    // 7. If _millisecond_ is not present, let _millisecondNumber_ be 𝔽(MillisecondFromTime(_tv_)).
    let millisecond_number = millisecond_number.unwrap_or_else(|| date::ms_from_time(tv));
    // 8. Let _date_ be MakeDate(𝔽(Day(_tv_)), MakeTime(𝔽(HourFromTime(_tv_)), 𝔽(MinuteFromTime(_tv_)), _secondNumber_, _millisecondNumber_)).
    let date = date::make_date(
        date::day(tv),
        date::make_time(
            date::hour_from_time(tv),
            date::min_from_time(tv),
            second_number,
            millisecond_number,
        ),
    );
    // 9. Let _v_ be TimeClip(_date_).
    let v = date::time_clip(date);
    // 10. Set _dateObj_.[[DateValue]] to _v_.
    date::set_time_value(agent, date_obj, v);
    // 11. Return _v_.
    Ok(Value::Number(v))
}
