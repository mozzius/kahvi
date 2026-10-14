//! Date.prototype.toDateString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.todatestring

use kahvi_core::prelude::*;

/// Date.prototype.toDateString
pub(crate) fn prototype_to_date_string(
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
    // 4. If _tv_ is *NaN*, return *"Invalid Date"*.
    // 5. Let _t_ be LocalTime(_tv_).
    // 6. Return DateString(_t_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.toDateString"))
}
