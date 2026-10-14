//! Date.prototype.setUTCFullYear ( _year_ [ , _month_ [ , _day_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.setutcfullyear

use kahvi_core::prelude::*;

/// Date.prototype.setUTCFullYear
pub(crate) fn prototype_set_utc_full_year(
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
    // 4. If _tv_ is *NaN*, set _tv_ to *+0*_𝔽.
    // 5. Let _yearNumber_ be ? ToNumber(_year_).
    // 6. If _month_ is present, let _monthNumber_ be ? ToNumber(_month_); else let _monthNumber_ be 𝔽(MonthFromTime(_tv_)).
    // 7. If _day_ is present, let _dayNumber_ be ? ToNumber(_day_); else let _dayNumber_ be 𝔽(DateFromTime(_tv_)).
    // 8. Let _newDate_ be MakeDate(MakeDay(_yearNumber_, _monthNumber_, _dayNumber_), 𝔽(TimeWithinDay(_tv_))).
    // 9. Let _v_ be TimeClip(_newDate_).
    // 10. Set _dateObj_.[[DateValue]] to _v_.
    // 11. Return _v_.
    //
    // The *"length"* property of this method is *3*_𝔽.
    // NOTE:
    // If _month_ is not present, this method behaves as if _month_ was present with the value `getUTCMonth()`. If _day_ is not present, it behaves as if _day_ was present with the value `getUTCDate()`.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.setUTCFullYear"))
}
