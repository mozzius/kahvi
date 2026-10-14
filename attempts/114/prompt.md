# Ticket B-typed-array-prototype-copy-within: TypedArray.prototype.copyWithin

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

- `crates/kahvi-builtins/src/typed_array/prototype_copy_within.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

%TypedArray%.prototype.copyWithin ( _target_, _start_ [ , _end_ ] )
  id: sec-%typedarray%.prototype.copywithin   type: built-in function

The interpretation and use of the arguments of this method are the same as for `Array.prototype.copyWithin` as defined in [sec-array.prototype.copywithin].
This method performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
3. Let _length_ be TypedArrayLength(_taRecord_).
4. Let _targetIndex_ be ? ToClampedIndex(_target_, _length_).
5. Let _startIndex_ be ? ToClampedIndex(_start_, _length_).
6. If _end_ is *undefined*, let _endIndex_ be _length_; else let _endIndex_ be ? ToClampedIndex(_end_, _length_).
7. Let _count_ be min(_endIndex_ - _startIndex_, _length_ - _targetIndex_).
8. If _count_ > 0, then
   a. NOTE: The copying must be performed in a manner that preserves the bit-level encoding of the source data.
   b. Let _buffer_ be _obj_.[[ViewedArrayBuffer]].
   c. Set _taRecord_ to ? ValidateTypedArrayBounds(_obj_, ~seq-cst~).
   d. Set _length_ to TypedArrayLength(_taRecord_).
   e. NOTE: Side-effects of the above steps may have reduced the size of _obj_, in which case copying should proceed with the longest still-applicable prefix.
   f. Set _count_ to min(_count_, _length_ - _startIndex_, _length_ - _targetIndex_).
   g. Let _elementSize_ be TypedArrayElementSize(_obj_).
   h. Let _byteOffset_ be _obj_.[[ByteOffset]].
   i. Let _toByteIndex_ be (_targetIndex_ × _elementSize_) + _byteOffset_.
   j. Let _fromByteIndex_ be (_startIndex_ × _elementSize_) + _byteOffset_.
   k. Let _countBytes_ be _count_ × _elementSize_.
   l. If _fromByteIndex_ < _toByteIndex_ and _toByteIndex_ < _fromByteIndex_ + _countBytes_, then
      i. Let _direction_ be -1.
      ii. Set _fromByteIndex_ to _fromByteIndex_ + _countBytes_ - 1.
      iii. Set _toByteIndex_ to _toByteIndex_ + _countBytes_ - 1.
   m. Else,
      i. Let _direction_ be 1.
   n. Repeat, while _countBytes_ > 0,
      i. Assert: _fromByteIndex_ ≥ 0 and _toByteIndex_ ≥ 0.
      ii. Let _value_ be GetValueFromBuffer(_buffer_, _fromByteIndex_, ~uint8~, *true*, ~unordered~).
      iii. Perform SetValueInBuffer(_buffer_, _toByteIndex_, ~uint8~, _value_, *true*, ~unordered~).
      iv. Set _fromByteIndex_ to _fromByteIndex_ + _direction_.
      v. Set _toByteIndex_ to _toByteIndex_ + _direction_.
      vi. Set _countBytes_ to _countBytes_ - 1.
9. Return _obj_.

--- referenced: ValidateTypedArray ( _obj_: an ECMAScript language value, _order_: ~seq-cst~ or ~unordered~, ): either a normal completion containing a TypedArray With Buffer Witness Record or a throw completion
  id: sec-validatetypedarray

1. Perform ? RequireInternalSlot(_obj_, [[TypedArrayName]]).
2. Assert: _obj_ has a [[ViewedArrayBuffer]] internal slot.
3. Return ? ValidateTypedArrayBounds(_obj_, _order_).

--- referenced: TypedArrayLength ( _taRecord_: a TypedArray With Buffer Witness Record, ): a non-negative integer
  id: sec-typedarraylength

1. Assert: IsTypedArrayOutOfBounds(_taRecord_) is *false*.
2. Let _obj_ be _taRecord_.[[Object]].
3. If _obj_.[[ArrayLength]] is not ~auto~, return _obj_.[[ArrayLength]].
4. Assert: IsFixedLengthArrayBuffer(_obj_.[[ViewedArrayBuffer]]) is *false*.
5. Let _byteOffset_ be _obj_.[[ByteOffset]].
6. Let _elementSize_ be TypedArrayElementSize(_obj_).
7. Let _byteLength_ be _taRecord_.[[CachedBufferByteLength]].
8. Assert: _byteLength_ is not ~detached~.
9. Return floor((_byteLength_ - _byteOffset_) / _elementSize_).

--- referenced: ToClampedIndex ( _value_: an ECMAScript language value, _length_: a non-negative integer, ): either a normal completion containing a non-negative integer or a throw completion
  id: sec-toclampedindex

description: It converts _value_ to an integer (interpreting negative values relative to _length_) and clamps the result to the inclusive interval from 0 to _length_.

1. Let _index_ be ? ToAbsoluteIndex(_value_, _length_).
2. Return the result of clamping _index_ between 0 and _length_.

--- referenced: ValidateTypedArrayBounds ( _ta_: a TypedArray, _order_: ~seq-cst~ or ~unordered~, ): either a normal completion containing a TypedArray With Buffer Witness Record or a throw completion
  id: sec-validatetypedarraybounds

description: It validates that _ta_ does not exceed the bounds of its backing buffer.

1. Let _taRecord_ be MakeTypedArrayWithBufferWitnessRecord(_ta_, _order_).
2. If IsTypedArrayOutOfBounds(_taRecord_) is *true*, throw a *TypeError* exception.
3. Return _taRecord_.

--- referenced: TypedArrayElementSize ( _obj_: a TypedArray, ): a non-negative integer
  id: sec-typedarrayelementsize

1. Return the Element Size value specified in [table-the-typedarray-constructors] for _obj_.[[TypedArrayName]].

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

--- referenced: SetValueInBuffer ( _arrayBuffer_: an ArrayBuffer or SharedArrayBuffer, _byteIndex_: a non-negative integer, _type_: a TypedArray element type, _value_: a Number or a BigInt, _isTypedArray_: a Boolean, _order_: ~seq-cst~, ~unordered~, or ~init~, optional _isLittleEndian_: a Boolean, ): ~unused~
  id: sec-setvalueinbuffer

1. Assert: IsDetachedBuffer(_arrayBuffer_) is *false*.
2. Assert: There are sufficient bytes in _arrayBuffer_ starting at _byteIndex_ to represent a value of _type_.
3. Assert: _value_ is a BigInt if IsBigIntElementType(_type_) is *true*; else, _value_ is a Number.
4. Let _block_ be _arrayBuffer_.[[ArrayBufferData]].
5. Let _elementSize_ be the Element Size value specified in [table-the-typedarray-constructors] for Element Type _type_.
6. Let _agentRecord_ be the Agent Record of the surrounding agent.
7. If _isLittleEndian_ is not present, then
   a. Set _isLittleEndian_ to _agentRecord_.[[LittleEndian]].
8. Let _rawBytes_ be NumericToRawBytes(_type_, _value_, _isLittleEndian_).
9. If IsSharedArrayBuffer(_arrayBuffer_) is *true*, then
   a. Let _execution_ be _agentRecord_.[[CandidateExecution]].
   b. Let _eventsRecord_ be the Agent Events Record of _execution_.[[EventsRecords]] whose [[AgentSignifier]] is AgentSignifier().
   c. If _isTypedArray_ is *true* and IsNoTearConfiguration(_type_, _order_) is *true*, let _noTear_ be *true*; else let _noTear_ be *false*.
   d. Append WriteSharedMemory { [[Order]]: _order_, [[NoTear]]: _noTear_, [[Block]]: _block_, [[ByteIndex]]: _byteIndex_, [[ElementSize]]: _elementSize_, [[Payload]]: _rawBytes_ } to _eventsRecord_.[[EventList]].
10. Else,
   a. Store the individual bytes of _rawBytes_ into _block_, starting at _block_[_byteIndex_].
11. Return ~unused~.


## Done when

- Every test262 run under built-ins/TypedArray/prototype/copyWithin/ passes (check with `./t262 built-ins/TypedArray/prototype/copyWithin/`). 8 pass on the main branch right now.

## Previous attempts

- attempt 106: rejected by gates: files outside scope: crates/kahvi-core/src/host.rs, crates/kahvi-core/src/object/host_object.rs, crates/kahvi-ffi/src/lib.rs; lint: unsafe is architect-only at crates/kahvi-ffi/src/lib.rs:227: unsafe fn enter_callback<T>(; lint: unsafe is architect-only at crates/kahvi-ffi/src/lib.rs:281: String::from_utf8_lossy(unsafe { std::slice::from_raw_parts(source, len) }).into_owned();; lint: unsafe is architect-only at crates/kahvi-ffi/src/lib.rs:299: unsafe {; lint: unsafe is architect-only at crates/kahvi-ffi/src/lib.rs:372: pub unsafe extern "C" fn kahvi_strict_equals(; lint: unsafe is architect-on
- attempt 110: worker reported BLOCKED but changed files: BigInt support and `Array.prototype.fill` are not implemented, which the remaining 12 copyWithin test262 runs need.
- attempt 110: rejected by gates: files outside scope: crates/kahvi-core/src/host.rs, crates/kahvi-core/src/object/host_object.rs, crates/kahvi-ffi/src/lib.rs; lint: unsafe is architect-only at crates/kahvi-ffi/src/lib.rs:227: unsafe fn enter_callback<T>(; lint: unsafe is architect-only at crates/kahvi-ffi/src/lib.rs:281: String::from_utf8_lossy(unsafe { std::slice::from_raw_parts(source, len) }).into_owned();; lint: unsafe is architect-only at crates/kahvi-ffi/src/lib.rs:299: unsafe {; lint: unsafe is architect-only at crates/kahvi-ffi/src/lib.rs:372: pub unsafe extern "C" fn kahvi_strict_equals(; lint: unsafe is architect-on
- attempt 110: escalated from haiku to sonnet

