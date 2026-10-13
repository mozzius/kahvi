//! Number.prototype.toPrecision ( _precision_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-number.prototype.toprecision

use kahvi_core::prelude::*;

/// Number.prototype.toPrecision
pub(crate) fn prototype_to_precision(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method returns a String containing this Number value represented either in decimal exponential notation with one digit before the significand's decimal point and _precision_ - 1 digits after the significand's decimal point or in decimal fixed notation with _precision_ significant digits. If _precision_ is *undefined*, it calls ToString instead.
    // It performs the following steps when called:
    //
    // 1. Let _number_ be ? ThisNumberValue(*this* value).
    // 2. If _precision_ is *undefined*, return ! ToString(_number_).
    // 3. Let _precisionCount_ be ? ToIntegerOrInfinity(_precision_).
    // 4. If _number_ is not a finite Number, return Number::toString(_number_, 10).
    // 5. If _precisionCount_ is not an integer, throw a *RangeError* exception.
    // 6. If _precisionCount_ < 1 or _precisionCount_ > 100, throw a *RangeError* exception.
    // 7. Set _number_ to ℝ(_number_).
    // 8. Let _sign_ be the empty String.
    // 9. If _number_ < 0, then
    //    a. Set _sign_ to the code unit 0x002D (HYPHEN-MINUS).
    //    b. Set _number_ to -_number_.
    // 10. If _number_ = 0, then
    //    a. Let _significand_ be the String value consisting of _precisionCount_ occurrences of the code unit 0x0030 (DIGIT ZERO).
    //    b. Let _exponent_ be 0.
    // 11. Else,
    //    a. Let _exponent_ and _intSignificand_ be integers such that 10^_precisionCount_ - 1 ≤ _intSignificand_ < 10^_precisionCount_ and for which _intSignificand_ × 10^_exponent_ - _precisionCount_ + 1 - _number_ is as close to zero as possible. If there are two such sets of _exponent_ and _intSignificand_, pick the _exponent_ and _intSignificand_ for which _intSignificand_ × 10^_exponent_ - _precisionCount_ + 1 is larger.
    //    b. Let _significand_ be the String value consisting of the digits of the decimal representation of _intSignificand_ (in order, with no leading zeroes).
    //    c. If _exponent_ < -6 or _exponent_ ≥ _precisionCount_, then
    //       i. Assert: _exponent_ ≠ 0.
    //       ii. If _precisionCount_ ≠ 1, then
    //          1. Let _intPart_ be the first code unit of _significand_.
    //          2. Let _fractionalPart_ be the other _precisionCount_ - 1 code units of _significand_.
    //          3. Set _significand_ to the string-concatenation of _intPart_, *"."*, and _fractionalPart_.
    //       iii. If _exponent_ > 0, then
    //          1. Let _exponentSign_ be the code unit 0x002B (PLUS SIGN).
    //       iv. Else,
    //          1. Assert: _exponent_ < 0.
    //          2. Let _exponentSign_ be the code unit 0x002D (HYPHEN-MINUS).
    //          3. Set _exponent_ to -_exponent_.
    //       v. Let _exponentDigits_ be the String value consisting of the digits of the decimal representation of _exponent_ (in order, with no leading zeroes).
    //       vi. Return the string-concatenation of _sign_, _significand_, the code unit 0x0065 (LATIN SMALL LETTER E), _exponentSign_, and _exponentDigits_.
    // 12. If _exponent_ = _precisionCount_ - 1, return the string-concatenation of _sign_ and _significand_.
    // 13. If _exponent_ ≥ 0, then
    //    a. Set _significand_ to the string-concatenation of the first _exponent_ + 1 code units of _significand_, the code unit 0x002E (FULL STOP), and the remaining _precisionCount_ - (_exponent_ + 1) code units of _significand_.
    // 14. Else,
    //    a. Set _significand_ to the string-concatenation of the code unit 0x0030 (DIGIT ZERO), the code unit 0x002E (FULL STOP), -(_exponent_ + 1) occurrences of the code unit 0x0030 (DIGIT ZERO), and the String _significand_.
    // 15. Return the string-concatenation of _sign_ and _significand_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Number.prototype.toPrecision"))
}
