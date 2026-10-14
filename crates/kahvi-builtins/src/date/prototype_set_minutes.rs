//! Date.prototype.setMinutes ( _minute_ [ , _second_ [ , _millisecond_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.setminutes

use crate::support::date;
use kahvi_core::prelude::*;

/// Date.prototype.setMinutes
pub(crate) fn prototype_set_minutes(
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
    let tv = date::this_time_value(agent, this)?;
    // 3. Let _tv_ be _dateObj_.[[DateValue]].
    // (read by this_time_value above)
    // 4. Let _minuteNumber_ be ? ToNumber(_minute_).
    let minute = args.get_or_undefined(0);
    let minute_number = ops::to_number(agent, minute)?;
    // 5. If _second_ is present, let _secondNumber_ be ? ToNumber(_second_).
    let second_number = match args.get(1) {
        Some(&second) => Some(ops::to_number(agent, second)?),
        None => None,
    };
    // 6. If _millisecond_ is present, let _millisecondNumber_ be ? ToNumber(_millisecond_).
    let millisecond_number = match args.get(2) {
        Some(&millisecond) => Some(ops::to_number(agent, millisecond)?),
        None => None,
    };
    // 7. If _tv_ is *NaN*, return *NaN*.
    if tv.is_nan() {
        return Ok(Value::Number(f64::NAN));
    }
    // 8. Set _tv_ to LocalTime(_tv_).
    let tv = date::local_time(agent, tv);
    // 9. If _second_ is not present, let _secondNumber_ be 𝔽(SecondFromTime(_tv_)).
    let second_number = second_number.unwrap_or_else(|| date::sec_from_time(tv));
    // 10. If _millisecond_ is not present, let _millisecondNumber_ be 𝔽(MillisecondFromTime(_tv_)).
    let millisecond_number = millisecond_number.unwrap_or_else(|| date::ms_from_time(tv));
    // 11. Let _date_ be MakeDate(𝔽(Day(_tv_)), MakeTime(𝔽(HourFromTime(_tv_)), _minuteNumber_, _secondNumber_, _millisecondNumber_)).
    let date = date::make_date(
        date::day(tv),
        date::make_time(
            date::hour_from_time(tv),
            minute_number,
            second_number,
            millisecond_number,
        ),
    );
    // 12. Let _u_ be TimeClip(UTC(_date_)).
    let u = date::time_clip(date::utc(agent, date));
    // 13. Set _dateObj_.[[DateValue]] to _u_.
    date::set_time_value(agent, date_obj, u);
    // 14. Return _u_.
    Ok(Value::Number(u))
}
