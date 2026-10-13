//! Date.prototype.getTime ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.gettime

use kahvi_core::prelude::*;

/// Date.prototype.getTime
pub(crate) fn prototype_get_time(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _dateObj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
    // 3. Return _dateObj_.[[DateValue]].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.getTime"))
}
