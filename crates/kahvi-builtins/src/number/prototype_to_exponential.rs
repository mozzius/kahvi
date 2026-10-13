//! Number.prototype.toExponential ( _fractionDigits_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-number.prototype.toexponential

use kahvi_core::prelude::*;

/// Number.prototype.toExponential
pub(crate) fn prototype_to_exponential(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method returns a String containing this Number value represented in decimal exponential notation with one digit before the significand's decimal point and _fractionDigits_ digits after the significand's decimal point. If _fractionDigits_ is *undefined*, it includes as many significand digits as necessary to uniquely specify the Number (just like in ToString except that in this case the Number is always output in exponential notation).
    // It performs the following steps when called:
    //
    // 1. Let _number_ be ? ThisNumberValue(*this* value).
    // 2. Let _fractionCount_ be ? ToIntegerOrInfinity(_fractionDigits_).
    // 3. Assert: If _fractionDigits_ is *undefined*, then _fractionCount_ is 0.
    // 4. If _number_ is not a finite Number, return Number::toString(_number_, 10).
    // 5. If _fractionCount_ is not an integer, throw a *RangeError* exception.
    // 6. If _fractionCount_ < 0 or _fractionCount_ > 100, throw a *RangeError* exception.
    // 7. Set _number_ to ℝ(_number_).
    // 8. Let _sign_ be the empty String.
    // 9. If _number_ < 0, then
    //    a. Set _sign_ to *"-"*.
    //    b. Set _number_ to -_number_.
    // 10. If _number_ = 0, then
    //    a. Let _significand_ be the String value consisting of _fractionCount_ + 1 occurrences of the code unit 0x0030 (DIGIT ZERO).
    //    b. Let _exponent_ be 0.
    // 11. Else,
    //    a. If _fractionDigits_ is not *undefined*, then
    //       i. Let _exponent_ and _intSignificand_ be integers such that 10^_fractionCount_ ≤ _intSignificand_ < 10^_fractionCount_ + 1 and for which _intSignificand_ × 10^_exponent_ - _fractionCount_ - _number_ is as close to zero as possible. If there are two such sets of _exponent_ and _intSignificand_, pick the _exponent_ and _intSignificand_ for which _intSignificand_ × 10^_exponent_ - _fractionCount_ is larger.
    //    b. Else,
    //       i. Let _exponent_, _intSignificand_, and _ff_ be integers such that _ff_ ≥ 0, 10^_ff_ ≤ _intSignificand_ < 10^_ff_ + 1, 𝔽(_intSignificand_ × 10^_exponent_ - _ff_) is 𝔽(_number_), and _ff_ is as small as possible. Note that the decimal representation of _intSignificand_ has _ff_ + 1 digits, _intSignificand_ is not divisible by 10, and the least significant digit of _intSignificand_ is not necessarily uniquely determined by these criteria.
    //       ii. Set _fractionCount_ to _ff_.
    //    c. Let _significand_ be the String value consisting of the digits of the decimal representation of _intSignificand_ (in order, with no leading zeroes).
    // 12. If _fractionCount_ ≠ 0, then
    //    a. Let _intPart_ be the first code unit of _significand_.
    //    b. Let _fractionalPart_ be the other _fractionCount_ code units of _significand_.
    //    c. Set _significand_ to the string-concatenation of _intPart_, *"."*, and _fractionalPart_.
    // 13. If _exponent_ = 0, then
    //    a. Let _exponentSign_ be *"+"*.
    //    b. Let _exponentDigits_ be *"0"*.
    // 14. Else,
    //    a. If _exponent_ > 0, then
    //       i. Let _exponentSign_ be *"+"*.
    //    b. Else,
    //       i. Assert: _exponent_ < 0.
    //       ii. Let _exponentSign_ be *"-"*.
    //       iii. Set _exponent_ to -_exponent_.
    //    c. Let _exponentDigits_ be the String value consisting of the digits of the decimal representation of _exponent_ (in order, with no leading zeroes).
    // 15. Set _significand_ to the string-concatenation of _significand_, *"e"*, _exponentSign_, and _exponentDigits_.
    // 16. Return the string-concatenation of _sign_ and _significand_.
    //
    // NOTE:
    // For implementations that provide more accurate conversions than required by the rules above, it is recommended that the following alternative version of step [step-number-proto-toexponential-intermediate-values] be used as a guideline:
    //
    // 1. Let _exponent_, _intSignificand_, and _ff_ be integers such that _ff_ ≥ 0, 10^_ff_ ≤ _intSignificand_ < 10^_ff_ + 1, 𝔽(_intSignificand_ × 10^_exponent_ - _ff_) is 𝔽(_number_), and _ff_ is as small as possible. If there are multiple possibilities for _intSignificand_, choose _intSignificand_ such that 𝔽(_intSignificand_ × 10^_exponent_ - _ff_) is closest to 𝔽(_number_). If there are two such possible values of _intSignificand_, choose the one that is even.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Number.prototype.toExponential"))
}
