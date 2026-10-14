# Ticket B-number-prototype-to-precision: Number.prototype.toPrecision

Implement this built-in in its file, following the spec steps already in
the stub: keep each step as a comment directly above the code for it.

- Abstract operations are in `kahvi_core::ops` (see docs/API.md); use them
  rather than re-implementing. If a helper you need is missing and other
  built-ins would need it too, write it in your file and say so in your
  final report; do not edit other files.
- Anything you cannot finish must still return
  `Err(agent.not_implemented("..."))` naming what is missing. Never return
  a made-up value.
- No new Rust tests: the test262 target is the measure.

## Files you may change

- `crates/kahvi-builtins/src/number/prototype_to_precision.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Number.prototype.toPrecision ( _precision_ )
  id: sec-number.prototype.toprecision   type: built-in function

This method returns a String containing this Number value represented either in decimal exponential notation with one digit before the significand's decimal point and _precision_ - 1 digits after the significand's decimal point or in decimal fixed notation with _precision_ significant digits. If _precision_ is *undefined*, it calls ToString instead.
It performs the following steps when called:

1. Let _number_ be ? ThisNumberValue(*this* value).
2. If _precision_ is *undefined*, return ! ToString(_number_).
3. Let _precisionCount_ be ? ToIntegerOrInfinity(_precision_).
4. If _number_ is not a finite Number, return Number::toString(_number_, 10).
5. If _precisionCount_ is not an integer, throw a *RangeError* exception.
6. If _precisionCount_ < 1 or _precisionCount_ > 100, throw a *RangeError* exception.
7. Set _number_ to ℝ(_number_).
8. Let _sign_ be the empty String.
9. If _number_ < 0, then
   a. Set _sign_ to the code unit 0x002D (HYPHEN-MINUS).
   b. Set _number_ to -_number_.
10. If _number_ = 0, then
   a. Let _significand_ be the String value consisting of _precisionCount_ occurrences of the code unit 0x0030 (DIGIT ZERO).
   b. Let _exponent_ be 0.
11. Else,
   a. Let _exponent_ and _intSignificand_ be integers such that 10^_precisionCount_ - 1 ≤ _intSignificand_ < 10^_precisionCount_ and for which _intSignificand_ × 10^_exponent_ - _precisionCount_ + 1 - _number_ is as close to zero as possible. If there are two such sets of _exponent_ and _intSignificand_, pick the _exponent_ and _intSignificand_ for which _intSignificand_ × 10^_exponent_ - _precisionCount_ + 1 is larger.
   b. Let _significand_ be the String value consisting of the digits of the decimal representation of _intSignificand_ (in order, with no leading zeroes).
   c. If _exponent_ < -6 or _exponent_ ≥ _precisionCount_, then
      i. Assert: _exponent_ ≠ 0.
      ii. If _precisionCount_ ≠ 1, then
         1. Let _intPart_ be the first code unit of _significand_.
         2. Let _fractionalPart_ be the other _precisionCount_ - 1 code units of _significand_.
         3. Set _significand_ to the string-concatenation of _intPart_, *"."*, and _fractionalPart_.
      iii. If _exponent_ > 0, then
         1. Let _exponentSign_ be the code unit 0x002B (PLUS SIGN).
      iv. Else,
         1. Assert: _exponent_ < 0.
         2. Let _exponentSign_ be the code unit 0x002D (HYPHEN-MINUS).
         3. Set _exponent_ to -_exponent_.
      v. Let _exponentDigits_ be the String value consisting of the digits of the decimal representation of _exponent_ (in order, with no leading zeroes).
      vi. Return the string-concatenation of _sign_, _significand_, the code unit 0x0065 (LATIN SMALL LETTER E), _exponentSign_, and _exponentDigits_.
12. If _exponent_ = _precisionCount_ - 1, return the string-concatenation of _sign_ and _significand_.
13. If _exponent_ ≥ 0, then
   a. Set _significand_ to the string-concatenation of the first _exponent_ + 1 code units of _significand_, the code unit 0x002E (FULL STOP), and the remaining _precisionCount_ - (_exponent_ + 1) code units of _significand_.
14. Else,
   a. Set _significand_ to the string-concatenation of the code unit 0x0030 (DIGIT ZERO), the code unit 0x002E (FULL STOP), -(_exponent_ + 1) occurrences of the code unit 0x0030 (DIGIT ZERO), and the String _significand_.
15. Return the string-concatenation of _sign_ and _significand_.

--- referenced: ThisNumberValue ( _arg_: an ECMAScript language value, ): either a normal completion containing a Number or a throw completion
  id: sec-thisnumbervalue

1. If _arg_ is a Number, return _arg_.
2. If _arg_ is an Object and _arg_ has a [[NumberData]] internal slot, then
   a. Let _number_ be _arg_.[[NumberData]].
   b. Assert: _number_ is a Number.
   c. Return _number_.
3. Throw a *TypeError* exception.

--- referenced: ToString ( _arg_: an ECMAScript language value, ): either a normal completion containing a String or a throw completion
  id: sec-tostring

description: It converts _arg_ to a value of type String.

1. If _arg_ is a String, return _arg_.
2. If _arg_ is a Symbol, throw a *TypeError* exception.
3. If _arg_ is *undefined*, return *"undefined"*.
4. If _arg_ is *null*, return *"null"*.
5. If _arg_ is *true*, return *"true"*.
6. If _arg_ is *false*, return *"false"*.
7. If _arg_ is a Number, return Number::toString(_arg_, 10).
8. If _arg_ is a BigInt, return BigInt::toString(_arg_, 10).
9. Assert: _arg_ is an Object.
10. Let _primitiveValue_ be ? ToPrimitive(_arg_, ~string~).
11. Assert: _primitiveValue_ is not an Object.
12. Return ? ToString(_primitiveValue_).

--- referenced: ToIntegerOrInfinity ( _arg_: an ECMAScript language value, ): either a normal completion containing either an integer, +∞, or -∞, or a throw completion
  id: sec-tointegerorinfinity

description: It converts _arg_ to an integer representing its Number value with fractional part truncated, or to +∞ or -∞ when that Number value is infinite.

1. Let _number_ be ? ToNumber(_arg_).
2. If _number_ is one of *NaN*, *+0*_𝔽, or *-0*_𝔽, return 0.
3. If _number_ is *+∞*_𝔽, return +∞.
4. If _number_ is *-∞*_𝔽, return -∞.
5. Return truncate(ℝ(_number_)).

NOTE:
𝔽(ToIntegerOrInfinity(_x_)) never returns *-0*_𝔽 for any value of _x_. The truncation of the fractional part is performed after converting _x_ to a mathematical value.


## Done when

- Every test262 run under built-ins/Number/prototype/toPrecision/ passes (check with `./t262 built-ins/Number/prototype/toPrecision/`). 8 pass on the main branch right now.

