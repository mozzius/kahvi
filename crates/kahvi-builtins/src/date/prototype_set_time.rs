//! Date.prototype.setTime ( _time_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.settime

use crate::support::date;
use kahvi_core::prelude::*;

/// Date.prototype.setTime
pub(crate) fn prototype_set_time(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _dateObj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
    date::this_time_value(agent, this)?;
    let Value::Object(date_obj) = this else {
        return Err(agent.throw_type_error("this is not a Date object"));
    };
    // 3. Let _t_ be ? ToNumber(_time_).
    let t = ops::to_number(agent, args.get_or_undefined(0))?;
    // 4. Let _v_ be TimeClip(_t_).
    let v = date::time_clip(t);
    // 5. Set _dateObj_.[[DateValue]] to _v_.
    date::set_time_value(agent, date_obj, v);
    // 6. Return _v_.
    Ok(Value::Number(v))
}
