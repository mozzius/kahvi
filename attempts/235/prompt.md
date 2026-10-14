# Ticket B-global-unescape: unescape

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

- `crates/kahvi-builtins/src/global/unescape.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

unescape ( _string_ )
  id: sec-unescape-string   type: built-in function

This function is a property of the global object. It computes a new version of a String value in which each escape sequence of the sort that might be introduced by the `escape` function is replaced with the code unit that it represents.
It is the %unescape% intrinsic object.
It performs the following steps when called:

1. Set _string_ to ? ToString(_string_).
2. Let _length_ be the length of _string_.
3. Let _result_ be the empty String.
4. Let _k_ be 0.
5. Repeat, while _k_ < _length_,
   a. Let _codeUnit_ be the code unit at index _k_ within _string_.
   b. If _codeUnit_ is the code unit 0x0025 (PERCENT SIGN), then
      i. Let _hexDigits_ be the empty String.
      ii. Let _optionalAdvance_ be 0.
      iii. If _k_ + 5 < _length_ and the code unit at index _k_ + 1 within _string_ is the code unit 0x0075 (LATIN SMALL LETTER U), then
         1. Set _hexDigits_ to the substring of _string_ from _k_ + 2 to _k_ + 6.
         2. Set _optionalAdvance_ to 5.
      iv. Else if _k_ + 3 ≤ _length_, then
         1. Set _hexDigits_ to the substring of _string_ from _k_ + 1 to _k_ + 3.
         2. Set _optionalAdvance_ to 2.
      v. Let _parseResult_ be ParseText(_hexDigits_, |HexDigits[~Sep]|).
      vi. If _parseResult_ is a Parse Node, then
         1. Let _codeUnitNumber_ be the MV of _parseResult_.
         2. Set _codeUnit_ to the code unit whose numeric value is _codeUnitNumber_.
         3. Set _k_ to _k_ + _optionalAdvance_.
   c. Set _result_ to the string-concatenation of _result_ and _codeUnit_.
   d. Set _k_ to _k_ + 1.
6. Return _result_.

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

- Every test262 run under annexB/built-ins/unescape/ passes (check with `./t262 annexB/built-ins/unescape/`). 8 pass on the main branch right now.

