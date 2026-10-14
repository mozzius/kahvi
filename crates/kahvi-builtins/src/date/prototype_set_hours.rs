//! Date.prototype.setHours ( _hour_ [ , _minute_ [ , _second_ [ , _millisecond_ ] ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.sethours

use crate::support::date;
use kahvi_core::prelude::*;

/// Date.prototype.setHours
pub(crate) fn prototype_set_hours(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _dateObj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
    let Value::Object(date_obj) = this else {
        return Err(agent.throw_type_error("this is not a Date object"));
    };
    let tv = date::this_time_value(agent, this)?;
    // 3. Let _tv_ be _dateObj_.[[DateValue]].
    // (read by this_time_value above)
    // 4. Let _hourNumber_ be ? ToNumber(_hour_).
    let hour = args.get_or_undefined(0);
    let hour_number = ops::to_number(agent, hour)?;
    // 5. If _minute_ is present, let _minuteNumber_ be ? ToNumber(_minute_).
    let minute_number = match args.get(1) {
        Some(&minute) => Some(ops::to_number(agent, minute)?),
        None => None,
    };
    // 6. If _second_ is present, let _secondNumber_ be ? ToNumber(_second_).
    let second_number = match args.get(2) {
        Some(&second) => Some(ops::to_number(agent, second)?),
        None => None,
    };
    // 7. If _millisecond_ is present, let _millisecondNumber_ be ? ToNumber(_millisecond_).
    let millisecond_number = match args.get(3) {
        Some(&millisecond) => Some(ops::to_number(agent, millisecond)?),
        None => None,
    };
    // 8. If _tv_ is *NaN*, return *NaN*.
    if tv.is_nan() {
        return Ok(Value::Number(f64::NAN));
    }
    // 9. Set _tv_ to LocalTime(_tv_).
    let tv = date::local_time(agent, tv);
    // 10. If _minute_ is not present, let _minuteNumber_ be 𝔽(MinuteFromTime(_tv_)).
    let minute_number = minute_number.unwrap_or_else(|| date::min_from_time(tv));
    // 11. If _second_ is not present, let _secondNumber_ be 𝔽(SecondFromTime(_tv_)).
    let second_number = second_number.unwrap_or_else(|| date::sec_from_time(tv));
    // 12. If _millisecond_ is not present, let _millisecondNumber_ be 𝔽(MillisecondFromTime(_tv_)).
    let millisecond_number = millisecond_number.unwrap_or_else(|| date::ms_from_time(tv));
    // 13. Let _date_ be MakeDate(𝔽(Day(_tv_)), MakeTime(_hourNumber_, _minuteNumber_, _secondNumber_, _millisecondNumber_)).
    let date = date::make_date(
        date::day(tv),
        date::make_time(
            hour_number,
            minute_number,
            second_number,
            millisecond_number,
        ),
    );
    // 14. Let _u_ be TimeClip(UTC(_date_)).
    let u = date::time_clip(date::utc(agent, date));
    // 15. Set _dateObj_.[[DateValue]] to _u_.
    date::set_time_value(agent, date_obj, u);
    // 16. Return _u_.
    Ok(Value::Number(u))
}
