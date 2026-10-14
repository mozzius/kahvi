//! Date.prototype.setMonth ( _month_ [ , _day_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.setmonth

use kahvi_core::prelude::*;

/// Date.prototype.setMonth
pub(crate) fn prototype_set_month(
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
    // 4. Let _monthNumber_ be ? ToNumber(_month_).
    // 5. If _day_ is present, let _dayNumber_ be ? ToNumber(_day_).
    // 6. If _tv_ is *NaN*, return *NaN*.
    // 7. Set _tv_ to LocalTime(_tv_).
    // 8. If _day_ is not present, let _dayNumber_ be 𝔽(DateFromTime(_tv_)).
    // 9. Let _newDate_ be MakeDate(MakeDay(𝔽(YearFromTime(_tv_)), _monthNumber_, _dayNumber_), 𝔽(TimeWithinDay(_tv_))).
    // 10. Let _u_ be TimeClip(UTC(_newDate_)).
    // 11. Set _dateObj_.[[DateValue]] to _u_.
    // 12. Return _u_.
    //
    // The *"length"* property of this method is *2*_𝔽.
    // NOTE:
    // If _day_ is not present, this method behaves as if _day_ was present with the value `getDate()`.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.setMonth"))
}
