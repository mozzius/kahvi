//! Date.prototype.setSeconds ( _second_ [ , _millisecond_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.setseconds

use kahvi_core::prelude::*;

/// Date.prototype.setSeconds
pub(crate) fn prototype_set_seconds(
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
    // 4. Let _secondNumber_ be ? ToNumber(_second_).
    // 5. If _millisecond_ is present, let _millisecondNumber_ be ? ToNumber(_millisecond_).
    // 6. If _tv_ is *NaN*, return *NaN*.
    // 7. Set _tv_ to LocalTime(_tv_).
    // 8. If _millisecond_ is not present, let _millisecondNumber_ be 𝔽(MillisecondFromTime(_tv_)).
    // 9. Let _date_ be MakeDate(𝔽(Day(_tv_)), MakeTime(𝔽(HourFromTime(_tv_)), 𝔽(MinuteFromTime(_tv_)), _secondNumber_, _millisecondNumber_)).
    // 10. Let _u_ be TimeClip(UTC(_date_)).
    // 11. Set _dateObj_.[[DateValue]] to _u_.
    // 12. Return _u_.
    //
    // The *"length"* property of this method is *2*_𝔽.
    // NOTE:
    // If _millisecond_ is not present, this method behaves as if _millisecond_ was present with the value `getMilliseconds()`.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.setSeconds"))
}
