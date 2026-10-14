//! Date.prototype.toDateString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.todatestring

use crate::support::date;
use kahvi_core::prelude::*;

const DAY_NAMES: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const MONTH_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// DateString(tv) (§21.4.4.41.2), with the year formatted as the spec requires:
/// a `-` sign for negative years and at least four digits of the absolute year.
fn date_string(tv: f64) -> String {
    // 1. Let weekday be the Name of the entry in Table 63 with the Number identified by WeekDay(tv).
    let weekday = DAY_NAMES[date::week_day(tv) as usize];
    // 2. Let month be the Name of the entry in Table 64 with the Number identified by MonthFromTime(tv).
    let month = MONTH_NAMES[date::month_from_time(tv) as usize];
    // 3. Let day be ToZeroPaddedDecimalString(DateFromTime(tv), 2).
    let day = date::date_from_time(tv) as i64;
    // 4. Let yv be YearFromTime(tv).
    let yv = date::year_from_time(tv);
    // 5. If yv is +0 or yv > 0, let yearSign be the empty String; otherwise, let yearSign be "-".
    let year_sign = if yv < 0.0 { "-" } else { "" };
    // 6. Let paddedYear be ToZeroPaddedDecimalString(abs(yv), 4).
    let padded_year = yv.abs() as i64;
    // 7. Return the string-concatenation of weekday, the code unit 0x0020 (SPACE), month,
    //    the code unit 0x0020 (SPACE), day, the code unit 0x0020 (SPACE), yearSign, and paddedYear.
    format!("{weekday} {month} {day:02} {year_sign}{padded_year:04}")
}

/// Date.prototype.toDateString
pub(crate) fn prototype_to_date_string(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let dateObj be the this value.
    // 2. Perform ? RequireInternalSlot(dateObj, [[DateValue]]).
    //    (thisTimeValue performs the RequireInternalSlot check: it throws a TypeError
    //    unless dateObj is an Object with a [[DateValue]] internal slot.)
    // 3. Let tv be dateObj.[[DateValue]].
    let tv = date::this_time_value(agent, this)?;
    // 4. If tv is NaN, return "Invalid Date".
    if tv.is_nan() {
        return Ok(Value::String(agent.string("Invalid Date")));
    }
    // 5. Let t be LocalTime(tv).
    let t = date::local_time(agent, tv);
    // 6. Return DateString(t).
    let s = date_string(t);
    Ok(Value::String(agent.string(&s)))
}
