//! Date.prototype.setSeconds ( _second_ [ , _millisecond_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.setseconds

use crate::support::date;
use kahvi_core::prelude::*;

/// Date.prototype.setSeconds
pub(crate) fn prototype_set_seconds(
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
    // 4. Let _secondNumber_ be ? ToNumber(_second_).
    let second = args.get_or_undefined(0);
    let second_number = ops::to_number(agent, second)?;
    // 5. If _millisecond_ is present, let _millisecondNumber_ be ? ToNumber(_millisecond_).
    let millisecond_number = match args.get(1) {
        Some(&millisecond) => Some(ops::to_number(agent, millisecond)?),
        None => None,
    };
    // 6. If _tv_ is *NaN*, return *NaN*.
    if tv.is_nan() {
        return Ok(Value::Number(f64::NAN));
    }
    // 7. Set _tv_ to LocalTime(_tv_).
    let tv = date::local_time(agent, tv);
    // 8. If _millisecond_ is not present, let _millisecondNumber_ be 𝔽(MillisecondFromTime(_tv_)).
    let millisecond_number = millisecond_number.unwrap_or_else(|| date::ms_from_time(tv));
    // 9. Let _date_ be MakeDate(𝔽(Day(_tv_)), MakeTime(𝔽(HourFromTime(_tv_)), 𝔽(MinuteFromTime(_tv_)), _secondNumber_, _millisecondNumber_)).
    let date = date::make_date(
        date::day(tv),
        date::make_time(
            date::hour_from_time(tv),
            date::min_from_time(tv),
            second_number,
            millisecond_number,
        ),
    );
    // 10. Let _u_ be TimeClip(UTC(_date_)).
    let u = date::time_clip(date::utc(agent, date));
    // 11. Set _dateObj_.[[DateValue]] to _u_.
    date::set_time_value(agent, date_obj, u);
    // 12. Return _u_.
    Ok(Value::Number(u))
}
