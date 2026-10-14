//! Number.prototype.toPrecision ( _precision_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-number.prototype.toprecision

use crate::support::this_value::this_number_value;
use kahvi_core::prelude::*;

/// Number.prototype.toPrecision
pub(crate) fn prototype_to_precision(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _number_ be ? ThisNumberValue(*this* value).
    let mut number = this_number_value(agent, this)?;
    let precision = args.get_or_undefined(0);
    // 2. If _precision_ is *undefined*, return ! ToString(_number_).
    if precision.is_undefined() {
        return Ok(Value::String(ops::number_to_js_string(agent, number)));
    }
    // 3. Let _precisionCount_ be ? ToIntegerOrInfinity(_precision_).
    let precision_count = ops::to_integer_or_infinity(agent, precision)?;
    // 4. If _number_ is not a finite Number, return Number::toString(_number_, 10).
    if !number.is_finite() {
        return Ok(Value::String(ops::number_to_js_string(agent, number)));
    }
    // 5. If _precisionCount_ is not an integer, throw a *RangeError* exception.
    if !ops::is_integral_number(Value::Number(precision_count)) {
        return Err(agent.throw_range_error("toPrecision() argument must be between 1 and 100"));
    }
    // 6. If _precisionCount_ < 1 or _precisionCount_ > 100, throw a *RangeError* exception.
    if !(1.0..=100.0).contains(&precision_count) {
        return Err(agent.throw_range_error("toPrecision() argument must be between 1 and 100"));
    }
    // `precision_count` is now an integer in 1..=100.
    let p = precision_count as usize;
    // 7. Set _number_ to ℝ(_number_).
    // (A finite Number is already a real value here; the decimal digits are taken from its exact expansion below.)
    // 8. Let _sign_ be the empty String.
    let mut sign = String::new();
    // 9. If _number_ < 0, then
    if number < 0.0 {
        // a. Set _sign_ to the code unit 0x002D (HYPHEN-MINUS).
        sign.push('-');
        // b. Set _number_ to -_number_.
        number = -number;
    }
    let (mut significand, exponent) = if number == 0.0 {
        // 10. If _number_ = 0, then
        // a. Let _significand_ be the String value consisting of _precisionCount_ occurrences of the code unit 0x0030 (DIGIT ZERO).
        // b. Let _exponent_ be 0.
        ("0".repeat(p), 0_i32)
    } else {
        // 11. Else,
        // a. Let _exponent_ and _intSignificand_ be integers such that 10^_precisionCount_ - 1 ≤ _intSignificand_ < 10^_precisionCount_ and for which _intSignificand_ × 10^_exponent_ - _precisionCount_ + 1 - _number_ is as close to zero as possible. If there are two such sets of _exponent_ and _intSignificand_, pick the _exponent_ and _intSignificand_ for which _intSignificand_ × 10^_exponent_ - _precisionCount_ + 1 is larger.
        let (digits, exponent) = round_to_precision(number, p);
        // b. Let _significand_ be the String value consisting of the digits of the decimal representation of _intSignificand_ (in order, with no leading zeroes).
        let significand: String = digits.iter().map(|&d| char::from(b'0' + d)).collect();
        (significand, exponent)
    };
    // 11.c. If _exponent_ < -6 or _exponent_ ≥ _precisionCount_, then
    if number != 0.0 && (exponent < -6 || exponent >= p as i32) {
        // i. Assert: _exponent_ ≠ 0.
        debug_assert!(exponent != 0);
        // ii. If _precisionCount_ ≠ 1, then
        if p != 1 {
            // 1. Let _intPart_ be the first code unit of _significand_.
            // 2. Let _fractionalPart_ be the other _precisionCount_ - 1 code units of _significand_.
            // 3. Set _significand_ to the string-concatenation of _intPart_, *"."*, and _fractionalPart_.
            let (int_part, fractional_part) = significand.split_at(1);
            significand = format!("{int_part}.{fractional_part}");
        }
        let (exponent_sign, exponent_digits) = if exponent > 0 {
            // iii. If _exponent_ > 0, then
            // 1. Let _exponentSign_ be the code unit 0x002B (PLUS SIGN).
            ('+', exponent)
        } else {
            // iv. Else,
            // 1. Assert: _exponent_ < 0.
            debug_assert!(exponent < 0);
            // 2. Let _exponentSign_ be the code unit 0x002D (HYPHEN-MINUS).
            // 3. Set _exponent_ to -_exponent_.
            ('-', -exponent)
        };
        // v. Let _exponentDigits_ be the String value consisting of the digits of the decimal representation of _exponent_ (in order, with no leading zeroes).
        // vi. Return the string-concatenation of _sign_, _significand_, the code unit 0x0065 (LATIN SMALL LETTER E), _exponentSign_, and _exponentDigits_.
        let result = format!("{sign}{significand}e{exponent_sign}{exponent_digits}");
        return Ok(Value::String(agent.string(&result)));
    }
    // 12. If _exponent_ = _precisionCount_ - 1, return the string-concatenation of _sign_ and _significand_.
    if exponent == p as i32 - 1 {
        let result = format!("{sign}{significand}");
        return Ok(Value::String(agent.string(&result)));
    }
    // 13. If _exponent_ ≥ 0, then
    if exponent >= 0 {
        // a. Set _significand_ to the string-concatenation of the first _exponent_ + 1 code units of _significand_, the code unit 0x002E (FULL STOP), and the remaining _precisionCount_ - (_exponent_ + 1) code units of _significand_.
        let (int_part, fractional_part) = significand.split_at((exponent + 1) as usize);
        significand = format!("{int_part}.{fractional_part}");
    } else {
        // 14. Else,
        // a. Set _significand_ to the string-concatenation of the code unit 0x0030 (DIGIT ZERO), the code unit 0x002E (FULL STOP), -(_exponent_ + 1) occurrences of the code unit 0x0030 (DIGIT ZERO), and the String _significand_.
        significand = format!("0.{}{significand}", "0".repeat((-(exponent + 1)) as usize));
    }
    // 15. Return the string-concatenation of _sign_ and _significand_.
    let result = format!("{sign}{significand}");
    Ok(Value::String(agent.string(&result)))
}

/// Finds the _exponent_ and _intSignificand_ of step 11.a for a positive finite
/// _number_ and returns the decimal digits of _intSignificand_ (exactly
/// _precision_ of them) together with _exponent_.
///
/// The exact decimal expansion of the double is rounded half-up, so a tie
/// picks the larger _intSignificand_ × 10^_exponent_ as the spec requires.
fn round_to_precision(number: f64, precision: usize) -> (Vec<u8>, i32) {
    // Every finite double has an exact decimal expansion of at most 767
    // significant digits, so 800 digits after the point are exact.
    let formatted = format!("{number:.800e}");
    let Some((mantissa, exponent)) = formatted.split_once('e') else {
        unreachable!("exponential formatting always contains an 'e'");
    };
    let Ok(mut exponent) = exponent.parse::<i32>() else {
        unreachable!("exponential formatting always has an integer exponent");
    };
    let digits: Vec<u8> = mantissa
        .bytes()
        .filter(u8::is_ascii_digit)
        .map(|b| b - b'0')
        .collect();
    let mut out: Vec<u8> = digits.iter().take(precision).copied().collect();
    // Round up when the first dropped digit is 5 or more (ties go to the larger value).
    let mut carry = digits.get(precision).is_some_and(|&d| d >= 5);
    for d in out.iter_mut().rev() {
        if !carry {
            break;
        }
        if *d == 9 {
            *d = 0;
        } else {
            *d += 1;
            carry = false;
        }
    }
    if carry {
        // All digits were 9: the significand becomes 10^(precision-1) and the exponent grows.
        out.insert(0, 1);
        out.truncate(precision);
        exponent += 1;
    }
    (out, exponent)
}
