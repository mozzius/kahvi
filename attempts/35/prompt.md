# Ticket B-global-parse-float: parseFloat

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

- `crates/kahvi-builtins/src/global/parse_float.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

parseFloat ( _string_ )
  id: sec-parsefloat-string   type: built-in function

This function produces a Number value dictated by interpretation of the contents of the _string_ argument as a decimal literal.
It is the %parseFloat% intrinsic object.
It performs the following steps when called:

1. Let _inputString_ be ? ToString(_string_).
2. Let _trimmedString_ be ! TrimString(_inputString_, ~start~).
3. Let _trimmed_ be StringToCodePoints(_trimmedString_).
4. Let _trimmedPrefix_ be the longest prefix of _trimmed_ that satisfies the syntax of a |StrDecimalLiteral|, which might be _trimmed_ itself. If there is no such prefix, return *NaN*.
5. Let _parsedNumber_ be ParseText(_trimmedPrefix_, |StrDecimalLiteral|).
6. Assert: _parsedNumber_ is a Parse Node.
7. Return the StringNumericValue of _parsedNumber_.

NOTE:
This function may interpret only a leading portion of _string_ as a Number value; it ignores any code units that cannot be interpreted as part of the notation of a decimal literal, and no indication is given that any such code units were ignored.

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

- Every test262 run under built-ins/parseFloat/ passes (check with `./t262 built-ins/parseFloat/`). 10 pass on the main branch right now.

