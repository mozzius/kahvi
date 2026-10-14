# Ticket B-typed-array-prototype-reverse: TypedArray.prototype.reverse

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

- `crates/kahvi-builtins/src/typed_array/prototype_reverse.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

%TypedArray%.prototype.reverse ( )
  id: sec-%typedarray%.prototype.reverse   type: built-in function

The interpretation and use of the arguments of this method are the same as for `Array.prototype.reverse` as defined in [sec-array.prototype.reverse].
This method performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
3. Let _length_ be TypedArrayLength(_taRecord_).
4. Let _middle_ be floor(_length_ / 2).
5. Let _lower_ be 0.
6. Repeat, while _lower_ ≠ _middle_,
   a. Let _upper_ be _length_ - _lower_ - 1.
   b. Let _lowerValue_ be TypedArrayGetElement(_obj_, 𝔽(_lower_)).
   c. Let _upperValue_ be TypedArrayGetElement(_obj_, 𝔽(_upper_)).
   d. Perform ! TypedArraySetElement(_obj_, 𝔽(_lower_), _upperValue_).
   e. Perform ! TypedArraySetElement(_obj_, 𝔽(_upper_), _lowerValue_).
   f. Set _lower_ to _lower_ + 1.
7. Return _obj_.

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

--- referenced: TypedArrayGetElement ( _obj_: a TypedArray, _index_: a Number, ): a Number, a BigInt, or *undefined*
  id: sec-typedarraygetelement

1. If IsValidIntegerIndex(_obj_, _index_) is *false*, return *undefined*.
2. Let _offset_ be _obj_.[[ByteOffset]].
3. Let _elementSize_ be TypedArrayElementSize(_obj_).
4. Let _byteIndexInBuffer_ be (ℝ(_index_) × _elementSize_) + _offset_.
5. Let _elementType_ be TypedArrayElementType(_obj_).
6. Return GetValueFromBuffer(_obj_.[[ViewedArrayBuffer]], _byteIndexInBuffer_, _elementType_, *true*, ~unordered~).

--- referenced: TypedArraySetElement ( _obj_: a TypedArray, _index_: a Number, _value_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-typedarraysetelement

1. If _obj_.[[ContentType]] is ~bigint~, let _number_ be ? ToBigInt(_value_).
2. Else, let _number_ be ? ToNumber(_value_).
3. If IsValidIntegerIndex(_obj_, _index_) is *true*, then
   a. Let _offset_ be _obj_.[[ByteOffset]].
   b. Let _elementSize_ be TypedArrayElementSize(_obj_).
   c. Let _byteIndexInBuffer_ be (ℝ(_index_) × _elementSize_) + _offset_.
   d. Let _elementType_ be TypedArrayElementType(_obj_).
   e. Perform SetValueInBuffer(_obj_.[[ViewedArrayBuffer]], _byteIndexInBuffer_, _elementType_, _number_, *true*, ~unordered~).
4. Return ~unused~.

NOTE:
This operation always appears to succeed, but it has no effect when attempting to write past the end of a TypedArray or to a TypedArray which is backed by a detached ArrayBuffer.


## Done when

- Every test262 run under built-ins/TypedArray/prototype/reverse/ passes (check with `./t262 built-ins/TypedArray/prototype/reverse/`). 8 pass on the main branch right now.

