# Ticket B-typed-array-prototype-slice: TypedArray.prototype.slice

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

- `crates/kahvi-builtins/src/typed_array/prototype_slice.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

%TypedArray%.prototype.slice ( _start_, _end_ )
  id: sec-%typedarray%.prototype.slice   type: built-in function

The interpretation and use of the arguments of this method are the same as for `Array.prototype.slice` as defined in [sec-array.prototype.slice].
This method performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
3. Let _sourceArrayLength_ be TypedArrayLength(_taRecord_).
4. Let _startIndex_ be ? ToClampedIndex(_start_, _sourceArrayLength_).
5. If _end_ is *undefined*, let _endIndex_ be _sourceArrayLength_; else let _endIndex_ be ? ToClampedIndex(_end_, _sourceArrayLength_).
6. Let _countBytes_ be max(_endIndex_ - _startIndex_, 0).
7. Let _resultArray_ be ? TypedArraySpeciesCreate(_obj_, « 𝔽(_countBytes_) »).
8. If _countBytes_ > 0, then
   a. Set _taRecord_ to ? ValidateTypedArrayBounds(_obj_, ~seq-cst~).
   b. Set _endIndex_ to min(_endIndex_, TypedArrayLength(_taRecord_)).
   c. Set _countBytes_ to max(_endIndex_ - _startIndex_, 0).
   d. Let _sourceType_ be TypedArrayElementType(_obj_).
   e. Let _targetType_ be TypedArrayElementType(_resultArray_).
   f. If _sourceType_ is _targetType_, then
      i. NOTE: The transfer must be performed in a manner that preserves the bit-level encoding of the source data.
      ii. Let _sourceBuffer_ be _obj_.[[ViewedArrayBuffer]].
      iii. Let _targetBuffer_ be _resultArray_.[[ViewedArrayBuffer]].
      iv. Let _elementSize_ be TypedArrayElementSize(_obj_).
      v. Let _sourceByteOffset_ be _obj_.[[ByteOffset]].
      vi. Let _sourceByteIndex_ be (_startIndex_ × _elementSize_) + _sourceByteOffset_.
      vii. Let _targetByteIndex_ be _resultArray_.[[ByteOffset]].
      viii. Let _endByteIndex_ be _targetByteIndex_ + (_countBytes_ × _elementSize_).
      ix. Repeat, while _targetByteIndex_ < _endByteIndex_,
         1. Let _value_ be GetValueFromBuffer(_sourceBuffer_, _sourceByteIndex_, ~uint8~, *true*, ~unordered~).
         2. Perform SetValueInBuffer(_targetBuffer_, _targetByteIndex_, ~uint8~, _value_, *true*, ~unordered~).
         3. Set _sourceByteIndex_ to _sourceByteIndex_ + 1.
         4. Set _targetByteIndex_ to _targetByteIndex_ + 1.
   g. Else,
      i. Let _n_ be 0.
      ii. Let _k_ be _startIndex_.
      iii. Repeat, while _k_ < _endIndex_,
         1. Let _kValue_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
         2. Perform ! TypedArraySetElement(_resultArray_, 𝔽(_n_), _kValue_).
         3. Set _k_ to _k_ + 1.
         4. Set _n_ to _n_ + 1.
9. Return _resultArray_.

This method is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.

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

--- referenced: TypedArraySpeciesCreate ( _exemplar_: a TypedArray, _argList_: a List of ECMAScript language values, ): either a normal completion containing a TypedArray or a throw completion
  id: typedarray-species-create

description: It is used to specify the creation of a new TypedArray using a constructor function that is derived from _exemplar_. Unlike ArraySpeciesCreate, which can create non-Array objects through the use of %Symbol.species%, this operation enforces that the constructor function creates an actual TypedArray.

1. Let _defaultCtor_ be the intrinsic object associated with the constructor name _exemplar_.[[TypedArrayName]] in [table-the-typedarray-constructors].
2. Let _ctor_ be ? SpeciesConstructor(_exemplar_, _defaultCtor_).
3. Let _result_ be ? TypedArrayCreateFromConstructor(_ctor_, _argList_).
4. If _result_.[[ContentType]] is not _exemplar_.[[ContentType]], throw a *TypeError* exception.
5. Return _result_.

--- referenced: ValidateTypedArrayBounds ( _ta_: a TypedArray, _order_: ~seq-cst~ or ~unordered~, ): either a normal completion containing a TypedArray With Buffer Witness Record or a throw completion
  id: sec-validatetypedarraybounds

description: It validates that _ta_ does not exceed the bounds of its backing buffer.

1. Let _taRecord_ be MakeTypedArrayWithBufferWitnessRecord(_ta_, _order_).
2. If IsTypedArrayOutOfBounds(_taRecord_) is *true*, throw a *TypeError* exception.
3. Return _taRecord_.

--- referenced: TypedArrayElementType ( _obj_: a TypedArray, ): a TypedArray element type
  id: sec-typedarrayelementtype

1. Return the Element Type value specified in [table-the-typedarray-constructors] for _obj_.[[TypedArrayName]].

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

--- referenced: TypedArrayGetElement ( _obj_: a TypedArray, _index_: a Number, ): a Number, a BigInt, or *undefined*
  id: sec-typedarraygetelement

1. If IsValidIntegerIndex(_obj_, _index_) is *false*, return *undefined*.
2. Let _offset_ be _obj_.[[ByteOffset]].
3. Let _elementSize_ be TypedArrayElementSize(_obj_).
4. Let _byteIndexInBuffer_ be (ℝ(_index_) × _elementSize_) + _offset_.
5. Let _elementType_ be TypedArrayElementType(_obj_).
6. Return GetValueFromBuffer(_obj_.[[ViewedArrayBuffer]], _byteIndexInBuffer_, _elementType_, *true*, ~unordered~).


## Done when

- Every test262 run under built-ins/TypedArray/prototype/slice/ passes (check with `./t262 built-ins/TypedArray/prototype/slice/`). 8 pass on the main branch right now.

