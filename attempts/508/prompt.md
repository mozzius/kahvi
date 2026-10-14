# Ticket B-uint8-array-from-base64: Uint8Array.fromBase64

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

Write FromBase64 (sec-frombase64) and the option helpers (GetUint8ArrayBytes is not needed here) as `pub(crate)` functions in this file: Uint8Array.prototype.setFromBase64 calls them.

## Files you may change

- `crates/kahvi-builtins/src/uint8_array/from_base64.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`

## Spec

Uint8Array.fromBase64 ( _string_ [ , _options_ ] )
  id: sec-uint8array.frombase64   type: built-in function

1. If _string_ is not a String, throw a *TypeError* exception.
2. Let _optionsObj_ be ? GetOptionsObject(_options_).
3. Let _alphabet_ be ? Get(_optionsObj_, *"alphabet"*).
4. If _alphabet_ is *undefined*, set _alphabet_ to *"base64"*.
5. If _alphabet_ is neither *"base64"* nor *"base64url"*, throw a *TypeError* exception.
6. Let _lastChunkHandling_ be ? Get(_optionsObj_, *"lastChunkHandling"*).
7. If _lastChunkHandling_ is *undefined*, set _lastChunkHandling_ to *"loose"*.
8. If _lastChunkHandling_ is not one of *"loose"*, *"strict"*, or *"stop-before-partial"*, throw a *TypeError* exception.
9. Let _result_ be FromBase64(_string_, _alphabet_, _lastChunkHandling_).
10. If _result_.[[Error]] is not ~none~, then
   a. Throw _result_.[[Error]].
11. Let _resultLength_ be the number of elements in _result_.[[Bytes]].
12. Let _ta_ be ? AllocateTypedArray(*"Uint8Array"*, %Uint8Array%, *"%Uint8Array.prototype%"*, _resultLength_).
13. Assert: _ta_.[[ViewedArrayBuffer]].[[ArrayBufferByteLength]] is the number of elements in _result_.[[Bytes]].
14. Set the value at each index of _ta_.[[ViewedArrayBuffer]].[[ArrayBufferData]] to the value at the corresponding index of _result_.[[Bytes]].
15. Return _ta_.

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

--- referenced: AllocateTypedArray ( _ctorName_: a String which is the name of a TypedArray constructor in [table-the-typedarray-constructors], _newTarget_: a constructor, _defaultProto_: a String, optional _length_: a non-negative integer, ): either a normal completion containing a TypedArray or a throw completion
  id: sec-allocatetypedarray

description: It is used to validate and create an instance of a TypedArray constructor. If the _length_ argument is passed, an ArrayBuffer of that length is also allocated and associated with the new TypedArray instance. AllocateTypedArray provides common semantics that is used by _TypedArray_.

1. Let _proto_ be ? GetPrototypeFromConstructor(_newTarget_, _defaultProto_).
2. Let _obj_ be TypedArrayCreate(_proto_).
3. Assert: _obj_.[[ViewedArrayBuffer]] is *undefined*.
4. Set _obj_.[[TypedArrayName]] to _ctorName_.
5. If _ctorName_ is either *"BigInt64Array"* or *"BigUint64Array"*, set _obj_.[[ContentType]] to ~bigint~.
6. Else, set _obj_.[[ContentType]] to ~number~.
7. If _length_ is not present, then
   a. Set _obj_.[[ByteLength]] to 0.
   b. Set _obj_.[[ByteOffset]] to 0.
   c. Set _obj_.[[ArrayLength]] to 0.
8. Else,
   a. Perform ? AllocateTypedArrayBuffer(_obj_, _length_).
9. Return _obj_.


## Done when

- Every test262 run under built-ins/Uint8Array/fromBase64/ passes (check with `./t262 built-ins/Uint8Array/fromBase64/`). 8 pass on the main branch right now.

