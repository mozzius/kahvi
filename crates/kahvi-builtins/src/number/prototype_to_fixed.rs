//! Number.prototype.toFixed ( _fractionDigits_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-number.prototype.tofixed

use kahvi_core::prelude::*;

/// Number.prototype.toFixed
pub(crate) fn prototype_to_fixed(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method returns a String containing this Number value represented in decimal fixed-point notation with _fractionDigits_ digits after the decimal point. If _fractionDigits_ is *undefined*, 0 is assumed.
    // It performs the following steps when called:
    //
    // 1. Let _number_ be ? ThisNumberValue(*this* value).
    // 2. Let _fractionCount_ be ? ToIntegerOrInfinity(_fractionDigits_).
    // 3. Assert: If _fractionDigits_ is *undefined*, then _fractionCount_ is 0.
    // 4. If _fractionCount_ is not an integer, throw a *RangeError* exception.
    // 5. If _fractionCount_ < 0 or _fractionCount_ > 100, throw a *RangeError* exception.
    // 6. If _number_ is not a finite Number, return Number::toString(_number_, 10).
    // 7. Set _number_ to ℝ(_number_).
    // 8. Let _sign_ be the empty String.
    // 9. If _number_ < 0, then
    //    a. Set _sign_ to *"-"*.
    //    b. Set _number_ to -_number_.
    // 10. If _number_ ≥ 10^21, then
    //    a. Let _digitString_ be ! ToString(𝔽(_number_)).
    // 11. Else,
    //    a. Let _intValue_ be an integer for which _intValue_ / 10^_fractionCount_ - _number_ is as close to zero as possible. If there are two such _intValue_, pick the larger _intValue_.
    //    b. If _intValue_ = 0, let _digitString_ be *"0"*; else let _digitString_ be the String value consisting of the digits of the decimal representation of _intValue_ (in order, with no leading zeroes).
    //    c. If _fractionCount_ ≠ 0, then
    //       i. Let _digitCount_ be the length of _digitString_.
    //       ii. If _digitCount_ ≤ _fractionCount_, then
    //          1. Let _zeroPad_ be the String value consisting of _fractionCount_ + 1 - _digitCount_ occurrences of the code unit 0x0030 (DIGIT ZERO).
    //          2. Set _digitString_ to the string-concatenation of _zeroPad_ and _digitString_.
    //          3. Set _digitCount_ to _fractionCount_ + 1.
    //       iii. Let _intPart_ be the first _digitCount_ - _fractionCount_ code units of _digitString_.
    //       iv. Let _fractionalPart_ be the other _fractionCount_ code units of _digitString_.
    //       v. Set _digitString_ to the string-concatenation of _intPart_, *"."*, and _fractionalPart_.
    // 12. Return the string-concatenation of _sign_ and _digitString_.
    //
    // NOTE:
    // The output of `toFixed` may be more precise than `toString` for some values because toString only prints enough significant digits to distinguish the number from adjacent Number values. For example,
    // `(1000000000000000128).toString()` returns *"1000000000000000100"*, while
    // `(1000000000000000128).toFixed(0)` returns *"1000000000000000128"*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Number.prototype.toFixed"))
}
