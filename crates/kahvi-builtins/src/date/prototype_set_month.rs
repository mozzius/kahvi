//! Date.prototype.setMonth ( _month_ [ , _day_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.setmonth

use crate::support::date;
use kahvi_core::prelude::*;

/// Date.prototype.setMonth
pub(crate) fn prototype_set_month(
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
    let t = date::this_time_value(agent, this)?;
    let Value::Object(date_obj) = this else {
        return Err(agent.throw_type_error("this is not a Date object"));
    };
    // 4. Let _monthNumber_ be ? ToNumber(_month_).
    let month_number = ops::to_number(agent, args.get_or_undefined(0))?;
    // 5. If _day_ is present, let _dayNumber_ be ? ToNumber(_day_).
    let day_number = if args.len() > 1 {
        Some(ops::to_number(agent, args.get_or_undefined(1))?)
    } else {
        None
    };
    // 6. If _tv_ is *NaN*, return *NaN*.
    if t.is_nan() {
        return Ok(Value::Number(f64::NAN));
    }
    // 7. Set _tv_ to LocalTime(_tv_).
    let tv = date::local_time(agent, t);
    // 8. If _day_ is not present, let _dayNumber_ be 𝔽(DateFromTime(_tv_)).
    let day_number = day_number.unwrap_or_else(|| date::date_from_time(tv));
    // 9. Let _newDate_ be MakeDate(MakeDay(𝔽(YearFromTime(_tv_)), _monthNumber_, _dayNumber_), 𝔽(TimeWithinDay(_tv_))).
    let new_date = date::make_date(
        date::make_day(date::year_from_time(tv), month_number, day_number),
        date::time_within_day(tv),
    );
    // 10. Let _u_ be TimeClip(UTC(_newDate_)).
    let u = date::time_clip(date::utc(agent, new_date));
    // 11. Set _dateObj_.[[DateValue]] to _u_.
    date::set_time_value(agent, date_obj, u);
    // 12. Return _u_.
    Ok(Value::Number(u))
}
