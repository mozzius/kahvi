//! Date.prototype.setUTCMilliseconds ( _millisecond_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.setutcmilliseconds

use kahvi_core::prelude::*;

/// Date.prototype.setUTCMilliseconds
pub(crate) fn prototype_set_utc_milliseconds(
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
    // 4. Let _millisecondNumber_ be ? ToNumber(_millisecond_).
    // 5. If _tv_ is *NaN*, return *NaN*.
    // 6. Let _time_ be MakeTime(𝔽(HourFromTime(_tv_)), 𝔽(MinuteFromTime(_tv_)), 𝔽(SecondFromTime(_tv_)), _millisecondNumber_).
    // 7. Let _v_ be TimeClip(MakeDate(𝔽(Day(_tv_)), _time_)).
    // 8. Set _dateObj_.[[DateValue]] to _v_.
    // 9. Return _v_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.setUTCMilliseconds"))
}
