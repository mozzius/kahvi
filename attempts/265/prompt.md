# Ticket B-global-escape: escape

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

- `crates/kahvi-builtins/src/global/escape.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

escape ( _string_ )
  id: sec-escape-string   type: built-in function

This function is a property of the global object. It computes a new version of a String value in which certain code units have been replaced by a hexadecimal escape sequence.
When replacing a code unit of numeric value less than or equal to 0x00FF, a two-digit escape sequence of the form %xx is used. When replacing a code unit of numeric value strictly greater than 0x00FF, a four-digit escape sequence of the form %uxxxx is used.
It is the %escape% intrinsic object.
It performs the following steps when called:

1. Set _string_ to ? ToString(_string_).
2. Let _length_ be the length of _string_.
3. Let _result_ be the empty String.
4. Let _unescapedSet_ be the string-concatenation of the ASCII word characters and *"@\*+-./"*.
5. Let _k_ be 0.
6. Repeat, while _k_ < _length_,
   a. Let _codeUnit_ be the code unit at index _k_ within _string_.
   b. If _unescapedSet_ contains _codeUnit_, then
      i. Let _nextPart_ be _codeUnit_.
   c. Else,
      i. Let _codeUnitNumber_ be the numeric value of _codeUnit_.
      ii. If _codeUnitNumber_ < 256, then
         1. Let _hex_ be the String representation of _codeUnitNumber_, formatted as an uppercase hexadecimal number.
         2. Let _nextPart_ be the string-concatenation of *"%"* and StringPad(_hex_, 2, *"0"*, ~start~).
      iii. Else,
         1. Let _hex_ be the String representation of _codeUnitNumber_, formatted as an uppercase hexadecimal number.
         2. Let _nextPart_ be the string-concatenation of *"%u"* and StringPad(_hex_, 4, *"0"*, ~start~).
   d. Set _result_ to the string-concatenation of _result_ and _nextPart_.
   e. Set _k_ to _k_ + 1.
7. Return _result_.

NOTE:
The encoding is partly based on the encoding described in RFC 1738, but the entire encoding specified in this standard is described above without regard to the contents of RFC 1738. This encoding does not reflect changes to RFC 1738 made by RFC 3986.

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

--- referenced: StringPad ( _string_: a String, _maxLength_: a non-negative integer, _fillString_: a String, _placement_: ~start~ or ~end~, ): a String
  id: sec-stringpad

1. Let _stringLength_ be the length of _string_.
2. If _maxLength_ ≤ _stringLength_, return _string_.
3. If _fillString_ is the empty String, return _string_.
4. Let _fillLength_ be _maxLength_ - _stringLength_.
5. Let _truncatedStringFiller_ be the String value consisting of repeated concatenations of _fillString_ truncated to length _fillLength_.
6. If _placement_ is ~start~, return the string-concatenation of _truncatedStringFiller_ and _string_.
7. Return the string-concatenation of _string_ and _truncatedStringFiller_.

NOTE:
The argument _maxLength_ will be clamped such that it can be no smaller than the length of _string_.
NOTE:
The argument _fillString_ defaults to *" "* (the String value consisting of the code unit 0x0020 SPACE).


## Done when

- Every test262 run under annexB/built-ins/escape/ passes (check with `./t262 annexB/built-ins/escape/`). 8 pass on the main branch right now.

