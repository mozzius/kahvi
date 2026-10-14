//! Date.prototype.setMinutes ( _minute_ [ , _second_ [ , _millisecond_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.setminutes

use kahvi_core::prelude::*;

/// Date.prototype.setMinutes
pub(crate) fn prototype_set_minutes(
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
    // 8. Set _tv_ to LocalTime(_tv_).
    // 9. If _second_ is not present, let _secondNumber_ be 𝔽(SecondFromTime(_tv_)).
    // 10. If _millisecond_ is not present, let _millisecondNumber_ be 𝔽(MillisecondFromTime(_tv_)).
    // 11. Let _date_ be MakeDate(𝔽(Day(_tv_)), MakeTime(𝔽(HourFromTime(_tv_)), _minuteNumber_, _secondNumber_, _millisecondNumber_)).
    // 12. Let _u_ be TimeClip(UTC(_date_)).
    // 13. Set _dateObj_.[[DateValue]] to _u_.
    // 14. Return _u_.
    //
    // The *"length"* property of this method is *3*_𝔽.
    // NOTE:
    // If _second_ is not present, this method behaves as if _second_ was present with the value `getSeconds()`. If _millisecond_ is not present, this behaves as if _millisecond_ was present with the value `getMilliseconds()`.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.setMinutes"))
}
