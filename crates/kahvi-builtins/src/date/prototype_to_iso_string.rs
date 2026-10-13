//! Date.prototype.toISOString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.toisostring

use kahvi_core::prelude::*;

/// Date.prototype.toISOString
pub(crate) fn prototype_to_iso_string(
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
    // 4. If _tv_ is *NaN*, throw a *RangeError* exception.
    // 5. Assert: _tv_ is an integral Number.
    // 6. If _tv_ corresponds with a year that cannot be represented in the Date Time String Format, throw a *RangeError* exception.
    // 7. Return a String representation of _tv_ in the Date Time String Format on the UTC time scale, including all format elements and the UTC offset representation *"Z"*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.toISOString"))
}
