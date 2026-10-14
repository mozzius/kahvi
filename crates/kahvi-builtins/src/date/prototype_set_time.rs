//! Date.prototype.setTime ( _time_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.settime

use kahvi_core::prelude::*;

/// Date.prototype.setTime
pub(crate) fn prototype_set_time(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _dateObj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
    // 3. Let _t_ be ? ToNumber(_time_).
    // 4. Let _v_ be TimeClip(_t_).
    // 5. Set _dateObj_.[[DateValue]] to _v_.
    // 6. Return _v_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.setTime"))
}
