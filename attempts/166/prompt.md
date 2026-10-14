# Ticket B-string-prototype-to-lower-case: String.prototype.toLowerCase

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

- `crates/kahvi-builtins/src/string/prototype_to_lower_case.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/string_iterator.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`

## Spec

String.prototype.toLowerCase ( )
  id: sec-string.prototype.tolowercase   type: built-in function

This method interprets a String value as a sequence of UTF-16 encoded code points, as described in [sec-ecmascript-language-types-string-type].
It performs the following steps when called:

1. Let _thisValue_ be the *this* value.
2. Perform ? RequireObjectCoercible(_thisValue_).
3. Let _string_ be ? ToString(_thisValue_).
4. Let _sText_ be StringToCodePoints(_string_).
5. Let _lowerText_ be toLowercase(_sText_), according to the Unicode Default Case Conversion algorithm.
6. Let _lowercaseString_ be CodePointsToString(_lowerText_).
7. Return _lowercaseString_.

The result must be derived according to the locale-insensitive case mappings in the Unicode Character Database (this explicitly includes not only the file UnicodeData.txt, but also all locale-insensitive mappings in the file SpecialCasing.txt that accompanies it).
NOTE:
The case mapping of some code points may produce multiple code points. In this case the result String may not be the same length as the source String. Because both `toUpperCase` and `toLowerCase` have context-sensitive behaviour, the methods are not symmetrical. In other words, `s.toUpperCase().toLowerCase()` is not necessarily equal to `s.toLowerCase()`.
NOTE:
This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.

--- referenced: RequireObjectCoercible ( _arg_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireobjectcoercible

description: It throws an error if _arg_ is a value that cannot be converted to an Object using ToObject.

1. If _arg_ is either *undefined* or *null*, throw a *TypeError* exception.
2. Return ~unused~.

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

- Every test262 run under built-ins/String/prototype/toLowerCase/ passes (check with `./t262 built-ins/String/prototype/toLowerCase/`). 16 pass on the main branch right now.

