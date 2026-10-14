//! Date.prototype.toUTCString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.toutcstring

use kahvi_core::prelude::*;

/// Date.prototype.toUTCString
pub(crate) fn prototype_to_utc_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method returns a String value representing the instant in time corresponding to the *this* value. The format of the String is based upon HTTP-date from RFC 7231, generalized to support the full range of times supported by ECMAScript Dates.
    // It performs the following steps when called:
    //
    // 1. Let _dateObj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
    // 3. Let _tv_ be _dateObj_.[[DateValue]].
    // 4. If _tv_ is *NaN*, return *"Invalid Date"*.
    // 5. Let _weekday_ be the Name of the entry in [sec-todatestring-day-names] whose WeekDay Index = WeekDay(_tv_).
    // 6. Let _month_ be the Name of the entry in [sec-todatestring-month-names] whose Month Index = MonthFromTime(_tv_).
    // 7. Let _day_ be ToZeroPaddedDecimalString(DateFromTime(_tv_), 2).
    // 8. Let _yv_ be YearFromTime(_tv_).
    // 9. If _yv_ ≥ 0, let _yearSign_ be the empty String; else let _yearSign_ be *"-"*.
    // 10. Let _paddedYear_ be ToZeroPaddedDecimalString(abs(_yv_), 4).
    // 11. Return the string-concatenation of _weekday_, *","*, the code unit 0x0020 (SPACE), _day_, the code unit 0x0020 (SPACE), _month_, the code unit 0x0020 (SPACE), _yearSign_, _paddedYear_, the code unit 0x0020 (SPACE), and TimeString(_tv_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.toUTCString"))
}
