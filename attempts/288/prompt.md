# Ticket B-number-prototype-to-exponential: Number.prototype.toExponential

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

- `crates/kahvi-builtins/src/number/prototype_to_exponential.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Number.prototype.toExponential ( _fractionDigits_ )
  id: sec-number.prototype.toexponential   type: built-in function

This method returns a String containing this Number value represented in decimal exponential notation with one digit before the significand's decimal point and _fractionDigits_ digits after the significand's decimal point. If _fractionDigits_ is *undefined*, it includes as many significand digits as necessary to uniquely specify the Number (just like in ToString except that in this case the Number is always output in exponential notation).
It performs the following steps when called:

1. Let _number_ be ? ThisNumberValue(*this* value).
2. Let _fractionCount_ be ? ToIntegerOrInfinity(_fractionDigits_).
3. Assert: If _fractionDigits_ is *undefined*, then _fractionCount_ is 0.
4. If _number_ is not a finite Number, return Number::toString(_number_, 10).
5. If _fractionCount_ is not an integer, throw a *RangeError* exception.
6. If _fractionCount_ < 0 or _fractionCount_ > 100, throw a *RangeError* exception.
7. Set _number_ to ℝ(_number_).
8. Let _sign_ be the empty String.
9. If _number_ < 0, then
   a. Set _sign_ to *"-"*.
   b. Set _number_ to -_number_.
10. If _number_ = 0, then
   a. Let _significand_ be the String value consisting of _fractionCount_ + 1 occurrences of the code unit 0x0030 (DIGIT ZERO).
   b. Let _exponent_ be 0.
11. Else,
   a. If _fractionDigits_ is not *undefined*, then
      i. Let _exponent_ and _intSignificand_ be integers such that 10^_fractionCount_ ≤ _intSignificand_ < 10^_fractionCount_ + 1 and for which _intSignificand_ × 10^_exponent_ - _fractionCount_ - _number_ is as close to zero as possible. If there are two such sets of _exponent_ and _intSignificand_, pick the _exponent_ and _intSignificand_ for which _intSignificand_ × 10^_exponent_ - _fractionCount_ is larger.
   b. Else,
      i. Let _exponent_, _intSignificand_, and _ff_ be integers such that _ff_ ≥ 0, 10^_ff_ ≤ _intSignificand_ < 10^_ff_ + 1, 𝔽(_intSignificand_ × 10^_exponent_ - _ff_) is 𝔽(_number_), and _ff_ is as small as possible. Note that the decimal representation of _intSignificand_ has _ff_ + 1 digits, _intSignificand_ is not divisible by 10, and the least significant digit of _intSignificand_ is not necessarily uniquely determined by these criteria.
      ii. Set _fractionCount_ to _ff_.
   c. Let _significand_ be the String value consisting of the digits of the decimal representation of _intSignificand_ (in order, with no leading zeroes).
12. If _fractionCount_ ≠ 0, then
   a. Let _intPart_ be the first code unit of _significand_.
   b. Let _fractionalPart_ be the other _fractionCount_ code units of _significand_.
   c. Set _significand_ to the string-concatenation of _intPart_, *"."*, and _fractionalPart_.
13. If _exponent_ = 0, then
   a. Let _exponentSign_ be *"+"*.
   b. Let _exponentDigits_ be *"0"*.
14. Else,
   a. If _exponent_ > 0, then
      i. Let _exponentSign_ be *"+"*.
   b. Else,
      i. Assert: _exponent_ < 0.
      ii. Let _exponentSign_ be *"-"*.
      iii. Set _exponent_ to -_exponent_.
   c. Let _exponentDigits_ be the String value consisting of the digits of the decimal representation of _exponent_ (in order, with no leading zeroes).
15. Set _significand_ to the string-concatenation of _significand_, *"e"*, _exponentSign_, and _exponentDigits_.
16. Return the string-concatenation of _sign_ and _significand_.

NOTE:
For implementations that provide more accurate conversions than required by the rules above, it is recommended that the following alternative version of step [step-number-proto-toexponential-intermediate-values] be used as a guideline:

1. Let _exponent_, _intSignificand_, and _ff_ be integers such that _ff_ ≥ 0, 10^_ff_ ≤ _intSignificand_ < 10^_ff_ + 1, 𝔽(_intSignificand_ × 10^_exponent_ - _ff_) is 𝔽(_number_), and _ff_ is as small as possible. If there are multiple possibilities for _intSignificand_, choose _intSignificand_ such that 𝔽(_intSignificand_ × 10^_exponent_ - _ff_) is closest to 𝔽(_number_). If there are two such possible values of _intSignificand_, choose the one that is even.

--- referenced: ThisNumberValue ( _arg_: an ECMAScript language value, ): either a normal completion containing a Number or a throw completion
  id: sec-thisnumbervalue

1. If _arg_ is a Number, return _arg_.
2. If _arg_ is an Object and _arg_ has a [[NumberData]] internal slot, then
   a. Let _number_ be _arg_.[[NumberData]].
   b. Assert: _number_ is a Number.
   c. Return _number_.
3. Throw a *TypeError* exception.

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

- Every test262 run under built-ins/Number/prototype/toExponential/ passes (check with `./t262 built-ins/Number/prototype/toExponential/`). 8 pass on the main branch right now.

