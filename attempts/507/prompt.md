# Ticket B-uint8-array-prototype-to-base64: Uint8Array.prototype.toBase64

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

- `crates/kahvi-builtins/src/uint8_array/prototype_to_base64.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`

## Spec

Uint8Array.prototype.toBase64 ( [ _options_ ] )
  id: sec-uint8array.prototype.tobase64   type: built-in function

1. Let _obj_ be the *this* value.
2. Perform ? ValidateUint8Array(_obj_).
3. Let _optionsObj_ be ? GetOptionsObject(_options_).
4. Let _alphabet_ be ? Get(_optionsObj_, *"alphabet"*).
5. If _alphabet_ is *undefined*, set _alphabet_ to *"base64"*.
6. If _alphabet_ is neither *"base64"* nor *"base64url"*, throw a *TypeError* exception.
7. Let _omitPadding_ be ToBoolean(? Get(_optionsObj_, *"omitPadding"*)).
8. Let _toEncode_ be ? GetUint8ArrayBytes(_obj_).
9. If _alphabet_ is *"base64"*, then
   a. Let _outAscii_ be the sequence of code points which results from encoding _toEncode_ according to the base64 encoding specified in section 4 of RFC 4648. Padding is included if and only if _omitPadding_ is *false*.
10. Else,
   a. Assert: _alphabet_ is *"base64url"*.
   b. Let _outAscii_ be the sequence of code points which results from encoding _toEncode_ according to the base64url encoding specified in section 5 of RFC 4648. Padding is included if and only if _omitPadding_ is *false*.
11. Return CodePointsToString(_outAscii_).

--- referenced: ValidateUint8Array ( _ta_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-validateuint8array

1. Perform ? RequireInternalSlot(_ta_, [[TypedArrayName]]).
2. If _ta_.[[TypedArrayName]] is not *"Uint8Array"*, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: GetOptionsObject ( _options_: an ECMAScript language value, ): either a normal completion containing an Object or a throw completion
  id: sec-getoptionsobject

1. If _options_ is *undefined*, then
   a. Return OrdinaryObjectCreate(*null*).
2. If _options_ is an Object, then
   a. Return _options_.
3. Throw a *TypeError* exception.

--- referenced: Get ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-get-o-p

description: It is used to retrieve the value of a specific property of an object.

1. Return ? _obj_.[[Get]](_propertyKey_, _obj_).

--- referenced: ToBoolean ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-toboolean

description: It converts _arg_ to a value of type Boolean.

1. If _arg_ is a Boolean, return _arg_.
2. If _arg_ is one of *undefined*, *null*, *+0*_𝔽, *-0*_𝔽, *NaN*, *0*_ℤ, or the empty String, return *false*.
3. If the host is a web browser or otherwise supports [sec-IsHTMLDDA-internal-slot], then
   a. If _arg_ is an Object and _arg_ has an [[IsHTMLDDA]] internal slot, return *false*.
4. Return *true*.

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


## Done when

- Every test262 run under built-ins/Uint8Array/prototype/toBase64/ passes (check with `./t262 built-ins/Uint8Array/prototype/toBase64/`). 8 pass on the main branch right now.

