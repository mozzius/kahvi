# Ticket B-typed-array-prototype-sort: TypedArray.prototype.sort

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

- `crates/kahvi-builtins/src/typed_array/prototype_sort.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

%TypedArray%.prototype.sort ( _comparator_ )
  id: sec-%typedarray%.prototype.sort   type: built-in function

This is a distinct method that, except as described below, implements the same requirements as those of `Array.prototype.sort` as defined in [sec-array.prototype.sort]. The implementation of this method may be optimized with the knowledge that the *this* value is an object that has a fixed length and whose integer-indexed properties are not sparse.
This method is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.
It performs the following steps when called:

1. If _comparator_ is not *undefined* and IsCallable(_comparator_) is *false*, throw a *TypeError* exception.
2. Let _obj_ be the *this* value.
3. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
4. Let _length_ be TypedArrayLength(_taRecord_).
5. NOTE: The following closure performs a numeric comparison rather than the string comparison used in [sec-array.prototype.sort].
6. Let _sortCompare_ be a new Abstract Closure with parameters (_x_, _y_) that captures _comparator_ and performs the following steps when called:
   a. Return ? CompareTypedArrayElements(_x_, _y_, _comparator_).
7. Let _sortedList_ be ? SortIndexedProperties(_obj_, _length_, _sortCompare_, ~read-through-holes~).
8. Let _j_ be 0.
9. Repeat, while _j_ < _length_,
   a. Perform ! TypedArraySetElement(_obj_, 𝔽(_j_), _sortedList_[_j_]).
   b. Set _j_ to _j_ + 1.
10. Return _obj_.

NOTE:
Because *NaN* always compares greater than any other value (see CompareTypedArrayElements), *NaN* property values always sort to the end of the result when _comparator_ is not provided.

--- referenced: IsCallable ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-iscallable

description: It determines if _arg_ is a callable function with a [[Call]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Call]] internal method, return *true*.
3. Return *false*.

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

--- referenced: CompareTypedArrayElements ( _x_: a Number or a BigInt, _y_: a Number or a BigInt, _comparator_: a function object or *undefined*, ): either a normal completion containing a Number or an abrupt completion
  id: sec-comparetypedarrayelements

1. Assert: _x_ is a Number and _y_ is a Number, or _x_ is a BigInt and _y_ is a BigInt.
2. If _comparator_ is not *undefined*, then
   a. Let _result_ be ? ToNumber(? Call(_comparator_, *undefined*, « _x_, _y_ »)).
   b. If _result_ is *NaN*, return *+0*_𝔽.
   c. Return _result_.
3. If _x_ is *NaN* and _y_ is *NaN*, return *+0*_𝔽.
4. If _x_ is *NaN*, return *1*_𝔽.
5. If _y_ is *NaN*, return *-1*_𝔽.
6. If _x_ < _y_, return *-1*_𝔽.
7. If _x_ > _y_, return *1*_𝔽.
8. If _x_ is *-0*_𝔽 and _y_ is *+0*_𝔽, return *-1*_𝔽.
9. If _x_ is *+0*_𝔽 and _y_ is *-0*_𝔽, return *1*_𝔽.
10. Return *+0*_𝔽.

NOTE:
This performs a numeric comparison rather than the string comparison used in [sec-comparearrayelements].

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

- Every test262 run under built-ins/TypedArray/prototype/sort/ passes (check with `./t262 built-ins/TypedArray/prototype/sort/`). 8 pass on the main branch right now.

