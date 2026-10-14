//! Date.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.tostring

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
    let padded_year = (yv.abs()) as i64;
    // 7. Return the string-concatenation of weekday, the code unit 0x0020 (SPACE), month,
    //    the code unit 0x0020 (SPACE), day, the code unit 0x0020 (SPACE), yearSign, and paddedYear.
    format!("{weekday} {month} {day:02} {year_sign}{padded_year:04}")
}

/// ToDateString(tv) (§21.4.4.41.4)
fn to_date_string(agent: &Agent, tv: f64) -> String {
    // 1. If tv is NaN, return "Invalid Date".
    if tv.is_nan() {
        return "Invalid Date".into();
    }
    // 2. Let t be LocalTime(tv).
    let t = date::local_time(agent, tv);
    // 3. Return the string-concatenation of DateString(t), the code unit 0x0020 (SPACE),
    //    TimeString(t), and TimeZoneString(tv).
    format!(
        "{} {}{}",
        date_string(t),
        date::time_string(t),
        date::time_zone_string(agent, tv)
    )
}

/// Date.prototype.toString
pub(crate) fn prototype_to_string(
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
    // 4. Return ToDateString(_tv_).
    //
    // NOTE:
    // For any Date `d` such that `d.[[DateValue]]` is evenly divisible by 1000, the result of `Date.parse(d.toString())` = `d.valueOf()`. See [sec-date.parse].
    // NOTE:
    // This method is not generic; it throws a *TypeError* exception if its *this* value is not a Date. Therefore, it cannot be transferred to other kinds of objects for use as a method.
    // [subclause sec-timestring: TimeString (
    // _tv_: a Number, but not *NaN*,
    // ): a String]
    // [subclause sec-datestring: DateString (
    // _tv_: a Number, but not *NaN*,
    // ): a String]
    // [subclause sec-timezoneestring: TimeZoneString (
    // _tv_: an integral Number,
    // ): a String]
    // [subclause sec-todatestring: ToDateString (
    // _tv_: an integral Number or *NaN*,
    // ): a String]
    // 1. Let dateObj be the this value.
    let date_obj = this;
    // 2. Perform ? RequireInternalSlot(dateObj, [[DateValue]]).
    // 3. Let tv be dateObj.[[DateValue]].
    let tv = date::this_time_value(agent, date_obj)?;
    // 4. Return ToDateString(tv).
    let s = to_date_string(agent, tv);
    Ok(Value::String(agent.string(&s)))
}
