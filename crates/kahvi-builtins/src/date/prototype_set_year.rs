//! Date.prototype.setYear ( _year_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.setyear

use crate::support::date;
use kahvi_core::prelude::*;

/// Date.prototype.setYear
pub(crate) fn prototype_set_year(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // The `setFullYear` method is preferred for nearly all purposes, because it avoids the “year 2000 problem.”
    // This method performs the following steps when called:
    //
    // 1. Let _dateObj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
    let Value::Object(date_obj) = this else {
        return Err(agent.throw_type_error("this is not a Date object"));
    };
    // 3. Let _time_ be _dateObj_.[[DateValue]].
    let time = date::this_time_value(agent, this)?;
    // 4. If _time_ is *NaN*, set _time_ to *+0*_𝔽; else set _time_ to LocalTime(_time_).
    let time = if time.is_nan() {
        0.0
    } else {
        date::local_time(agent, time)
    };
    // 5. Let _fullYear_ be MakeFullYear(? ToNumber(_year_)).
    let year = ops::to_number(agent, args.get_or_undefined(0))?;
    let full_year = date::make_full_year(year);
    // 6. Let _day_ be MakeDay(_fullYear_, 𝔽(MonthFromTime(_time_)), 𝔽(DateFromTime(_time_))).
    let day = date::make_day(
        full_year,
        date::month_from_time(time),
        date::date_from_time(time),
    );
    // 7. Let _date_ be MakeDate(_day_, 𝔽(TimeWithinDay(_time_))).
    let date = date::make_date(day, date::time_within_day(time));
    // 8. Let _utcTimestamp_ be TimeClip(UTC(_date_)).
    let utc_timestamp = date::time_clip(date::utc(agent, date));
    // 9. Set _dateObj_.[[DateValue]] to _utcTimestamp_.
    date::set_time_value(agent, date_obj, utc_timestamp);
    // 10. Return _utcTimestamp_.
    Ok(Value::Number(utc_timestamp))
}
