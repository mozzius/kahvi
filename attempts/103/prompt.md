# Ticket B-typed-array-prototype-subarray: TypedArray.prototype.subarray

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

- `crates/kahvi-builtins/src/typed_array/prototype_subarray.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

%TypedArray%.prototype.subarray ( _start_, _end_ )
  id: sec-%typedarray%.prototype.subarray   type: built-in function

This method returns a new _TypedArray_ whose element type is the element type of this _TypedArray_ and whose ArrayBuffer is the ArrayBuffer of this _TypedArray_, referencing the elements in the interval from _start_ (inclusive) to _end_ (exclusive). If either _start_ or _end_ is negative, it refers to an index from the end of the array, as opposed to from the beginning.
It performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Perform ? RequireInternalSlot(_obj_, [[TypedArrayName]]).
3. Assert: _obj_ has a [[ViewedArrayBuffer]] internal slot.
4. Let _buffer_ be _obj_.[[ViewedArrayBuffer]].
5. Let _sourceRecord_ be MakeTypedArrayWithBufferWitnessRecord(_obj_, ~seq-cst~).
6. If IsTypedArrayOutOfBounds(_sourceRecord_) is *true*, then
   a. Let _sourceLength_ be 0.
7. Else,
   a. Let _sourceLength_ be TypedArrayLength(_sourceRecord_).
8. Let _startIndex_ be ? ToClampedIndex(_start_, _sourceLength_).
9. Let _elementSize_ be TypedArrayElementSize(_obj_).
10. Let _sourceByteOffset_ be _obj_.[[ByteOffset]].
11. Let _beginByteOffset_ be _sourceByteOffset_ + (_startIndex_ × _elementSize_).
12. If _obj_.[[ArrayLength]] is ~auto~ and _end_ is *undefined*, then
   a. Return ? TypedArraySpeciesCreate(_obj_, « _buffer_, 𝔽(_beginByteOffset_) »).
13. If _end_ is *undefined*, let _endIndex_ be _sourceLength_; else let _endIndex_ be ? ToClampedIndex(_end_, _sourceLength_).
14. Let _newLength_ be max(_endIndex_ - _startIndex_, 0).
15. Return ? TypedArraySpeciesCreate(_obj_, « _buffer_, 𝔽(_beginByteOffset_), 𝔽(_newLength_) »).

This method is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: MakeTypedArrayWithBufferWitnessRecord ( _obj_: a TypedArray, _order_: ~seq-cst~ or ~unordered~, ): a TypedArray With Buffer Witness Record
  id: sec-maketypedarraywithbufferwitnessrecord

1. Let _buffer_ be _obj_.[[ViewedArrayBuffer]].
2. If IsDetachedBuffer(_buffer_) is *true*, then
   a. Let _byteLength_ be ~detached~.
3. Else,
   a. Let _byteLength_ be ArrayBufferByteLength(_buffer_, _order_).
4. Return the TypedArray With Buffer Witness Record { [[Object]]: _obj_, [[CachedBufferByteLength]]: _byteLength_ }.

--- referenced: IsTypedArrayOutOfBounds ( _taRecord_: a TypedArray With Buffer Witness Record, ): a Boolean
  id: sec-istypedarrayoutofbounds

description: It checks if any of the object's numeric properties reference a value at an index not contained within the underlying buffer's bounds.

1. Let _obj_ be _taRecord_.[[Object]].
2. Let _bufferByteLength_ be _taRecord_.[[CachedBufferByteLength]].
3. If IsDetachedBuffer(_obj_.[[ViewedArrayBuffer]]) is *true*, then
   a. Assert: _bufferByteLength_ is ~detached~.
   b. Return *true*.
4. Assert: _bufferByteLength_ is a non-negative integer.
5. Let _byteOffsetStart_ be _obj_.[[ByteOffset]].
6. If _obj_.[[ArrayLength]] is ~auto~, then
   a. Let _byteOffsetEnd_ be _bufferByteLength_.
7. Else,
   a. Let _elementSize_ be TypedArrayElementSize(_obj_).
   b. Let _arrayByteLength_ be _obj_.[[ArrayLength]] × _elementSize_.
   c. Let _byteOffsetEnd_ be _byteOffsetStart_ + _arrayByteLength_.
8. NOTE: A 0-length TypedArray whose [[ByteOffset]] is _bufferByteLength_ is not considered out-of-bounds.
9. If _byteOffsetStart_ > _bufferByteLength_ or _byteOffsetEnd_ > _bufferByteLength_, return *true*.
10. Return *false*.

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

--- referenced: TypedArrayElementSize ( _obj_: a TypedArray, ): a non-negative integer
  id: sec-typedarrayelementsize

1. Return the Element Size value specified in [table-the-typedarray-constructors] for _obj_.[[TypedArrayName]].

--- referenced: TypedArraySpeciesCreate ( _exemplar_: a TypedArray, _argList_: a List of ECMAScript language values, ): either a normal completion containing a TypedArray or a throw completion
  id: typedarray-species-create

description: It is used to specify the creation of a new TypedArray using a constructor function that is derived from _exemplar_. Unlike ArraySpeciesCreate, which can create non-Array objects through the use of %Symbol.species%, this operation enforces that the constructor function creates an actual TypedArray.

1. Let _defaultCtor_ be the intrinsic object associated with the constructor name _exemplar_.[[TypedArrayName]] in [table-the-typedarray-constructors].
2. Let _ctor_ be ? SpeciesConstructor(_exemplar_, _defaultCtor_).
3. Let _result_ be ? TypedArrayCreateFromConstructor(_ctor_, _argList_).
4. If _result_.[[ContentType]] is not _exemplar_.[[ContentType]], throw a *TypeError* exception.
5. Return _result_.


## Done when

- Every test262 run under built-ins/TypedArray/prototype/subarray/ passes (check with `./t262 built-ins/TypedArray/prototype/subarray/`). 8 pass on the main branch right now.

