# Ticket B-global-parse-int: parseInt

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

- `crates/kahvi-builtins/src/global/parse_int.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

parseInt ( _string_, _radix_ )
  id: sec-parseint-string-radix   type: built-in function

This function produces an integral Number dictated by interpretation of the contents of _string_ according to the specified _radix_. Leading white space in _string_ is ignored. If _radix_ coerces to 0 (such as when it is *undefined*), it is assumed to be 10 except when the number representation begins with *"0x"* or *"0X"*, in which case it is assumed to be 16. If _radix_ is 16, the number representation may optionally begin with *"0x"* or *"0X"*.
It is the %parseInt% intrinsic object.
It performs the following steps when called:

1. Let _inputString_ be ? ToString(_string_).
2. Let _radixMV_ be ℝ(? ToInt32(_radix_)).
3. If _radixMV_ ≠ 0 and _radixMV_ is not in the inclusive interval from 2 to 36, return *NaN*.
4. Let _trimmedString_ be ! TrimString(_inputString_, ~start~).
5. If _trimmedString_ is the empty String, return *NaN*.
6. Let _sign_ be 1.
7. If the first code unit of _trimmedString_ is the code unit 0x002D (HYPHEN-MINUS), then
   a. Set _sign_ to -1.
   b. Set _trimmedString_ to the substring of _trimmedString_ from 1.
8. Else if the first code unit of _trimmedString_ is the code unit 0x002B (PLUS SIGN), then
   a. Set _trimmedString_ to the substring of _trimmedString_ from 1.
9. If _radixMV_ = 0 or _radixMV_ = 16, then
   a. If the length of _trimmedString_ ≥ 2 and the substring of _trimmedString_ from 0 to 2 is either *"0x"* or *"0X"*, then
      i. Set _trimmedString_ to the substring of _trimmedString_ from 2.
      ii. Set _radixMV_ to 16.
10. If _radixMV_ = 0, set _radixMV_ to 10.
11. If _trimmedString_ contains a code unit that is not a radix-_radixMV_ digit, let _end_ be the index within _trimmedString_ of the first such code unit; else let _end_ be the length of _trimmedString_.
12. Let _numberString_ be the substring of _trimmedString_ from 0 to _end_.
13. If _numberString_ is empty, return *NaN*.
14. Let _mathInt_ be the integer value that is represented by _numberString_ in radix-_radixMV_ notation, using the letters A through Z and a through z for digits with values 10 through 35. (However, if _radixMV_ = 10 and _numberString_ contains more than 20 significant digits, every significant digit after the 20th may be replaced by a 0 digit, at the option of the implementation; and if _radixMV_ is not one of 2, 4, 8, 10, 16, or 32, then _mathInt_ may be an implementation-approximated integer representing the integer value denoted by _numberString_ in radix-_radixMV_ notation.)
15. If _sign_ = -1 and _mathInt_ = 0, return *-0*_𝔽.
16. Return 𝔽(_sign_ × _mathInt_).

NOTE:
This function may interpret only a leading portion of _string_ as an integer value; it ignores any code units that cannot be interpreted as part of the notation of an integer, and no indication is given that any such code units were ignored.

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

--- referenced: ToInt32 ( _arg_: an ECMAScript language value, ): either a normal completion containing an integral Number or a throw completion
  id: sec-toint32

description: It converts _arg_ to one of 2^32 integral Number values in the inclusive interval from 𝔽(-2^31) to 𝔽(2^31 - 1), excluding *-0*_𝔽.

1. Let _int_ be ? ToIntegerOrInfinity(_arg_).
2. Return 𝔽(ToFixedSizeInteger(_int_, ~signed~, 32)).

--- referenced: TrimString ( _arg_: an ECMAScript language value, _where_: ~start~, ~end~, or ~start+end~, ): either a normal completion containing a String or a throw completion
  id: sec-trimstring

description: It interprets _arg_ as a sequence of UTF-16 encoded code points, as described in [sec-ecmascript-language-types-string-type].

1. Perform ? RequireObjectCoercible(_arg_).
2. Let _string_ be ? ToString(_arg_).
3. If _where_ is ~start~, then
   a. Let _trimmedString_ be the String value that is a copy of _string_ with leading white space removed.
4. Else if _where_ is ~end~, then
   a. Let _trimmedString_ be the String value that is a copy of _string_ with trailing white space removed.
5. Else,
   a. Assert: _where_ is ~start+end~.
   b. Let _trimmedString_ be the String value that is a copy of _string_ with both leading and trailing white space removed.
6. Return _trimmedString_.

The definition of white space is the union of |WhiteSpace| and |LineTerminator|. When determining whether a Unicode code point is in Unicode general category “Space_Separator” (“Zs”), code unit sequences are interpreted as UTF-16 encoded code point sequences as specified in [sec-ecmascript-language-types-string-type].


## Done when

- Every test262 run under built-ins/parseInt/ passes (check with `./t262 built-ins/parseInt/`). 8 pass on the main branch right now.

