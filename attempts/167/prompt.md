# Ticket B-typed-array-prototype-with: TypedArray.prototype.with

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

- `crates/kahvi-builtins/src/typed_array/prototype_with.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

%TypedArray%.prototype.with ( _index_, _value_ )
  id: sec-%typedarray%.prototype.with   type: built-in function

This method performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
3. Let _length_ be TypedArrayLength(_taRecord_).
4. Let _actualIndex_ be ? ToAbsoluteIndex(_index_, _length_).
5. If _obj_.[[ContentType]] is ~bigint~, let _numericValue_ be ? ToBigInt(_value_).
6. Else, let _numericValue_ be ? ToNumber(_value_).
7. If IsValidIntegerIndex(_obj_, 𝔽(_actualIndex_)) is *false*, throw a *RangeError* exception.
8. Let _resultArray_ be ? TypedArrayCreateSameType(_obj_, _length_).
9. Let _k_ be 0.
10. Repeat, while _k_ < _length_,
   a. If _k_ = _actualIndex_, let _fromValue_ be _numericValue_.
   b. Else, let _fromValue_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
   c. Perform ! TypedArraySetElement(_resultArray_, 𝔽(_k_), _fromValue_).
   d. Set _k_ to _k_ + 1.
11. Return _resultArray_.

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

--- referenced: ToBigInt ( _arg_: an ECMAScript language value, ): either a normal completion containing a BigInt or a throw completion
  id: sec-tobigint

description: It converts _arg_ to a BigInt value, or throws if an implicit conversion from Number would be required.

1. Let _primitive_ be ? ToPrimitive(_arg_, ~number~).
2. Return the value that _primitive_ corresponds to in [table-tobigint].

Argument Type
|
Result
|
Undefined
|
Throw a *TypeError* exception.
|
Null
|
Throw a *TypeError* exception.
|
Boolean
|
Return `1n` if _primitive_ is *true* and `0n` if _primitive_ is *false*.
|
BigInt
|
Return _primitive_.
|
Number
|
Throw a *TypeError* exception.
|
String
|

1. Let _n_ be StringToBigInt(_primitive_).
2. If _n_ is *undefined*, throw a *SyntaxError* exception.
3. Return _n_.

|
Symbol
|
Throw a *TypeError* exception.
|

--- referenced: ToNumber ( _arg_: an ECMAScript language value, ): either a normal completion containing a Number or a throw completion
  id: sec-tonumber

description: It converts _arg_ to a value of type Number.

1. If _arg_ is a Number, return _arg_.
2. If _arg_ is either a Symbol or a BigInt, throw a *TypeError* exception.
3. If _arg_ is *undefined*, return *NaN*.
4. If _arg_ is either *null* or *false*, return *+0*_𝔽.
5. If _arg_ is *true*, return *1*_𝔽.
6. If _arg_ is a String, return StringToNumber(_arg_).
7. Assert: _arg_ is an Object.
8. Let _primitiveValue_ be ? ToPrimitive(_arg_, ~number~).
9. Assert: _primitiveValue_ is not an Object.
10. Return ? ToNumber(_primitiveValue_).

[subclause sec-tonumber-applied-to-the-string-type: ToNumber Applied to the String Type]

--- referenced: IsValidIntegerIndex ( _obj_: a TypedArray, _index_: a Number, ): a Boolean
  id: sec-isvalidintegerindex

1. If IsDetachedBuffer(_obj_.[[ViewedArrayBuffer]]) is *true*, return *false*.
2. If _index_ is not an integral Number, return *false*.
3. If _index_ is *-0*_𝔽 or _index_ < *-0*_𝔽, return *false*.
4. Let _taRecord_ be MakeTypedArrayWithBufferWitnessRecord(_obj_, ~unordered~).
5. NOTE: Bounds checking is not a synchronizing operation when _obj_'s backing buffer is a growable SharedArrayBuffer.
6. If IsTypedArrayOutOfBounds(_taRecord_) is *true*, return *false*.
7. Let _length_ be TypedArrayLength(_taRecord_).
8. If ℝ(_index_) ≥ _length_, return *false*.
9. Return *true*.

--- referenced: TypedArrayCreateSameType ( _exemplar_: a TypedArray, _length_: a non-negative integer, ): either a normal completion containing a TypedArray or a throw completion
  id: sec-typedarray-create-same-type

description: It is used to specify the creation of a new TypedArray using a constructor function that is derived from _exemplar_. Unlike TypedArraySpeciesCreate, which can construct custom TypedArray subclasses through the use of %Symbol.species%, this operation always uses one of the built-in TypedArray constructors.

1. Let _ctor_ be the intrinsic object associated with the constructor name _exemplar_.[[TypedArrayName]] in [table-the-typedarray-constructors].
2. Let _result_ be ? TypedArrayCreateFromConstructor(_ctor_, « 𝔽(_length_) »).
3. Assert: _result_ has [[TypedArrayName]] and [[ContentType]] internal slots.
4. Assert: _result_.[[ContentType]] is _exemplar_.[[ContentType]].
5. Return _result_.

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

- Every test262 run under built-ins/TypedArray/prototype/with/ passes (check with `./t262 built-ins/TypedArray/prototype/with/`). 8 pass on the main branch right now.

