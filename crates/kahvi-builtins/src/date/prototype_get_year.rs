//! Date.prototype.getYear ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.getyear

use kahvi_core::prelude::*;

/// Date.prototype.getYear
pub(crate) fn prototype_get_year(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // The `getFullYear` method is preferred for nearly all purposes, because it avoids the “year 2000 problem.”
    // This method performs the following steps when called:
    //
    // 1. Let _dateObj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
    // 3. Let _tv_ be _dateObj_.[[DateValue]].
    // 4. If _tv_ is *NaN*, return *NaN*.
    // 5. Return 𝔽(YearFromTime(LocalTime(_tv_))) - *1900*_𝔽.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.getYear"))
}
