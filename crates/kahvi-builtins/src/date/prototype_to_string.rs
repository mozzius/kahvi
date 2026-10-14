//! Date.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.tostring

use kahvi_core::prelude::*;

/// Date.prototype.toString
pub(crate) fn prototype_to_string(
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
    // 4. Return ToDateString(_tv_).
    //
    // NOTE:
    // For any Date `d` such that `d.[[DateValue]]` is evenly divisible by 1000, the result of `Date.parse(d.toString())` = `d.valueOf()`. See [sec-date.parse].
    // NOTE:
    // This method is not generic; it throws a *TypeError* exception if its *this* value is not a Date. Therefore, it cannot be transferred to other kinds of objects for use as a method.
    // [subclause sec-timestring: TimeString (
    // _tv_: a Number, but not *NaN*,
    // ): a String]
    // [subclause sec-datestring: DateString (
    // _tv_: a Number, but not *NaN*,
    // ): a String]
    // [subclause sec-timezoneestring: TimeZoneString (
    // _tv_: an integral Number,
    // ): a String]
    // [subclause sec-todatestring: ToDateString (
    // _tv_: an integral Number or *NaN*,
    // ): a String]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.toString"))
}
