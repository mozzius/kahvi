//! Date.prototype.getTimezoneOffset ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.gettimezoneoffset

use kahvi_core::prelude::*;

/// Date.prototype.getTimezoneOffset
pub(crate) fn prototype_get_timezone_offset(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _dateObj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
    // 3. Let _tv_ be _dateObj_.[[DateValue]].
    // 4. If _tv_ is *NaN*, return *NaN*.
    // 5. Return (_tv_ - LocalTime(_tv_)) / 𝔽(MillisecondsPerMinute).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.getTimezoneOffset"))
}
