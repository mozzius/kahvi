//! Date.prototype.setUTCMinutes ( _minute_ [ , _second_ [ , _millisecond_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.setutcminutes

use kahvi_core::prelude::*;

/// Date.prototype.setUTCMinutes
pub(crate) fn prototype_set_utc_minutes(
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
    // 4. Let _minuteNumber_ be ? ToNumber(_minute_).
    // 5. If _second_ is present, let _secondNumber_ be ? ToNumber(_second_).
    // 6. If _millisecond_ is present, let _millisecondNumber_ be ? ToNumber(_millisecond_).
    // 7. If _tv_ is *NaN*, return *NaN*.
    // 8. If _second_ is not present, let _secondNumber_ be 𝔽(SecondFromTime(_tv_)).
    // 9. If _millisecond_ is not present, let _millisecondNumber_ be 𝔽(MillisecondFromTime(_tv_)).
    // 10. Let _date_ be MakeDate(𝔽(Day(_tv_)), MakeTime(𝔽(HourFromTime(_tv_)), _minuteNumber_, _secondNumber_, _millisecondNumber_)).
    // 11. Let _v_ be TimeClip(_date_).
    // 12. Set _dateObj_.[[DateValue]] to _v_.
    // 13. Return _v_.
    //
    // The *"length"* property of this method is *3*_𝔽.
    // NOTE:
    // If _second_ is not present, this method behaves as if _second_ was present with the value `getUTCSeconds()`. If _millisecond_ is not present, it behaves as if _millisecond_ was present with the value return by `getUTCMilliseconds()`.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.setUTCMinutes"))
}
