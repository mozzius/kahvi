//! Date.prototype.setYear ( _year_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.setyear

use kahvi_core::prelude::*;

/// Date.prototype.setYear
pub(crate) fn prototype_set_year(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // The `setFullYear` method is preferred for nearly all purposes, because it avoids the “year 2000 problem.”
    // This method performs the following steps when called:
    //
    // 1. Let _dateObj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
    // 3. Let _time_ be _dateObj_.[[DateValue]].
    // 4. If _time_ is *NaN*, set _time_ to *+0*_𝔽; else set _time_ to LocalTime(_time_).
    // 5. Let _fullYear_ be MakeFullYear(? ToNumber(_year_)).
    // 6. Let _day_ be MakeDay(_fullYear_, 𝔽(MonthFromTime(_time_)), 𝔽(DateFromTime(_time_))).
    // 7. Let _date_ be MakeDate(_day_, 𝔽(TimeWithinDay(_time_))).
    // 8. Let _utcTimestamp_ be TimeClip(UTC(_date_)).
    // 9. Set _dateObj_.[[DateValue]] to _utcTimestamp_.
    // 10. Return _utcTimestamp_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.setYear"))
}
