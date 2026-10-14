# Ticket B-typed-array-prototype-index-of: TypedArray.prototype.indexOf

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

- `crates/kahvi-builtins/src/typed_array/prototype_index_of.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

%TypedArray%.prototype.indexOf ( _searchElement_ [ , _fromIndex_ ] )
  id: sec-%typedarray%.prototype.indexof   type: built-in function

The interpretation and use of the arguments of this method are the same as for `Array.prototype.indexOf` as defined in [sec-array.prototype.indexof].
This method performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
3. Let _length_ be TypedArrayLength(_taRecord_).
4. If _length_ = 0, return *-1*_𝔽.
5. Let _k_ be ? ToClampedIndex(_fromIndex_, _length_).
6. Repeat, while _k_ < _length_,
   a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
   b. Let _kPresent_ be ! HasProperty(_obj_, _propertyKey_).
   c. If _kPresent_ is *true*, then
      i. Let _elementK_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
      ii. If IsStrictlyEqual(_searchElement_, _elementK_) is *true*, return 𝔽(_k_).
   d. Set _k_ to _k_ + 1.
7. Return *-1*_𝔽.

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

--- referenced: ToString ( _arg_: an ECMAScript language value, ): either a normal completion containing a String or a throw completion
  id: sec-tostring

description: It converts _arg_ to a value of type String.

1. If _arg_ is a String, return _arg_.
2. If _arg_ is a Symbol, throw a *TypeError* exception.
3. If _arg_ is *undefined*, return *"undefined"*.
4. If _arg_ is *null*, return *"null"*.
5. If _arg_ is *true*, return *"true"*.
6. If _arg_ is *false*, return *"false"*.
7. If _arg_ is a Number, return Number::toString(_arg_, 10).
8. If _arg_ is a BigInt, return BigInt::toString(_arg_, 10).
9. Assert: _arg_ is an Object.
10. Let _primitiveValue_ be ? ToPrimitive(_arg_, ~string~).
11. Assert: _primitiveValue_ is not an Object.
12. Return ? ToString(_primitiveValue_).

--- referenced: HasProperty ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing a Boolean or a throw completion
  id: sec-hasproperty

description: It is used to determine whether an object has a property with the specified property key. The property may be either own or inherited.

1. Return ? _obj_.[[HasProperty]](_propertyKey_).

--- referenced: TypedArrayGetElement ( _obj_: a TypedArray, _index_: a Number, ): a Number, a BigInt, or *undefined*
  id: sec-typedarraygetelement

1. If IsValidIntegerIndex(_obj_, _index_) is *false*, return *undefined*.
2. Let _offset_ be _obj_.[[ByteOffset]].
3. Let _elementSize_ be TypedArrayElementSize(_obj_).
4. Let _byteIndexInBuffer_ be (ℝ(_index_) × _elementSize_) + _offset_.
5. Let _elementType_ be TypedArrayElementType(_obj_).
6. Return GetValueFromBuffer(_obj_.[[ViewedArrayBuffer]], _byteIndexInBuffer_, _elementType_, *true*, ~unordered~).

--- referenced: IsStrictlyEqual ( _x_: an ECMAScript language value, _y_: an ECMAScript language value, ): a Boolean
  id: sec-isstrictlyequal

description: It provides the semantics for the `===` operator.

1. If SameType(_x_, _y_) is *false*, return *false*.
2. If _x_ is a Number, then
   a. Return Number::equal(_x_, _y_).
3. Return SameValueNonNumber(_x_, _y_).

NOTE:
This algorithm differs from the SameValue Algorithm in its treatment of signed zeroes and NaNs.


## Done when

- Every test262 run under built-ins/TypedArray/prototype/indexOf/ passes (check with `./t262 built-ins/TypedArray/prototype/indexOf/`). 8 pass on the main branch right now.

