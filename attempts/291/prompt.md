# Ticket B-typed-array-prototype-at: TypedArray.prototype.at

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

- `crates/kahvi-builtins/src/typed_array/prototype_at.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

%TypedArray%.prototype.at ( _index_ )
  id: sec-%typedarray%.prototype.at   type: built-in function

1. Let _obj_ be the *this* value.
2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
3. Let _length_ be TypedArrayLength(_taRecord_).
4. Let _k_ be ? ToAbsoluteIndex(_index_, _length_).
5. If _k_ < 0 or _k_ ≥ _length_, return *undefined*.
6. Return TypedArrayGetElement(_obj_, 𝔽(_k_)).

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

--- referenced: ToAbsoluteIndex ( _value_: an ECMAScript language value, _length_: a non-negative integer, ): either a normal completion containing either an integer, +∞, or -∞, or a throw completion
  id: sec-toabsoluteindex

description: It converts _value_ to an integer, +∞, or -∞, interpreting negative values relative to _length_.

1. Let _int_ be ? ToIntegerOrInfinity(_value_).
2. If _int_ is finite and _int_ < 0, set _int_ to _length_ + _int_.
3. Return _int_.

--- referenced: TypedArrayGetElement ( _obj_: a TypedArray, _index_: a Number, ): a Number, a BigInt, or *undefined*
  id: sec-typedarraygetelement

1. If IsValidIntegerIndex(_obj_, _index_) is *false*, return *undefined*.
2. Let _offset_ be _obj_.[[ByteOffset]].
3. Let _elementSize_ be TypedArrayElementSize(_obj_).
4. Let _byteIndexInBuffer_ be (ℝ(_index_) × _elementSize_) + _offset_.
5. Let _elementType_ be TypedArrayElementType(_obj_).
6. Return GetValueFromBuffer(_obj_.[[ViewedArrayBuffer]], _byteIndexInBuffer_, _elementType_, *true*, ~unordered~).


## Done when

- Every test262 run under built-ins/TypedArray/prototype/at/ passes (check with `./t262 built-ins/TypedArray/prototype/at/`). 6 pass on the main branch right now.

