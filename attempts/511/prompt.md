# Ticket B-uint8-array-prototype-to-hex: Uint8Array.prototype.toHex

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

- `crates/kahvi-builtins/src/uint8_array/prototype_to_hex.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`

## Spec

Uint8Array.prototype.toHex ( )
  id: sec-uint8array.prototype.tohex   type: built-in function

1. Let _obj_ be the *this* value.
2. Perform ? ValidateUint8Array(_obj_).
3. Let _toEncode_ be ? GetUint8ArrayBytes(_obj_).
4. Let _out_ be the empty String.
5. For each byte _byte_ of _toEncode_, do
   a. Let _hex_ be Number::toString(𝔽(_byte_), 16).
   b. Set _hex_ to StringPad(_hex_, 2, *"0"*, ~start~).
   c. Set _out_ to the string-concatenation of _out_ and _hex_.
6. Return _out_.

--- referenced: ValidateUint8Array ( _ta_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-validateuint8array

1. Perform ? RequireInternalSlot(_ta_, [[TypedArrayName]]).
2. If _ta_.[[TypedArrayName]] is not *"Uint8Array"*, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: GetUint8ArrayBytes ( _ta_: a Uint8Array, ): either a normal completion containing a List of byte values or a throw completion
  id: sec-getuint8arraybytes

1. Let _buffer_ be _ta_.[[ViewedArrayBuffer]].
2. Let _taRecord_ be ? ValidateTypedArrayBounds(_ta_, ~seq-cst~).
3. Let _length_ be TypedArrayLength(_taRecord_).
4. Let _byteOffset_ be _ta_.[[ByteOffset]].
5. Let _bytes_ be a new empty List.
6. Let _index_ be 0.
7. Repeat, while _index_ < _length_,
   a. Let _byteIndex_ be _byteOffset_ + _index_.
   b. Let _byte_ be ℝ(GetValueFromBuffer(_buffer_, _byteIndex_, ~uint8~, *true*, ~unordered~)).
   c. Append _byte_ to _bytes_.
   d. Set _index_ to _index_ + 1.
8. Return _bytes_.

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

- Every test262 run under built-ins/Uint8Array/prototype/toHex/ passes (check with `./t262 built-ins/Uint8Array/prototype/toHex/`). 8 pass on the main branch right now.

