//! Date.prototype.setUTCFullYear ( _year_ [ , _month_ [ , _day_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.setutcfullyear

use crate::support::date;
use kahvi_core::prelude::*;

/// Date.prototype.setUTCFullYear
pub(crate) fn prototype_set_utc_full_year(
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
    // 3. Let _tv_ be _dateObj_.[[DateValue]].
    let tv = date::this_time_value(agent, this)?;
    // 4. If _tv_ is *NaN*, set _tv_ to *+0*_𝔽.
    let tv = if tv.is_nan() { 0.0 } else { tv };
    // 5. Let _yearNumber_ be ? ToNumber(_year_).
    let year_number = ops::to_number(agent, args.get_or_undefined(0))?;
    // 6. If _month_ is present, let _monthNumber_ be ? ToNumber(_month_); else let _monthNumber_ be 𝔽(MonthFromTime(_tv_)).
    let month_number = if args.len() >= 2 {
        ops::to_number(agent, args.get_or_undefined(1))?
    } else {
        date::month_from_time(tv)
    };
    // 7. If _day_ is present, let _dayNumber_ be ? ToNumber(_day_); else let _dayNumber_ be 𝔽(DateFromTime(_tv_)).
    let day_number = if args.len() >= 3 {
        ops::to_number(agent, args.get_or_undefined(2))?
    } else {
        date::date_from_time(tv)
    };
    // 8. Let _newDate_ be MakeDate(MakeDay(_yearNumber_, _monthNumber_, _dayNumber_), 𝔽(TimeWithinDay(_tv_))).
    let new_date = date::make_date(
        date::make_day(year_number, month_number, day_number),
        date::time_within_day(tv),
    );
    // 9. Let _v_ be TimeClip(_newDate_).
    let v = date::time_clip(new_date);
    // 10. Set _dateObj_.[[DateValue]] to _v_.
    date::set_time_value(agent, date_obj, v);
    // 11. Return _v_.
    Ok(Value::Number(v))
}
