//! Date.prototype.getUTCMonth ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.getutcmonth

use kahvi_core::prelude::*;

/// Date.prototype.getUTCMonth
pub(crate) fn prototype_get_utc_month(
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
    // 5. Return 𝔽(MonthFromTime(_tv_)).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.getUTCMonth"))
}
