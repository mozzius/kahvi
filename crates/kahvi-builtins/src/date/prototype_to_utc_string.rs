//! Date.prototype.toUTCString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.toutcstring

use crate::support::date;
use kahvi_core::prelude::*;

const DAY_NAMES: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const MONTH_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// Date.prototype.toUTCString
pub(crate) fn prototype_to_utc_string(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1-3. Let tv be dateObj.[[DateValue]] (RequireInternalSlot first).
    let tv = date::this_time_value(agent, this)?;
    // 4. If tv is NaN, return "Invalid Date".
    if tv.is_nan() {
        return Ok(Value::String(agent.string("Invalid Date")));
    }
    // 5. Let weekday be the Name of the entry in [sec-todatestring-day-names] whose WeekDay Index = WeekDay(tv).
    let weekday = DAY_NAMES[date::week_day(tv) as usize];
    // 6. Let month be the Name of the entry in [sec-todatestring-month-names] whose Month Index = MonthFromTime(tv).
    let month = MONTH_NAMES[date::month_from_time(tv) as usize];
    // 7. Let day be ToZeroPaddedDecimalString(DateFromTime(tv), 2).
    let day = format!("{:02}", date::date_from_time(tv) as i64);
    // 8. Let yv be YearFromTime(tv).
    let yv = date::year_from_time(tv);
    // 9. If yv ≥ 0, let yearSign be the empty String; else let yearSign be "-".
    let year_sign = if yv >= 0.0 { "" } else { "-" };
    // 10. Let paddedYear be ToZeroPaddedDecimalString(abs(yv), 4).
    let padded_year = format!("{:04}", yv.abs() as i64);
    // 11. Return the string-concatenation of weekday, ",", the code unit 0x0020 (SPACE), day,
    //     the code unit 0x0020 (SPACE), month, the code unit 0x0020 (SPACE), yearSign,
    //     paddedYear, the code unit 0x0020 (SPACE), and TimeString(tv).
    let s = format!(
        "{weekday}, {day} {month} {year_sign}{padded_year} {}",
        date::time_string(tv)
    );
    Ok(Value::String(agent.string(&s)))
}
