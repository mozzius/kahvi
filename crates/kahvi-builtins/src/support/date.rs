//! Date objects (§21.4): the time value math of §21.4.1, local time
//! through the host's LocalTZA, the Date Time String Format, and the
//! [[DateValue]] slot.
//!
//! Time values are f64 milliseconds since the epoch; NaN is an invalid date.
#![allow(dead_code)] // used by the Date tickets as they land

use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;

pub(crate) const MS_PER_DAY: f64 = 86_400_000.0;
pub(crate) const MS_PER_HOUR: f64 = 3_600_000.0;
pub(crate) const MS_PER_MINUTE: f64 = 60_000.0;
pub(crate) const MS_PER_SECOND: f64 = 1_000.0;

/// [[DateValue]]
#[derive(Debug, Clone, Copy)]
pub(crate) struct DateValue(pub(crate) f64);

/// thisTimeValue(value) (§21.4.4): the [[DateValue]] of a Date object, or a TypeError.
pub(crate) fn this_time_value(agent: &mut Agent, value: Value) -> JsResult<f64> {
    match value {
        Value::Object(o) => match agent.native::<DateValue>(o) {
            Some(d) => Ok(d.0),
            None => Err(agent.throw_type_error("this is not a Date object")),
        },
        _ => Err(agent.throw_type_error("this is not a Date object")),
    }
}

/// Sets the [[DateValue]] of a Date object (after `this_time_value` checked it).
pub(crate) fn set_time_value(agent: &mut Agent, date: Object, t: f64) {
    if let Some(d) = agent.native_mut::<DateValue>(date) {
        d.0 = t;
    }
}

/// A new Date object (OrdinaryCreateFromConstructor(newTarget, "%Date.prototype%", « [[DateValue]] »)).
pub(crate) fn create_date(agent: &mut Agent, new_target: Object, t: f64) -> JsResult<Object> {
    ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%Date.prototype%",
        ObjectKind::Native(Box::new(DateValue(t))),
    )
}

/// The current time value, from the host clock.
pub(crate) fn now(agent: &Agent) -> f64 {
    agent.host_now.map(|f| f()).unwrap_or(f64::NAN)
}

/// 𝔽(floor(x / y)) for the integral divisions of §21.4.1, and the modulo
/// with the sign of the divisor.
fn floor_div(x: f64, y: f64) -> f64 {
    (x / y).floor()
}

fn modulo(x: f64, y: f64) -> f64 {
    let r = x % y;
    if r != 0.0 && (r < 0.0) != (y < 0.0) {
        r + y
    } else {
        r
    }
}

/// Day(t) (§21.4.1.3)
pub(crate) fn day(t: f64) -> f64 {
    floor_div(t, MS_PER_DAY)
}

/// TimeWithinDay(t) (§21.4.1.4)
pub(crate) fn time_within_day(t: f64) -> f64 {
    modulo(t, MS_PER_DAY)
}

/// DaysInYear(y) (§21.4.1.5)
pub(crate) fn days_in_year(y: f64) -> f64 {
    if modulo(y, 4.0) != 0.0 {
        365.0
    } else if modulo(y, 100.0) != 0.0 {
        366.0
    } else if modulo(y, 400.0) != 0.0 {
        365.0
    } else {
        366.0
    }
}

/// DayFromYear(y) (§21.4.1.6)
pub(crate) fn day_from_year(y: f64) -> f64 {
    365.0 * (y - 1970.0) + floor_div(y - 1969.0, 4.0) - floor_div(y - 1901.0, 100.0)
        + floor_div(y - 1601.0, 400.0)
}

/// TimeFromYear(y) (§21.4.1.7)
pub(crate) fn time_from_year(y: f64) -> f64 {
    MS_PER_DAY * day_from_year(y)
}

/// YearFromTime(t) (§21.4.1.8): the largest y with TimeFromYear(y) ≤ t.
pub(crate) fn year_from_time(t: f64) -> f64 {
    let mut y = (t / (MS_PER_DAY * 365.2425)).floor() + 1970.0;
    while time_from_year(y) > t {
        y -= 1.0;
    }
    while time_from_year(y + 1.0) <= t {
        y += 1.0;
    }
    y
}

/// DayWithinYear(t) (§21.4.1.9)
pub(crate) fn day_within_year(t: f64) -> f64 {
    day(t) - day_from_year(year_from_time(t))
}

/// InLeapYear(t) (§21.4.1.10): 1 in a leap year, 0 otherwise.
pub(crate) fn in_leap_year(t: f64) -> f64 {
    if days_in_year(year_from_time(t)) == 366.0 {
        1.0
    } else {
        0.0
    }
}

/// The day of the year each month starts on (non-leap; add InLeapYear from March).
const MONTH_STARTS: [f64; 12] = [
    0.0, 31.0, 59.0, 90.0, 120.0, 151.0, 181.0, 212.0, 243.0, 273.0, 304.0, 334.0,
];

fn month_start(m: usize, leap: f64) -> f64 {
    MONTH_STARTS[m] + if m >= 2 { leap } else { 0.0 }
}

/// MonthFromTime(t) (§21.4.1.11): 0 (January) to 11.
pub(crate) fn month_from_time(t: f64) -> f64 {
    let d = day_within_year(t);
    let leap = in_leap_year(t);
    (0..12)
        .rev()
        .find(|&m| d >= month_start(m, leap))
        .unwrap_or(0) as f64
}

/// DateFromTime(t) (§21.4.1.12): 1 to 31.
pub(crate) fn date_from_time(t: f64) -> f64 {
    let m = month_from_time(t) as usize;
    day_within_year(t) - month_start(m, in_leap_year(t)) + 1.0
}

/// WeekDay(t) (§21.4.1.13): 0 (Sunday) to 6.
pub(crate) fn week_day(t: f64) -> f64 {
    modulo(day(t) + 4.0, 7.0)
}

/// HourFromTime(t) (§21.4.1.14)
pub(crate) fn hour_from_time(t: f64) -> f64 {
    modulo(floor_div(t, MS_PER_HOUR), 24.0)
}

/// MinFromTime(t) (§21.4.1.15)
pub(crate) fn min_from_time(t: f64) -> f64 {
    modulo(floor_div(t, MS_PER_MINUTE), 60.0)
}

/// SecFromTime(t) (§21.4.1.16)
pub(crate) fn sec_from_time(t: f64) -> f64 {
    modulo(floor_div(t, MS_PER_SECOND), 60.0)
}

/// msFromTime(t) (§21.4.1.17)
pub(crate) fn ms_from_time(t: f64) -> f64 {
    modulo(t, MS_PER_SECOND)
}

/// ToIntegerOrInfinity on a Number (the 𝔽(! ToIntegerOrInfinity(x)) of MakeTime and MakeDay).
fn to_int(x: f64) -> f64 {
    ops::integer_or_infinity(x)
}

/// MakeTime(hour, min, sec, ms) (§21.4.1.28)
pub(crate) fn make_time(hour: f64, min: f64, sec: f64, ms: f64) -> f64 {
    // 1. If hour, min, sec or ms is not finite, return NaN.
    if !(hour.is_finite() && min.is_finite() && sec.is_finite() && ms.is_finite()) {
        return f64::NAN;
    }
    // 2-6. The integral parts, combined with IEEE 754 arithmetic.
    let (h, m, s, milli) = (to_int(hour), to_int(min), to_int(sec), to_int(ms));
    // 7. Return ((h × msPerHour + m × msPerMinute) + s × msPerSecond) + milli.
    ((h * MS_PER_HOUR + m * MS_PER_MINUTE) + s * MS_PER_SECOND) + milli
}

/// MakeDay(year, month, date) (§21.4.1.29)
pub(crate) fn make_day(year: f64, month: f64, date: f64) -> f64 {
    // 1. If year, month or date is not finite, return NaN.
    if !(year.is_finite() && month.is_finite() && date.is_finite()) {
        return f64::NAN;
    }
    // 2-4. Let y, m, dt be 𝔽(! ToIntegerOrInfinity(...)).
    let (y, m, dt) = (to_int(year), to_int(month), to_int(date));
    // 5. Let ym be y + 𝔽(floor(ℝ(m) / 12)).
    let ym = y + floor_div(m, 12.0);
    // 6. If ym is not finite, return NaN.
    if !ym.is_finite() {
        return f64::NAN;
    }
    // 7. Let mn be 𝔽(ℝ(m) modulo 12).
    let mn = modulo(m, 12.0) as usize;
    // 8. Find a finite time value t such that YearFromTime(t) is ym, MonthFromTime(t) is mn
    //    and DateFromTime(t) is 1𝔽; if impossible (out of range), return NaN.
    if ym.abs() > 400_000.0 {
        return f64::NAN;
    }
    let leap = if days_in_year(ym) == 366.0 { 1.0 } else { 0.0 };
    let day_of_month_start = day_from_year(ym) + month_start(mn, leap);
    // 9. Return Day(t) + dt - 1𝔽.
    day_of_month_start + dt - 1.0
}

/// MakeDate(day, time) (§21.4.1.30)
pub(crate) fn make_date(day: f64, time: f64) -> f64 {
    // 1. If day is not finite or time is not finite, return NaN.
    if !day.is_finite() || !time.is_finite() {
        return f64::NAN;
    }
    // 2. Let tv be day × msPerDay + time.
    let tv = day * MS_PER_DAY + time;
    // 3. If tv is not finite, return NaN.
    if !tv.is_finite() {
        return f64::NAN;
    }
    tv
}

/// MakeFullYear(year) (§21.4.1.31): two-digit years mean 19xx.
pub(crate) fn make_full_year(year: f64) -> f64 {
    // 1. If year is NaN, return NaN.
    if year.is_nan() {
        return f64::NAN;
    }
    // 2. Let truncated be ! ToIntegerOrInfinity(year).
    let truncated = to_int(year);
    // 3. If truncated is in the inclusive interval from 0 to 99, return 1900𝔽 + 𝔽(truncated).
    if (0.0..=99.0).contains(&truncated) {
        return 1900.0 + truncated;
    }
    // 4. Return 𝔽(truncated).
    truncated
}

/// TimeClip(time) (§21.4.1.32)
pub(crate) fn time_clip(time: f64) -> f64 {
    // 1. If time is not finite, return NaN.
    // 2. If abs(ℝ(time)) > 8.64 × 10^15, return NaN.
    if !time.is_finite() || time.abs() > 8.64e15 {
        return f64::NAN;
    }
    // 3. Return 𝔽(! ToIntegerOrInfinity(time)) (and -0 becomes +0).
    to_int(time) + 0.0
}

/// LocalTZA(t, isUTC) (§21.4.1.25), from the host; UTC without one.
pub(crate) fn local_tza(agent: &Agent, t: f64, is_utc: bool) -> f64 {
    agent.host_local_tza.map(|f| f(t, is_utc)).unwrap_or(0.0)
}

/// LocalTime(t) (§21.4.1.26)
pub(crate) fn local_time(agent: &Agent, t: f64) -> f64 {
    t + local_tza(agent, t, true)
}

/// UTC(t) (§21.4.1.27)
pub(crate) fn utc(agent: &Agent, t: f64) -> f64 {
    // 1. If t is not finite, return NaN.
    if !t.is_finite() {
        return f64::NAN;
    }
    t - local_tza(agent, t, false)
}

const DAY_NAMES: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const MONTH_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

fn year_string(y: f64) -> String {
    if y >= 0.0 {
        format!("{:04}", y as i64)
    } else {
        format!("-{:06}", (-y) as i64)
    }
}

/// TimeString(tv) (§21.4.4.41.1): "HH:mm:ss GMT".
pub(crate) fn time_string(tv: f64) -> String {
    format!(
        "{:02}:{:02}:{:02} GMT",
        hour_from_time(tv) as i64,
        min_from_time(tv) as i64,
        sec_from_time(tv) as i64
    )
}

/// DateString(tv) (§21.4.4.41.2): "Www Mmm DD YYYY".
pub(crate) fn date_string(tv: f64) -> String {
    format!(
        "{} {} {:02} {}",
        DAY_NAMES[week_day(tv) as usize],
        MONTH_NAMES[month_from_time(tv) as usize],
        date_from_time(tv) as i64,
        year_string(year_from_time(tv))
    )
}

/// TimeZoneString(tv) (§21.4.4.41.3): "+HHMM" with no name.
pub(crate) fn time_zone_string(agent: &Agent, tv: f64) -> String {
    let offset = local_tza(agent, tv, true);
    let sign = if offset >= 0.0 { '+' } else { '-' };
    let abs = offset.abs();
    format!(
        "{sign}{:02}{:02}",
        hour_from_time(abs) as i64,
        min_from_time(abs) as i64
    )
}

/// ToDateString(tv) (§21.4.4.41.4)
pub(crate) fn to_date_string(agent: &Agent, tv: f64) -> String {
    // 1. If tv is NaN, return "Invalid Date".
    if tv.is_nan() {
        return "Invalid Date".into();
    }
    // 2. Let t be LocalTime(tv).
    let t = local_time(agent, tv);
    // 3. Return DateString(t), " ", TimeString(t), and TimeZoneString(tv).
    format!(
        "{} {}{}",
        date_string(t),
        time_string(t),
        time_zone_string(agent, tv)
    )
}

/// The Date Time String Format (§21.4.1.32), as produced by toISOString:
/// "YYYY-MM-DDTHH:mm:ss.sssZ" with expanded years for out-of-range ones.
pub(crate) fn iso_string(tv: f64) -> String {
    let y = year_from_time(tv);
    let year = if (0.0..=9999.0).contains(&y) {
        format!("{:04}", y as i64)
    } else if y < 0.0 {
        format!("-{:06}", (-y) as i64)
    } else {
        format!("+{:06}", y as i64)
    };
    format!(
        "{year}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        month_from_time(tv) as i64 + 1,
        date_from_time(tv) as i64,
        hour_from_time(tv) as i64,
        min_from_time(tv) as i64,
        sec_from_time(tv) as i64,
        ms_from_time(tv) as i64
    )
}

/// Parses the Date Time String Format (§21.4.1.32) and its date-only and
/// expanded-year forms; None if `s` does not conform. Date-only forms are
/// UTC; date-time forms without an offset are local time.
pub(crate) fn parse_iso(agent: &Agent, s: &str) -> Option<f64> {
    let b = s.as_bytes();
    let mut i = 0;
    let num = |i: &mut usize, n: usize| -> Option<f64> {
        let part = b.get(*i..*i + n)?;
        if !part.iter().all(u8::is_ascii_digit) {
            return None;
        }
        *i += n;
        std::str::from_utf8(part).ok()?.parse::<f64>().ok()
    };
    // YYYY, or ±YYYYYY (and -000000 is not allowed).
    let year = match b.first()? {
        b'+' | b'-' => {
            let neg = b[0] == b'-';
            i = 1;
            let y = num(&mut i, 6)?;
            if neg && y == 0.0 {
                return None;
            }
            if neg { -y } else { y }
        }
        _ => num(&mut i, 4)?,
    };
    let (mut month, mut date) = (1.0, 1.0);
    if b.get(i) == Some(&b'-') {
        i += 1;
        month = num(&mut i, 2)?;
        if b.get(i) == Some(&b'-') {
            i += 1;
            date = num(&mut i, 2)?;
        }
    }
    let (mut hour, mut minute, mut second, mut ms) = (0.0, 0.0, 0.0, 0.0);
    let mut has_time = false;
    let mut offset: Option<f64> = None;
    if b.get(i) == Some(&b'T') {
        has_time = true;
        i += 1;
        hour = num(&mut i, 2)?;
        if b.get(i) != Some(&b':') {
            return None;
        }
        i += 1;
        minute = num(&mut i, 2)?;
        if b.get(i) == Some(&b':') {
            i += 1;
            second = num(&mut i, 2)?;
            if b.get(i) == Some(&b'.') {
                i += 1;
                let start = i;
                while b.get(i).is_some_and(u8::is_ascii_digit) {
                    i += 1;
                }
                if i == start {
                    return None;
                }
                let digits = &s[start..i.min(start + 3)];
                ms = format!("{digits:0<3}").parse::<f64>().ok()?;
            }
        }
        match b.get(i) {
            Some(b'Z') => {
                offset = Some(0.0);
                i += 1;
            }
            Some(&c @ (b'+' | b'-')) => {
                i += 1;
                let oh = num(&mut i, 2)?;
                if b.get(i) != Some(&b':') {
                    return None;
                }
                i += 1;
                let om = num(&mut i, 2)?;
                if oh > 23.0 || om > 59.0 {
                    return None;
                }
                let o = oh * MS_PER_HOUR + om * MS_PER_MINUTE;
                offset = Some(if c == b'+' { o } else { -o });
            }
            _ => {}
        }
    }
    if i != b.len() {
        return None;
    }
    // Field ranges ("24:00" is allowed only as the end of a day).
    let leap = days_in_year(year) == 366.0;
    let days_in_month = [
        31.0,
        if leap { 29.0 } else { 28.0 },
        31.0,
        30.0,
        31.0,
        30.0,
        31.0,
        31.0,
        30.0,
        31.0,
        30.0,
        31.0,
    ];
    if !(1.0..=12.0).contains(&month) || date < 1.0 || date > days_in_month[month as usize - 1] {
        return None;
    }
    if minute > 59.0
        || second > 59.0
        || hour > 24.0
        || (hour == 24.0 && (minute > 0.0 || second > 0.0 || ms > 0.0))
    {
        return None;
    }
    let t = make_date(
        make_day(year, month - 1.0, date),
        make_time(hour, minute, second, ms),
    );
    let t = match offset {
        Some(o) => t - o,
        None if has_time => utc(agent, t),
        None => t,
    };
    let t = time_clip(t);
    (!t.is_nan()).then_some(t)
}

impl kahvi_core::gc::Trace for DateValue {
    fn trace(&self, _t: &mut kahvi_core::gc::Tracer) {}
}
