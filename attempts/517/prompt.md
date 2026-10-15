# Ticket B-atomics-load: Atomics.load

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

- `crates/kahvi-builtins/src/atomics/load.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/atomics.rs`
- `crates/kahvi-builtins/src/atomics/add.rs`
- `crates/kahvi-builtins/src/atomics/and.rs`
- `crates/kahvi-core/src/object/typed_array.rs`

## Spec

Atomics.load ( _ta_, _index_ )
  id: sec-atomics.load   type: built-in function

This function performs the following steps when called:

1. Let _byteIndexInBuffer_ be ? ValidateAtomicAccessOnIntegerTypedArray(_ta_, _index_).
2. Perform ? RevalidateAtomicAccess(_ta_, _byteIndexInBuffer_).
3. Let _buffer_ be _ta_.[[ViewedArrayBuffer]].
4. Let _elementType_ be TypedArrayElementType(_ta_).
5. Return GetValueFromBuffer(_buffer_, _byteIndexInBuffer_, _elementType_, *true*, ~seq-cst~).

--- referenced: ValidateAtomicAccessOnIntegerTypedArray ( _ta_: an ECMAScript language value, _requestIndex_: an ECMAScript language value, ): either a normal completion containing a non-negative integer or a throw completion
  id: sec-validateatomicaccessonintegertypedarray

1. Let _taRecord_ be ? ValidateIntegerTypedArray(_ta_, *false*).
2. Return ? ValidateAtomicAccess(_taRecord_, _requestIndex_).

--- referenced: RevalidateAtomicAccess ( _ta_: a TypedArray, _byteIndexInBuffer_: a non-negative integer, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-revalidateatomicaccess

description: This operation revalidates the index within the backing buffer for atomic operations after all argument coercions are performed in Atomics methods, as argument coercions can have arbitrary side effects, which could cause the buffer to become out of bounds. This operation does not throw when _ta_'s backing buffer is a SharedArrayBuffer.

1. NOTE: Bounds checking is not a synchronizing operation when _ta_'s backing buffer is a growable SharedArrayBuffer.
2. Let _taRecord_ be ? ValidateTypedArrayBounds(_ta_, ~unordered~).
3. Assert: _byteIndexInBuffer_ ≥ _ta_.[[ByteOffset]].
4. Let _elementSize_ be TypedArrayElementSize(_ta_).
5. Let _elementLastByteIndex_ be _byteIndexInBuffer_ + _elementSize_ - 1.
6. If _elementLastByteIndex_ ≥ _taRecord_.[[CachedBufferByteLength]], throw a *RangeError* exception.
7. Return ~unused~.

--- referenced: TypedArrayElementType ( _obj_: a TypedArray, ): a TypedArray element type
  id: sec-typedarrayelementtype

1. Return the Element Type value specified in [table-the-typedarray-constructors] for _obj_.[[TypedArrayName]].

--- referenced: GetValueFromBuffer ( _arrayBuffer_: an ArrayBuffer or SharedArrayBuffer, _byteIndex_: a non-negative integer, _type_: a TypedArray element type, _isTypedArray_: a Boolean, _order_: ~seq-cst~ or ~unordered~, optional _isLittleEndian_: a Boolean, ): a Number or a BigInt
  id: sec-getvaluefrombuffer

1. Assert: IsDetachedBuffer(_arrayBuffer_) is *false*.
2. Assert: There are sufficient bytes in _arrayBuffer_ starting at _byteIndex_ to represent a value of _type_.
3. Let _block_ be _arrayBuffer_.[[ArrayBufferData]].
4. Let _elementSize_ be the Element Size value specified in [table-the-typedarray-constructors] for Element Type _type_.
5. If IsSharedArrayBuffer(_arrayBuffer_) is *true*, then
   a. Assert: _block_ is a Shared Data Block.
   b. Let _rawValue_ be GetRawBytesFromSharedBlock(_block_, _byteIndex_, _type_, _isTypedArray_, _order_).
6. Else,
   a. Let _rawValue_ be a List whose elements are bytes from _block_ at indices in the interval from _byteIndex_ (inclusive) to _byteIndex_ + _elementSize_ (exclusive).
7. Assert: The number of elements in _rawValue_ is _elementSize_.
8. If _isLittleEndian_ is not present, then
   a. Let _agentRecord_ be the Agent Record of the surrounding agent.
   b. Set _isLittleEndian_ to _agentRecord_.[[LittleEndian]].
9. Return RawBytesToNumeric(_type_, _rawValue_, _isLittleEndian_).


## Done when

- Every test262 run under built-ins/Atomics/load/ passes (check with `./t262 built-ins/Atomics/load/`). 0 pass on the main branch right now.

