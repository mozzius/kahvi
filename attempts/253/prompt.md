# Ticket B-number-prototype-to-fixed: Number.prototype.toFixed

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

- `crates/kahvi-builtins/src/number/prototype_to_fixed.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Number.prototype.toFixed ( _fractionDigits_ )
  id: sec-number.prototype.tofixed   type: built-in function

NOTE:
This method returns a String containing this Number value represented in decimal fixed-point notation with _fractionDigits_ digits after the decimal point. If _fractionDigits_ is *undefined*, 0 is assumed.
It performs the following steps when called:

1. Let _number_ be ? ThisNumberValue(*this* value).
2. Let _fractionCount_ be ? ToIntegerOrInfinity(_fractionDigits_).
3. Assert: If _fractionDigits_ is *undefined*, then _fractionCount_ is 0.
4. If _fractionCount_ is not an integer, throw a *RangeError* exception.
5. If _fractionCount_ < 0 or _fractionCount_ > 100, throw a *RangeError* exception.
6. If _number_ is not a finite Number, return Number::toString(_number_, 10).
7. Set _number_ to ℝ(_number_).
8. Let _sign_ be the empty String.
9. If _number_ < 0, then
   a. Set _sign_ to *"-"*.
   b. Set _number_ to -_number_.
10. If _number_ ≥ 10^21, then
   a. Let _digitString_ be ! ToString(𝔽(_number_)).
11. Else,
   a. Let _intValue_ be an integer for which _intValue_ / 10^_fractionCount_ - _number_ is as close to zero as possible. If there are two such _intValue_, pick the larger _intValue_.
   b. If _intValue_ = 0, let _digitString_ be *"0"*; else let _digitString_ be the String value consisting of the digits of the decimal representation of _intValue_ (in order, with no leading zeroes).
   c. If _fractionCount_ ≠ 0, then
      i. Let _digitCount_ be the length of _digitString_.
      ii. If _digitCount_ ≤ _fractionCount_, then
         1. Let _zeroPad_ be the String value consisting of _fractionCount_ + 1 - _digitCount_ occurrences of the code unit 0x0030 (DIGIT ZERO).
         2. Set _digitString_ to the string-concatenation of _zeroPad_ and _digitString_.
         3. Set _digitCount_ to _fractionCount_ + 1.
      iii. Let _intPart_ be the first _digitCount_ - _fractionCount_ code units of _digitString_.
      iv. Let _fractionalPart_ be the other _fractionCount_ code units of _digitString_.
      v. Set _digitString_ to the string-concatenation of _intPart_, *"."*, and _fractionalPart_.
12. Return the string-concatenation of _sign_ and _digitString_.

NOTE:
The output of `toFixed` may be more precise than `toString` for some values because toString only prints enough significant digits to distinguish the number from adjacent Number values. For example,
`(1000000000000000128).toString()` returns *"1000000000000000100"*, while
`(1000000000000000128).toFixed(0)` returns *"1000000000000000128"*.

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


## Done when

- Every test262 run under built-ins/Number/prototype/toFixed/ passes (check with `./t262 built-ins/Number/prototype/toFixed/`). 10 pass on the main branch right now.

