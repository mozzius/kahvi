# Ticket B-uint8-array-from-hex: Uint8Array.fromHex

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

Write FromHex (sec-fromhex) as a `pub(crate)` function in this file: Uint8Array.prototype.setFromHex calls it.

## Files you may change

- `crates/kahvi-builtins/src/uint8_array/from_hex.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`

## Spec

Uint8Array.fromHex ( _string_ )
  id: sec-uint8array.fromhex   type: built-in function

1. If _string_ is not a String, throw a *TypeError* exception.
2. Let _result_ be FromHex(_string_).
3. If _result_.[[Error]] is not ~none~, then
   a. Throw _result_.[[Error]].
4. Let _resultLength_ be the number of elements in _result_.[[Bytes]].
5. Let _ta_ be ? AllocateTypedArray(*"Uint8Array"*, %Uint8Array%, *"%Uint8Array.prototype%"*, _resultLength_).
6. Assert: _ta_.[[ViewedArrayBuffer]].[[ArrayBufferByteLength]] is the number of elements in _result_.[[Bytes]].
7. Set the value at each index of _ta_.[[ViewedArrayBuffer]].[[ArrayBufferData]] to the value at the corresponding index of _result_.[[Bytes]].
8. Return _ta_.

--- referenced: FromHex ( _string_: a String, optional _maxLength_: a non-negative integer, ): a Record with fields [[Read]] (an integer), [[Bytes]] (a List of byte values), and [[Error]] (a *SyntaxError* object or ~none~)
  id: sec-fromhex

1. If _maxLength_ is not present, set _maxLength_ to 2^53 - 1.
2. Let _length_ be the length of _string_.
3. Let _bytes_ be a new empty List.
4. Let _read_ be 0.
5. If _length_ modulo 2 ≠ 0, then
   a. Let _error_ be a newly created *SyntaxError* object.
   b. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: _error_ }.
6. Repeat, while _read_ < _length_ and the number of elements in _bytes_ < _maxLength_,
   a. Let _hexits_ be the substring of _string_ from _read_ to _read_ + 2.
   b. If _hexits_ contains any code units which are not in *"0123456789abcdefABCDEF"*, then
      i. Let _error_ be a newly created *SyntaxError* object.
      ii. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: _error_ }.
   c. Set _read_ to _read_ + 2.
   d. Let _byte_ be the integer value represented by _hexits_ in base-16 notation, using the letters *A* through *F* and *a* through *f* for digits with values 10 through 15.
   e. Append _byte_ to _bytes_.
7. Return the Record { [[Read]]: _read_, [[Bytes]]: _bytes_, [[Error]]: ~none~ }.

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

- Every test262 run under built-ins/Uint8Array/fromHex/ passes (check with `./t262 built-ins/Uint8Array/fromHex/`). 8 pass on the main branch right now.

