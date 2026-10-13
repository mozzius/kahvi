//! Date.prototype.setUTCDate ( _day_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.setutcdate

use kahvi_core::prelude::*;

/// Date.prototype.setUTCDate
pub(crate) fn prototype_set_utc_date(
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
    // 4. Let _dayNumber_ be ? ToNumber(_day_).
    // 5. If _tv_ is *NaN*, return *NaN*.
    // 6. Let _newDate_ be MakeDate(MakeDay(𝔽(YearFromTime(_tv_)), 𝔽(MonthFromTime(_tv_)), _dayNumber_), 𝔽(TimeWithinDay(_tv_))).
    // 7. Let _v_ be TimeClip(_newDate_).
    // 8. Set _dateObj_.[[DateValue]] to _v_.
    // 9. Return _v_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.setUTCDate"))
}
