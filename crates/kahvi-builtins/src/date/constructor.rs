//! Date ( ..._values_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-date

use crate::support::date;
use kahvi_core::prelude::*;

/// Date
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If NewTarget is undefined, then
    let Some(new_target) = new_target else {
        // a. Let now be the time value (UTC) identifying the current time.
        let now = date::now(agent);
        // b. Return ToDateString(now).
        let s = date::to_date_string(agent, now);
        return Ok(Value::String(agent.string(&s)));
    };
    // 2. Let numberOfArgs be the number of elements in values.
    let dv = match args {
        // 3. If numberOfArgs = 0, then let dv be the time value (UTC) identifying the current time.
        [] => date::now(agent),
        // 4. Else if numberOfArgs = 1, then
        [value] => {
            // a-b. If value is an Object with a [[DateValue]] internal slot, let tv be value.[[DateValue]].
            let existing = match value {
                Value::Object(o) => agent.native::<date::DateValue>(*o).map(|d| d.0),
                _ => None,
            };
            let tv = match existing {
                Some(tv) => tv,
                None => {
                    // c.i Let v be ? ToPrimitive(value).
                    let v = ops::to_primitive(agent, *value, ops::PreferredType::Default)?;
                    match v {
                        // c.ii If v is a String, then let tv be the result of parsing v as a date (Date.parse).
                        Value::String(s) => {
                            let text = agent.str_lossy(s);
                            super::parse::parse_time_value(agent, &text)
                        }
                        // c.iii Else, let tv be ? ToNumber(v).
                        v => ops::to_number(agent, v)?,
                    }
                }
            };
            // d. Let dv be TimeClip(tv).
            date::time_clip(tv)
        }
        // 5. Else,
        values => {
            // b-h. Convert year, month and the optional parts, in order.
            let y = ops::to_number(agent, values[0])?;
            let m = ops::to_number(agent, values[1])?;
            let mut rest = [1.0, 0.0, 0.0, 0.0, 0.0];
            for (i, slot) in rest.iter_mut().enumerate() {
                if let Some(&v) = values.get(i + 2) {
                    *slot = ops::to_number(agent, v)?;
                }
            }
            let [dt, h, min, s, milli] = rest;
            // i. Let yr be MakeFullYear(y).
            let yr = date::make_full_year(y);
            // j. Let finalDate be MakeDate(MakeDay(yr, m, dt), MakeTime(h, min, s, milli)).
            let final_date =
                date::make_date(date::make_day(yr, m, dt), date::make_time(h, min, s, milli));
            // k. Let dv be TimeClip(UTC(finalDate)).
            date::time_clip(date::utc(agent, final_date))
        }
    };
    // 6. Let O be ? OrdinaryCreateFromConstructor(NewTarget, "%Date.prototype%", « [[DateValue]] »).
    // 7. Set O.[[DateValue]] to dv.
    let o = date::create_date(agent, new_target, dv)?;
    // 8. Return O.
    Ok(Value::Object(o))
}
