# Ticket I-number-format: Intl.NumberFormat formatting

The constructor, SetNumberFormatDigitOptions and resolvedOptions are done.
This ticket is the formatting: the `format` getter, FormatNumeric,
`formatToParts`, ToIntlMathematicalValue and FormatNumericToString,
following the spec steps already in each stub (keep each step as a comment
directly above the code for it). Read docs/INTL.md first.

- An Intl mathematical value is exact: keep it as a decimal
  (`icu::decimal::input::Decimal`, which parses strings and BigInts), never
  as an f64 once ToIntlMathematicalValue has read a string or BigInt.
- FormatNumericToString, ToRawPrecision and ToRawFixed are spec
  algorithms on decimals: implement them as written, with the rounding
  modes and increments from the slots. Make FormatNumericToString
  `pub(crate)` in `support/intl_number_format.rs` (Intl.PluralRules uses it).
- PartitionNumberPattern takes its patterns and symbols from ICU4X
  (`icu::decimal::DecimalFormatter` for digits, grouping and signs; the
  `icu::experimental` currency, percent, unit and compact formatters for
  the other styles and notations). Map ICU4X's parts to the spec's part
  types for formatToParts. Whatever ICU4X has no data for throws
  not-implemented naming it.
- Never write tables of expected outputs. No new Rust tests.

## Files you may change

- `crates/kahvi-builtins/src/intl_number_format/*.rs`
- `crates/kahvi-builtins/src/support/intl_number_format.rs`

## Read first

- `docs/INTL.md`
- `docs/API.md`
- `crates/kahvi-builtins/src/support/intl.rs`
- `crates/kahvi-builtins/src/intl_number_format/constructor.rs`

## Spec

get Intl.NumberFormat.prototype.format
  id: sec-intl.numberformat.prototype.format   type: built-in function

Intl.NumberFormat.prototype.format is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps:

1. Let _nf_ be the *this* value.
2. If the implementation supports the normative optional constructor mode of [legacy-constructor], then
   a. Set _nf_ to ? UnwrapNumberFormat(_nf_).
3. Perform ? RequireInternalSlot(_nf_, [[InitializedNumberFormat]]).
4. If _nf_.[[BoundFormat]] is *undefined*, then
   a. Let _closure_ be a new Abstract Closure with parameters (_value_) that captures _nf_ and performs the following steps when called:
      i. Let _x_ be ? ToIntlMathematicalValue(_value_).
      ii. Return FormatNumeric(_nf_, _x_).
   b. Set _nf_.[[BoundFormat]] to CreateBuiltinFunction(_closure_, 1, *""*, « »).
5. Return _nf_.[[BoundFormat]].

NOTE:
The returned function is bound to _nf_ so that it can be passed directly to `Array.prototype.map` or other functions.
This is considered a historical artefact, as part of a convention which is no longer followed for new features, but is preserved to maintain compatibility with existing programs.

--- referenced: UnwrapNumberFormat ( _nf_: an ECMAScript language value, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-unwrapnumberformat

description:
It returns the NumberFormat instance of its input object, which is either the value itself or a value associated with it by %Intl.NumberFormat% according to the normative optional constructor mode of [legacy-constructor].

1. If _nf_ is not an Object, throw a *TypeError* exception.
2. If _nf_ does not have an [[InitializedNumberFormat]] internal slot and ? OrdinaryHasInstance(%Intl.NumberFormat%, _nf_) is *true*, then
   a. Return ? Get(_nf_, %Intl%.[[FallbackSymbol]]).
3. Return _nf_.

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: ToIntlMathematicalValue ( _value_: an ECMAScript language value, ): either a normal completion containing an Intl mathematical value or a throw completion
  id: sec-tointlmathematicalvalue

description:
It returns _value_ converted to an Intl mathematical value, which is a mathematical value together with ~positive-infinity~, ~negative-infinity~, ~not-a-number~, and ~negative-zero~.
This abstract operation is similar to [sec-tonumeric], but a mathematical value can be returned instead of a Number or BigInt, so that exact decimal values can be represented.

1. Let _primValue_ be ? ToPrimitive(_value_, ~number~).
2. If _primValue_ is a BigInt, return ℝ(_primValue_).
3. If _primValue_ is a String, then
   a. Let _str_ be _primValue_.
4. Else,
   a. Let _x_ be ? ToNumber(_primValue_).
   b. If _x_ is *-0*_𝔽, return ~negative-zero~.
   c. Let _str_ be Number::toString(_x_, 10).
5. Let _text_ be StringToCodePoints(_str_).
6. Let _literal_ be ParseText(_text_, |StringNumericLiteral|).
7. If _literal_ is a List of errors, return ~not-a-number~.
8. Let _intlMV_ be the StringIntlMV of _literal_.
9. If _intlMV_ is a mathematical value, then
   a. Let _rounded_ be RoundMVResult(abs(_intlMV_)).
   b. If _rounded_ is *+∞*_𝔽 and _intlMV_ < 0, return ~negative-infinity~.
   c. If _rounded_ is *+∞*_𝔽, return ~positive-infinity~.
   d. If _rounded_ is *+0*_𝔽 and _intlMV_ < 0, return ~negative-zero~.
   e. If _rounded_ is *+0*_𝔽, return 0.
10. Return _intlMV_.

--- referenced: FormatNumeric ( _numberFormat_: an Intl.NumberFormat, _x_: an Intl mathematical value, ): a String
  id: sec-formatnumber

1. Let _parts_ be PartitionNumberPattern(_numberFormat_, _x_).
2. Let _result_ be the empty String.
3. For each Record { [[Type]], [[Value]] } _part_ of _parts_, do
   a. Set _result_ to the string-concatenation of _result_ and _part_.[[Value]].
4. Return _result_.


FormatNumeric ( _numberFormat_: an Intl.NumberFormat, _x_: an Intl mathematical value, ): a String
  id: sec-formatnumber   type: abstract operation

1. Let _parts_ be PartitionNumberPattern(_numberFormat_, _x_).
2. Let _result_ be the empty String.
3. For each Record { [[Type]], [[Value]] } _part_ of _parts_, do
   a. Set _result_ to the string-concatenation of _result_ and _part_.[[Value]].
4. Return _result_.


ToIntlMathematicalValue ( _value_: an ECMAScript language value, ): either a normal completion containing an Intl mathematical value or a throw completion
  id: sec-tointlmathematicalvalue   type: abstract operation

description:
It returns _value_ converted to an Intl mathematical value, which is a mathematical value together with ~positive-infinity~, ~negative-infinity~, ~not-a-number~, and ~negative-zero~.
This abstract operation is similar to [sec-tonumeric], but a mathematical value can be returned instead of a Number or BigInt, so that exact decimal values can be represented.

1. Let _primValue_ be ? ToPrimitive(_value_, ~number~).
2. If _primValue_ is a BigInt, return ℝ(_primValue_).
3. If _primValue_ is a String, then
   a. Let _str_ be _primValue_.
4. Else,
   a. Let _x_ be ? ToNumber(_primValue_).
   b. If _x_ is *-0*_𝔽, return ~negative-zero~.
   c. Let _str_ be Number::toString(_x_, 10).
5. Let _text_ be StringToCodePoints(_str_).
6. Let _literal_ be ParseText(_text_, |StringNumericLiteral|).
7. If _literal_ is a List of errors, return ~not-a-number~.
8. Let _intlMV_ be the StringIntlMV of _literal_.
9. If _intlMV_ is a mathematical value, then
   a. Let _rounded_ be RoundMVResult(abs(_intlMV_)).
   b. If _rounded_ is *+∞*_𝔽 and _intlMV_ < 0, return ~negative-infinity~.
   c. If _rounded_ is *+∞*_𝔽, return ~positive-infinity~.
   d. If _rounded_ is *+0*_𝔽 and _intlMV_ < 0, return ~negative-zero~.
   e. If _rounded_ is *+0*_𝔽, return 0.
10. Return _intlMV_.

--- referenced: ToNumber ( _arg_: an ECMAScript language value, ): either a normal completion containing a Number or a throw completion
  id: sec-tonumber

description: It converts _arg_ to a value of type Number.

1. If _arg_ is a Number, return _arg_.
2. If _arg_ is either a Symbol or a BigInt, throw a *TypeError* exception.
3. If _arg_ is *undefined*, return *NaN*.
4. If _arg_ is either *null* or *false*, return *+0*_𝔽.
5. If _arg_ is *true*, return *1*_𝔽.
6. If _arg_ is a String, return StringToNumber(_arg_).
7. Assert: _arg_ is an Object.
8. Let _primitiveValue_ be ? ToPrimitive(_arg_, ~number~).
9. Assert: _primitiveValue_ is not an Object.
10. Return ? ToNumber(_primitiveValue_).

[subclause sec-tonumber-applied-to-the-string-type: ToNumber Applied to the String Type]

--- referenced: RoundMVResult ( _n_: a mathematical value, ): a Number
  id: sec-roundmvresult

description: It converts _n_ to a Number in an implementation-defined manner. For the purposes of this abstract operation, a digit is significant if it is not zero or there is a non-zero digit to its left and there is a non-zero digit to its right. For the purposes of this abstract operation, “the mathematical value denoted by” a representation of a mathematical value is the inverse of “the decimal representation of” a mathematical value.

1. If the decimal representation of _n_ has 20 or fewer significant digits, return 𝔽(_n_).
2. Let _option1_ be the mathematical value denoted by the result of replacing each significant digit in the decimal representation of _n_ after the 20th with a 0 digit.
3. Let _option2_ be the mathematical value denoted by the result of replacing each significant digit in the decimal representation of _n_ after the 20th with a 0 digit and then incrementing it at the 20th position (with carrying as necessary).
4. Let _chosen_ be an implementation-defined choice of either _option1_ or _option2_.
5. Return 𝔽(_chosen_).


FormatNumericToString ( _intlObj_: an Object, _x_: a mathematical value or ~negative-zero~, ): a Record with fields [[RoundedNumber]] (a mathematical value or ~negative-zero~) and [[FormattedString]] (a String)
  id: sec-formatnumerictostring   type: abstract operation

description: It rounds _x_ to an Intl mathematical value according to the internal slots of _intlObj_. The [[RoundedNumber]] field contains the rounded result value and the [[FormattedString]] field contains a String value representation of that result formatted according to the internal slots of _intlObj_.

1. Assert: _intlObj_ has [[RoundingMode]], [[RoundingType]], [[MinimumSignificantDigits]], [[MaximumSignificantDigits]], [[MinimumIntegerDigits]], [[MinimumFractionDigits]], [[MaximumFractionDigits]], [[RoundingIncrement]], and [[TrailingZeroDisplay]] internal slots.
2. If _x_ is ~negative-zero~, then
   a. Let _sign_ be ~negative~.
   b. Set _x_ to 0.
3. Else,
   a. Assert: _x_ is a mathematical value.
   b. If _x_ < 0, let _sign_ be ~negative~; else let _sign_ be ~positive~.
   c. If _sign_ is ~negative~, then
      i. Set _x_ to -_x_.
4. Let _unsignedRoundingMode_ be GetUnsignedRoundingMode(_intlObj_.[[RoundingMode]], _sign_).
5. If _intlObj_.[[RoundingType]] is ~significant-digits~, then
   a. Let _result_ be ToRawPrecision(_x_, _intlObj_.[[MinimumSignificantDigits]], _intlObj_.[[MaximumSignificantDigits]], _unsignedRoundingMode_).
6. Else if _intlObj_.[[RoundingType]] is ~fraction-digits~, then
   a. Let _result_ be ToRawFixed(_x_, _intlObj_.[[MinimumFractionDigits]], _intlObj_.[[MaximumFractionDigits]], _intlObj_.[[RoundingIncrement]], _unsignedRoundingMode_).
7. Else,
   a. Let _significantResult_ be ToRawPrecision(_x_, _intlObj_.[[MinimumSignificantDigits]], _intlObj_.[[MaximumSignificantDigits]], _unsignedRoundingMode_).
   b. Let _fractionResult_ be ToRawFixed(_x_, _intlObj_.[[MinimumFractionDigits]], _intlObj_.[[MaximumFractionDigits]], _intlObj_.[[RoundingIncrement]], _unsignedRoundingMode_).
   c. If _fractionResult_.[[RoundingMagnitude]] < _significantResult_.[[RoundingMagnitude]], let _fixedIsMorePrecise_ be *true*; else let _fixedIsMorePrecise_ be *false*.
   d. If _intlObj_.[[RoundingType]] is ~more-precision~ and _fixedIsMorePrecise_ is *true*, then
      i. Let _result_ be _fractionResult_.
   e. Else if _intlObj_.[[RoundingType]] is ~less-precision~ and _fixedIsMorePrecise_ is *false*, then
      i. Let _result_ be _fractionResult_.
   f. Else,
      i. Let _result_ be _significantResult_.
8. Set _x_ to _result_.[[RoundedNumber]].
9. Let _string_ be _result_.[[FormattedString]].
10. If _intlObj_.[[TrailingZeroDisplay]] is *"stripIfInteger"* and _x_ modulo 1 = 0, then
   a. Let _i_ be StringIndexOf(_string_, *"."*, 0).
   b. If _i_ is not ~not-found~, set _string_ to the substring of _string_ from 0 to _i_.
11. Let _int_ be _result_.[[IntegerDigitsCount]].
12. Let _minIntDigits_ be _intlObj_.[[MinimumIntegerDigits]].
13. If _int_ < _minIntDigits_, then
   a. Let _forwardZeros_ be the String consisting of _minIntDigits_ - _int_ occurrences of the code unit 0x0030 (DIGIT ZERO).
   b. Set _string_ to the string-concatenation of _forwardZeros_ and _string_.
14. If _sign_ is ~negative~, then
   a. If _x_ is 0, set _x_ to ~negative-zero~. Otherwise, set _x_ to -_x_.
15. Return the Record { [[RoundedNumber]]: _x_, [[FormattedString]]: _string_ }.

--- referenced: GetUnsignedRoundingMode ( _roundingMode_: a rounding mode, _sign_: ~negative~ or ~positive~, ): a specification type from the Unsigned Rounding Mode column of [table-intl-unsigned-rounding-modes]
  id: sec-getunsignedroundingmode

description: It returns the rounding mode that should be applied to the absolute value of a number to produce the same result as if _roundingMode_ were applied to the signed value of the number (negative if _sign_ is ~negative~, or positive otherwise).

1. Return the specification type in the Unsigned Rounding Mode column of [table-intl-unsigned-rounding-modes] for the row where the value in the Identifier column is _roundingMode_ and the value in the Sign column is _sign_.

Conversion from rounding mode to unsigned rounding mode
Identifier |
Sign |
Unsigned Rounding Mode |
*"ceil"* |
~positive~ |
~infinity~ |
~negative~ |
~zero~ |
*"floor"* |
~positive~ |
~zero~ |
~negative~ |
~infinity~ |
*"expand"* |
~positive~ |
~infinity~ |
~negative~ |
~infinity~ |
*"trunc"* |
~positive~ |
~zero~ |
~negative~ |
~zero~ |
*"halfCeil"* |
~positive~ |
~half-infinity~ |
~negative~ |
~half-zero~ |
*"halfFloor"* |
~positive~ |
~half-zero~ |
~negative~ |
~half-infinity~ |
*"halfExpand"* |
~positive~ |
~half-infinity~ |
~negative~ |
~half-infinity~ |
*"halfTrunc"* |
~positive~ |
~half-zero~ |
~negative~ |
~half-zero~ |
*"halfEven"* |
~positive~ |
~half-even~ |
~negative~ |
~half-even~ |

--- referenced: StringIndexOf ( _string_: a String, _searchValue_: a String, _fromIndex_: a non-negative integer, ): a non-negative integer or ~not-found~
  id: sec-stringindexof

1. Let _length_ be the length of _string_.
2. If _searchValue_ is the empty String and _fromIndex_ ≤ _length_, return _fromIndex_.
3. Let _searchLength_ be the length of _searchValue_.
4. For each integer _i_ such that _fromIndex_ ≤ _i_ ≤ _length_ - _searchLength_, in ascending order, do
   a. Let _candidate_ be the substring of _string_ from _i_ to _i_ + _searchLength_.
   b. If _candidate_ is _searchValue_, return _i_.
5. Return ~not-found~.

NOTE:
If _searchValue_ is the empty String and _fromIndex_ ≤ the length of _string_, this algorithm returns _fromIndex_. The empty String is effectively found at every position within a string, including after the last code unit.
NOTE:
This algorithm always returns ~not-found~ if _fromIndex_ + the length of _searchValue_ > the length of _string_.


ToRawPrecision ( _x_: a non-negative mathematical value, _minPrecision_: an integer in the inclusive interval from 1 to 21, _maxPrecision_: an integer in the inclusive interval from 1 to 21, _unsignedRoundingMode_: a specification type from the Unsigned Rounding Mode column of [table-intl-unsigned-rounding-modes], ): a Record with fields [[FormattedString]] (a String), [[RoundedNumber]] (a mathematical value), [[IntegerDigitsCount]] (an integer), and [[RoundingMagnitude]] (an integer)
  id: sec-torawprecision   type: abstract operation

description:
It involves solving the following equation, which returns a valid mathematical value given integer inputs:
ToRawPrecisionFn(_n_, _e_, _p_) = _n_ × 10^_e_–_p_+1
where 10^_p_–1 ≤ _n_ < 10^_p_

1. Let _p_ be _maxPrecision_.
2. If _x_ = 0, then
   a. Let _m_ be the String consisting of _p_ occurrences of the code unit 0x0030 (DIGIT ZERO).
   b. Let _e_ be 0.
   c. Let _xFinal_ be 0.
3. Else,
   a. Let _n1_ and _e1_ each be an integer and _r1_ a mathematical value, with _r1_ = ToRawPrecisionFn(_n1_, _e1_, _p_), such that _r1_ ≤ _x_ and _r1_ is maximized.
   b. Let _n2_ and _e2_ each be an integer and _r2_ a mathematical value, with _r2_ = ToRawPrecisionFn(_n2_, _e2_, _p_), such that _r2_ ≥ _x_ and _r2_ is minimized.
   c. Let _xFinal_ be ApplyUnsignedRoundingMode(_x_, _r1_, _r2_, _unsignedRoundingMode_).
   d. If _xFinal_ is _r1_, then
      i. Let _n_ be _n1_.
      ii. Let _e_ be _e1_.
   e. Else,
      i. Let _n_ be _n2_.
      ii. Let _e_ be _e2_.
   f. Let _m_ be the String consisting of the digits of the decimal representation of _n_ (in order, with no leading zeroes).
4. If _e_ ≥ (_p_ - 1), then
   a. Set _m_ to the string-concatenation of _m_ and _e_ - _p_ + 1 occurrences of the code unit 0x0030 (DIGIT ZERO).
   b. Let _int_ be _e_ + 1.
5. Else if _e_ ≥ 0, then
   a. Set _m_ to the string-concatenation of the first _e_ + 1 code units of _m_, the code unit 0x002E (FULL STOP), and the remaining _p_ - (_e_ + 1) code units of _m_.
   b. Let _int_ be _e_ + 1.
6. Else,
   a. Assert: _e_ < 0.
   b. Set _m_ to the string-concatenation of *"0."*, -(_e_ + 1) occurrences of the code unit 0x0030 (DIGIT ZERO), and _m_.
   c. Let _int_ be 1.
7. If _m_ contains the code unit 0x002E (FULL STOP) and _maxPrecision_ > _minPrecision_, then
   a. Let _cut_ be _maxPrecision_ - _minPrecision_.
   b. Repeat, while _cut_ > 0 and the last code unit of _m_ is 0x0030 (DIGIT ZERO),
      i. Remove the last code unit from _m_.
      ii. Set _cut_ to _cut_ - 1.
   c. If the last code unit of _m_ is 0x002E (FULL STOP), then
      i. Remove the last code unit from _m_.
8. Return the Record { [[FormattedString]]: _m_, [[RoundedNumber]]: _xFinal_, [[IntegerDigitsCount]]: _int_, [[RoundingMagnitude]]: _e_–_p_+1 }.

--- referenced: ApplyUnsignedRoundingMode ( _x_: a mathematical value, _r1_: a mathematical value, _r2_: a mathematical value, _unsignedRoundingMode_: a specification type from the Unsigned Rounding Mode column of [table-intl-unsigned-rounding-modes], ): a mathematical value
  id: sec-applyunsignedroundingmode

description: It considers _x_, bracketed below by _r1_ and above by _r2_, and returns either _r1_ or _r2_ according to _unsignedRoundingMode_.

1. If _x_ is _r1_, return _r1_.
2. Assert: _r1_ < _x_ < _r2_.
3. If _unsignedRoundingMode_ is ~zero~, return _r1_.
4. If _unsignedRoundingMode_ is ~infinity~, return _r2_.
5. Let _d1_ be _x_ – _r1_.
6. Let _d2_ be _r2_ – _x_.
7. If _d1_ < _d2_, return _r1_.
8. If _d2_ < _d1_, return _r2_.
9. Assert: _d1_ is _d2_.
10. If _unsignedRoundingMode_ is ~half-zero~, return _r1_.
11. If _unsignedRoundingMode_ is ~half-infinity~, return _r2_.
12. Assert: _unsignedRoundingMode_ is ~half-even~.
13. Let _cardinality_ be (_r1_ / (_r2_ – _r1_)) modulo 2.
14. If _cardinality_ is 0, return _r1_.
15. Return _r2_.


ToRawFixed ( _x_: a non-negative mathematical value, _minFraction_: an integer in the inclusive interval from 0 to 100, _maxFraction_: an integer in the inclusive interval from 0 to 100, _roundingIncrement_: an integer, _unsignedRoundingMode_: a specification type from the Unsigned Rounding Mode column of [table-intl-unsigned-rounding-modes], ): a Record with fields [[FormattedString]] (a String), [[RoundedNumber]] (a mathematical value), [[IntegerDigitsCount]] (an integer), and [[RoundingMagnitude]] (an integer)
  id: sec-torawfixed   type: abstract operation

description:
It involves solving the following equation, which returns a valid mathematical value given integer inputs:
ToRawFixedFn(_n_, _f_) = _n_ × 10^–_f_

1. Let _f_ be _maxFraction_.
2. Let _n1_ be an integer and _r1_ a mathematical value, with _r1_ = ToRawFixedFn(_n1_, _f_), such that _n1_ modulo _roundingIncrement_ = 0, _r1_ ≤ _x_, and _r1_ is maximized.
3. Let _n2_ be an integer and _r2_ a mathematical value, with _r2_ = ToRawFixedFn(_n2_, _f_), such that _n2_ modulo _roundingIncrement_ = 0, _r2_ ≥ _x_, and _r2_ is minimized.
4. Let _xFinal_ be ApplyUnsignedRoundingMode(_x_, _r1_, _r2_, _unsignedRoundingMode_).
5. If _xFinal_ is _r1_, let _n_ be _n1_. Otherwise, let _n_ be _n2_.
6. If _n_ = 0, let _m_ be *"0"*. Otherwise, let _m_ be the String consisting of the digits of the decimal representation of _n_ (in order, with no leading zeroes).
7. If _f_ ≠ 0, then
   a. Let _k_ be the length of _m_.
   b. If _k_ ≤ _f_, then
      i. Let _z_ be the String value consisting of _f_ + 1 - _k_ occurrences of the code unit 0x0030 (DIGIT ZERO).
      ii. Set _m_ to the string-concatenation of _z_ and _m_.
      iii. Set _k_ to _f_ + 1.
   c. Let _a_ be the first _k_ - _f_ code units of _m_, and let _b_ be the remaining _f_ code units of _m_.
   d. Set _m_ to the string-concatenation of _a_, *"."*, and _b_.
   e. Let _int_ be the length of _a_.
8. Else,
   a. Let _int_ be the length of _m_.
9. Let _cut_ be _maxFraction_ - _minFraction_.
10. Repeat, while _cut_ > 0 and the last code unit of _m_ is 0x0030 (DIGIT ZERO),
   a. Remove the last code unit from _m_.
   b. Set _cut_ to _cut_ - 1.
11. If the last code unit of _m_ is 0x002E (FULL STOP), then
   a. Remove the last code unit from _m_.
12. Return the Record { [[FormattedString]]: _m_, [[RoundedNumber]]: _xFinal_, [[IntegerDigitsCount]]: _int_, [[RoundingMagnitude]]: –_f_ }.

--- referenced: ApplyUnsignedRoundingMode ( _x_: a mathematical value, _r1_: a mathematical value, _r2_: a mathematical value, _unsignedRoundingMode_: a specification type from the Unsigned Rounding Mode column of [table-intl-unsigned-rounding-modes], ): a mathematical value
  id: sec-applyunsignedroundingmode

description: It considers _x_, bracketed below by _r1_ and above by _r2_, and returns either _r1_ or _r2_ according to _unsignedRoundingMode_.

1. If _x_ is _r1_, return _r1_.
2. Assert: _r1_ < _x_ < _r2_.
3. If _unsignedRoundingMode_ is ~zero~, return _r1_.
4. If _unsignedRoundingMode_ is ~infinity~, return _r2_.
5. Let _d1_ be _x_ – _r1_.
6. Let _d2_ be _r2_ – _x_.
7. If _d1_ < _d2_, return _r1_.
8. If _d2_ < _d1_, return _r2_.
9. Assert: _d1_ is _d2_.
10. If _unsignedRoundingMode_ is ~half-zero~, return _r1_.
11. If _unsignedRoundingMode_ is ~half-infinity~, return _r2_.
12. Assert: _unsignedRoundingMode_ is ~half-even~.
13. Let _cardinality_ be (_r1_ / (_r2_ – _r1_)) modulo 2.
14. If _cardinality_ is 0, return _r1_.
15. Return _r2_.


PartitionNumberPattern ( _numberFormat_: an object initialized as a NumberFormat, _x_: an Intl mathematical value, ): a List of Records with fields [[Type]] (a String) and [[Value]] (a String)
  id: sec-partitionnumberpattern   type: abstract operation

description: It creates the parts representing the mathematical value of _x_ according to the effective locale and the formatting options of _numberFormat_.

1. Let _exponent_ be 0.
2. If _x_ is ~not-a-number~, then
   a. Let _formattedString_ be an ILD String value indicating the *NaN* value.
3. Else if _x_ is ~positive-infinity~, then
   a. Let _formattedString_ be an ILD String value indicating positive infinity.
4. Else if _x_ is ~negative-infinity~, then
   a. Let _formattedString_ be an ILD String value indicating negative infinity.
5. Else,
   a. If _x_ is not ~negative-zero~, then
      i. Assert: _x_ is a mathematical value.
      ii. If _numberFormat_.[[Style]] is *"percent"*, set _x_ be 100 × _x_.
      iii. Set _exponent_ to ComputeExponent(_numberFormat_, _x_).
      iv. Set _x_ to _x_ × 10^-_exponent_.
   b. Let _formatNumberResult_ be FormatNumericToString(_numberFormat_, _x_).
   c. Let _formattedString_ be _formatNumberResult_.[[FormattedString]].
   d. Set _x_ to _formatNumberResult_.[[RoundedNumber]].
6. Let _pattern_ be GetNumberFormatPattern(_numberFormat_, _x_).
7. Let _result_ be a new empty List.
8. Let _patternParts_ be PartitionPattern(_pattern_).
9. For each Record { [[Type]], [[Value]] } _patternPart_ of _patternParts_, do
   a. Let _partType_ be _patternPart_.[[Type]].
   b. If _partType_ is *"literal"*, then
      i. Append the Record { [[Type]]: *"literal"*, [[Value]]: _patternPart_.[[Value]] } to _result_.
   c. Else if _partType_ is *"number"*, then
      i. Let _notationSubParts_ be PartitionNotationSubPattern(_numberFormat_, _x_, _formattedString_, _exponent_).
      ii. Set _result_ to the list-concatenation of _result_ and _notationSubParts_.
   d. Else if _partType_ is *"plusSign"*, then
      i. Let _plusSignSymbol_ be the ILND String representing the plus sign.
      ii. Append the Record { [[Type]]: *"plusSign"*, [[Value]]: _plusSignSymbol_ } to _result_.
   e. Else if _partType_ is *"minusSign"*, then
      i. Let _minusSignSymbol_ be the ILND String representing the minus sign.
      ii. Append the Record { [[Type]]: *"minusSign"*, [[Value]]: _minusSignSymbol_ } to _result_.
   f. Else if _partType_ is *"percentSign"* and _numberFormat_.[[Style]] is *"percent"*, then
      i. Let _percentSignSymbol_ be the ILND String representing the percent sign.
      ii. Append the Record { [[Type]]: *"percentSign"*, [[Value]]: _percentSignSymbol_ } to _result_.
   g. Else if _partType_ is *"unitPrefix"* and _numberFormat_.[[Style]] is *"unit"*, then
      i. Let _unit_ be _numberFormat_.[[Unit]].
      ii. Let _unitDisplay_ be _numberFormat_.[[UnitDisplay]].
      iii. Let _unitString_ be an ILD String value representing _unit_ before _x_ in _unitDisplay_ form, which may depend on _x_ in languages having different plural forms.
      iv. Append the Record { [[Type]]: *"unit"*, [[Value]]: _unitString_ } to _result_.
   h. Else if _partType_ is *"unitSuffix"* and _numberFormat_.[[Style]] is *"unit"*, then
      i. Let _unit_ be _numberFormat_.[[Unit]].
      ii. Let _unitDisplay_ be _numberFormat_.[[UnitDisplay]].
      iii. Let _unitString_ be an ILD String value representing _unit_ after _x_ in _unitDisplay_ form, which may depend on _x_ in languages having different plural forms.
      iv. Append the Record { [[Type]]: *"unit"*, [[Value]]: _unitString_ } to _result_.
   i. Else if _partType_ is *"currencyCode"* and _numberFormat_.[[Style]] is *"currency"*, then
      i. Let _currency_ be _numberFormat_.[[Currency]].
      ii. Append the Record { [[Type]]: *"currency"*, [[Value]]: _currency_ } to _result_.
   j. Else if _partType_ is *"currencyPrefix"* and _numberFormat_.[[Style]] is *"currency"*, then
      i. Let _currency_ be _numberFormat_.[[Currency]].
      ii. Let _currencyDisplay_ be _numberFormat_.[[CurrencyDisplay]].
      iii. Let _currencyString_ be an ILD String value representing _currency_ before _x_ in _currencyDisplay_ form, which may depend on _x_ in languages having different plural forms.
      iv. Append the Record { [[Type]]: *"currency"*, [[Value]]: _currencyString_ } to _result_.
   k. Else if _partType_ is *"currencySuffix"* and _numberFormat_.[[Style]] is *"currency"*, then
      i. Let _currency_ be _numberFormat_.[[Currency]].
      ii. Let _currencyDisplay_ be _numberFormat_.[[CurrencyDisplay]].
      iii. Let _currencyString_ be an ILD String value representing _currency_ after _x_ in _currencyDisplay_ form, which may depend on _x_ in languages having different plural forms. If the implementation does not have such a representation of _currency_, use _currency_ itself.
      iv. Append the Record { [[Type]]: *"currency"*, [[Value]]: _currencyString_ } to _result_.
   l. Else,
      i. Let _unknown_ be an ILND String based on _x_ and _partType_.
      ii. Append the Record { [[Type]]: *"unknown"*, [[Value]]: _unknown_ } to _result_.
10. Return _result_.

--- referenced: ComputeExponent ( _numberFormat_: an Intl.NumberFormat, _x_: a mathematical value, ): an integer
  id: sec-computeexponent

description:
It computes an exponent (power of ten) by which to scale _x_ according to the number formatting settings.
It handles cases such as 999 rounding up to 1000, requiring a different exponent.

1. If _x_ = 0, then
   a. Return 0.
2. If _x_ < 0, then
   a. Let _x_ = -_x_.
3. Let _magnitude_ be floor(log10(_x_)).
4. Let _exponent_ be ComputeExponentForMagnitude(_numberFormat_, _magnitude_).
5. Set _x_ to _x_ × 10^-_exponent_.
6. Let _formatNumberResult_ be FormatNumericToString(_numberFormat_, _x_).
7. If _formatNumberResult_.[[RoundedNumber]] = 0, then
   a. Return _exponent_.
8. Let _newMagnitude_ be floor(log10(_formatNumberResult_.[[RoundedNumber]])).
9. If _newMagnitude_ is _magnitude_ - _exponent_, then
   a. Return _exponent_.
10. Return ComputeExponentForMagnitude(_numberFormat_, _magnitude_ + 1).

--- referenced: PartitionPattern ( _pattern_: a Pattern String, ): a List of Records with fields [[Type]] (a String) and [[Value]] (a String or *undefined*)
  id: sec-partitionpattern

description:
The [[Value]] field will be a String value if [[Type]] is *"literal"*, and *undefined* otherwise.

1. Let _result_ be a new empty List.
2. Let _placeholderEnd_ be -1.
3. Let _placeholderStart_ be StringIndexOf(_pattern_, *"{"*, 0).
4. Repeat, while _placeholderStart_ is not ~not-found~,
   a. Let _literal_ be the substring of _pattern_ from _placeholderEnd_ + 1 to _placeholderStart_.
   b. If _literal_ is not the empty String, then
      i. Append the Record { [[Type]]: *"literal"*, [[Value]]: _literal_ } to _result_.
   c. Set _placeholderEnd_ to StringIndexOf(_pattern_, *"}"*, _placeholderStart_).
   d. Assert: _placeholderEnd_ is not ~not-found~ and _placeholderStart_ < _placeholderEnd_.
   e. Let _placeholderName_ be the substring of _pattern_ from _placeholderStart_ + 1 to _placeholderEnd_.
   f. Append the Record { [[Type]]: _placeholderName_, [[Value]]: *undefined* } to _result_.
   g. Set _placeholderStart_ to StringIndexOf(_pattern_, *"{"*, _placeholderEnd_).
5. Let _tail_ be the substring of _pattern_ from _placeholderEnd_ + 1.
6. If _tail_ is not the empty String, then
   a. Append the Record { [[Type]]: *"literal"*, [[Value]]: _tail_ } to _result_.
7. Return _result_.


FormatNumericToParts ( _numberFormat_: an Intl.NumberFormat, _x_: an Intl mathematical value, ): an Array
  id: sec-formatnumbertoparts   type: abstract operation

1. Let _parts_ be PartitionNumberPattern(_numberFormat_, _x_).
2. Let _result_ be ! ArrayCreate(0).
3. Let _n_ be 0.
4. For each Record { [[Type]], [[Value]] } _part_ of _parts_, do
   a. Let _partObj_ be OrdinaryObjectCreate(%Object.prototype%).
   b. Perform ! CreateDataPropertyOrThrow(_partObj_, *"type"*, _part_.[[Type]]).
   c. Perform ! CreateDataPropertyOrThrow(_partObj_, *"value"*, _part_.[[Value]]).
   d. Perform ! CreateDataPropertyOrThrow(_result_, ! ToString(𝔽(_n_)), _partObj_).
   e. Set _n_ to _n_ + 1.
5. Return _result_.

--- referenced: ArrayCreate ( _length_: a non-negative integer, optional _proto_: an Object, ): either a normal completion containing an Array exotic object or a throw completion
  id: sec-arraycreate

description: It is used to specify the creation of new Arrays.

1. If _length_ > 2^32 - 1, throw a *RangeError* exception.
2. If _proto_ is not present, set _proto_ to %Array.prototype%.
3. Let _array_ be MakeBasicObject(« [[Prototype]], [[Extensible]] »).
4. Set _array_.[[Prototype]] to _proto_.
5. Set _array_.[[DefineOwnProperty]] as specified in [sec-array-exotic-objects-defineownproperty-p-desc].
6. Perform ! OrdinaryDefineOwnProperty(_array_, *"length"*, PropertyDescriptor { [[Value]]: 𝔽(_length_), [[Writable]]: *true*, [[Enumerable]]: *false*, [[Configurable]]: *false* }).
7. Return _array_.

--- referenced: OrdinaryObjectCreate ( _proto_: an Object or *null*, optional _additionalInternalSlotsList_: a List of names of internal slots, ): an Object
  id: sec-ordinaryobjectcreate

description: It is used to specify the runtime creation of new ordinary objects. _additionalInternalSlotsList_ contains the names of additional internal slots that must be defined as part of the object, beyond [[Prototype]] and [[Extensible]]. If _additionalInternalSlotsList_ is not provided, a new empty List is used.

1. Let _internalSlotsList_ be « [[Prototype]], [[Extensible]] ».
2. If _additionalInternalSlotsList_ is present, set _internalSlotsList_ to the list-concatenation of _internalSlotsList_ and _additionalInternalSlotsList_.
3. Let _obj_ be MakeBasicObject(_internalSlotsList_).
4. Set _obj_.[[Prototype]] to _proto_.
5. Return _obj_.

NOTE:
Although OrdinaryObjectCreate does little more than call MakeBasicObject, its use communicates the intention to create an ordinary object, and not an exotic one. Thus, within this specification, it is not called by any algorithm that subsequently modifies the internal methods of the object in ways that would make the result non-ordinary. Operations that create exotic objects invoke MakeBasicObject directly.

--- referenced: CreateDataPropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-createdatapropertyorthrow

description: It is used to create a new own property of an object. It throws a *TypeError* exception if the requested property update cannot be performed.

1. Let _success_ be ? CreateDataProperty(_obj_, _propertyKey_, _value_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.

NOTE:
This abstract operation creates a property whose attributes are set to the same defaults used for properties created by the ECMAScript language assignment operator. Normally, the property will not already exist. If it does exist and is not configurable or if _obj_ is not extensible, [[DefineOwnProperty]] will return *false* causing this operation to throw a *TypeError* exception.

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


Intl.NumberFormat.prototype.formatToParts ( _value_ )
  id: sec-intl.numberformat.prototype.formattoparts   type: built-in function

When the `formatToParts` method is called with an optional argument _value_, the following steps are taken:

1. Let _nf_ be the *this* value.
2. Perform ? RequireInternalSlot(_nf_, [[InitializedNumberFormat]]).
3. Let _x_ be ? ToIntlMathematicalValue(_value_).
4. Return FormatNumericToParts(_nf_, _x_).

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: ToIntlMathematicalValue ( _value_: an ECMAScript language value, ): either a normal completion containing an Intl mathematical value or a throw completion
  id: sec-tointlmathematicalvalue

description:
It returns _value_ converted to an Intl mathematical value, which is a mathematical value together with ~positive-infinity~, ~negative-infinity~, ~not-a-number~, and ~negative-zero~.
This abstract operation is similar to [sec-tonumeric], but a mathematical value can be returned instead of a Number or BigInt, so that exact decimal values can be represented.

1. Let _primValue_ be ? ToPrimitive(_value_, ~number~).
2. If _primValue_ is a BigInt, return ℝ(_primValue_).
3. If _primValue_ is a String, then
   a. Let _str_ be _primValue_.
4. Else,
   a. Let _x_ be ? ToNumber(_primValue_).
   b. If _x_ is *-0*_𝔽, return ~negative-zero~.
   c. Let _str_ be Number::toString(_x_, 10).
5. Let _text_ be StringToCodePoints(_str_).
6. Let _literal_ be ParseText(_text_, |StringNumericLiteral|).
7. If _literal_ is a List of errors, return ~not-a-number~.
8. Let _intlMV_ be the StringIntlMV of _literal_.
9. If _intlMV_ is a mathematical value, then
   a. Let _rounded_ be RoundMVResult(abs(_intlMV_)).
   b. If _rounded_ is *+∞*_𝔽 and _intlMV_ < 0, return ~negative-infinity~.
   c. If _rounded_ is *+∞*_𝔽, return ~positive-infinity~.
   d. If _rounded_ is *+0*_𝔽 and _intlMV_ < 0, return ~negative-zero~.
   e. If _rounded_ is *+0*_𝔽, return 0.
10. Return _intlMV_.

--- referenced: FormatNumericToParts ( _numberFormat_: an Intl.NumberFormat, _x_: an Intl mathematical value, ): an Array
  id: sec-formatnumbertoparts

1. Let _parts_ be PartitionNumberPattern(_numberFormat_, _x_).
2. Let _result_ be ! ArrayCreate(0).
3. Let _n_ be 0.
4. For each Record { [[Type]], [[Value]] } _part_ of _parts_, do
   a. Let _partObj_ be OrdinaryObjectCreate(%Object.prototype%).
   b. Perform ! CreateDataPropertyOrThrow(_partObj_, *"type"*, _part_.[[Type]]).
   c. Perform ! CreateDataPropertyOrThrow(_partObj_, *"value"*, _part_.[[Value]]).
   d. Perform ! CreateDataPropertyOrThrow(_result_, ! ToString(𝔽(_n_)), _partObj_).
   e. Set _n_ to _n_ + 1.
5. Return _result_.


## Done when

- Every test262 run under intl402/NumberFormat/ passes (check with `./t262 intl402/NumberFormat/`). 196 pass on the main branch right now.

## Previous attempts

- attempt 542: worker reported BLOCKED but changed files: currency fraction-digit data (CurrencyDigits) is missing from ICU4X, and the formatting methods are not written yet.
- attempt 542: rejected by gates: files outside scope: crates/kahvi-cli/src/main.rs
- attempt 547: worker reported BLOCKED but changed files: no currency fraction-digit data in ICU4X for `CurrencyDigits`, no way to list a locale's numbering systems, and the formatting methods are unwritten; I stopped because I ran out of my USD budget.
- attempt 547: rejected (made_up_results, contradicts_spec): The change follows the spec closely. ResolveOptions ordering, the GetOption/Get sequence in SetNumberFormatUnitOptions and SetNumberFormatDigitOptions, the resolvedOptions table order, UnwrapNumberFormat, and ChainNumberFormat all match the steps. CurrencyDigits is an honest not-implemented throw. T Replace locale_data() in constructor.rs with real [[LocaleData]] for the nu key derived from ICU4X (the supported numbering systems for the requested locale, default first). If ICU4X cannot supply that, make the nu path throw agent.not_implemented(...) naming the missing data rather than returning ["latn"]. Remove the unwrap_or("latn") default in step 8 and use the resolved [[nu]] value directly. Also replace the silent `if let Some(target) = agent.native_mut(...)` in the constructor with an error or an expect-style invariant, so a failed slot write is not ignored. Everything else (the spec ordering in SetNumberFormatDigitOptions, SetNumberFormatUnitOptions, resolvedOptions, UnwrapNumberFormat, ChainNumberFormat, and the CurrencyDigits not-implemented throw) checks out and can be kept.

