//! Date.prototype.setMilliseconds ( _millisecond_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.setmilliseconds

use crate::support::date;
use kahvi_core::prelude::*;

/// Date.prototype.setMilliseconds
pub(crate) fn prototype_set_milliseconds(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _dateObj_ be the *this* value.
    let Value::Object(date_obj) = this else {
        return Err(agent.throw_type_error("this is not a Date object"));
    };
    // 2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
    // 3. Let _tv_ be _dateObj_.[[DateValue]].
    let tv = date::this_time_value(agent, this)?;
    // 4. Let _millisecondNumber_ be ? ToNumber(_millisecond_).
    let millisecond = args.get_or_undefined(0);
    let millisecond_number = ops::to_number(agent, millisecond)?;
    // 5. If _tv_ is *NaN*, return *NaN*.
    if tv.is_nan() {
        return Ok(Value::Number(f64::NAN));
    }
    // 6. Set _tv_ to LocalTime(_tv_).
    let tv = date::local_time(agent, tv);
    // 7. Let _time_ be MakeTime(𝔽(HourFromTime(_tv_)), 𝔽(MinuteFromTime(_tv_)), 𝔽(SecondFromTime(_tv_)), _millisecondNumber_).
    let time = date::make_time(
        date::hour_from_time(tv),
        date::min_from_time(tv),
        date::sec_from_time(tv),
        millisecond_number,
    );
    // 8. Let _u_ be TimeClip(UTC(MakeDate(𝔽(Day(_tv_)), _time_))).
    let u = date::time_clip(date::utc(agent, date::make_date(date::day(tv), time)));
    // 9. Set _dateObj_.[[DateValue]] to _u_.
    date::set_time_value(agent, date_obj, u);
    // 10. Return _u_.
    Ok(Value::Number(u))
}
