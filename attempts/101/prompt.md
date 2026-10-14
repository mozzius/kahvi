# Ticket B-typed-array-prototype-filter: TypedArray.prototype.filter

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

- `crates/kahvi-builtins/src/typed_array/prototype_filter.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

%TypedArray%.prototype.filter ( _callback_ [ , _thisArg_ ] )
  id: sec-%typedarray%.prototype.filter   type: built-in function

The interpretation and use of the arguments of this method are the same as for `Array.prototype.filter` as defined in [sec-array.prototype.filter].
This method performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
3. Let _length_ be TypedArrayLength(_taRecord_).
4. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
5. Let _kept_ be a new empty List.
6. Let _captured_ be 0.
7. Let _k_ be 0.
8. Repeat, while _k_ < _length_,
   a. Let _kValue_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
   b. Let _selected_ be ToBoolean(? Call(_callback_, _thisArg_, « _kValue_, 𝔽(_k_), _obj_ »)).
   c. If _selected_ is *true*, then
      i. Append _kValue_ to _kept_.
      ii. Set _captured_ to _captured_ + 1.
   d. Set _k_ to _k_ + 1.
9. Let _result_ be ? TypedArraySpeciesCreate(_obj_, « 𝔽(_captured_) »).
10. Let _n_ be 0.
11. For each element _element_ of _kept_, do
   a. Perform ! TypedArraySetElement(_result_, 𝔽(_n_), _element_).
   b. Set _n_ to _n_ + 1.
12. Return _result_.

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

--- referenced: IsCallable ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-iscallable

description: It determines if _arg_ is a callable function with a [[Call]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Call]] internal method, return *true*.
3. Return *false*.

--- referenced: TypedArrayGetElement ( _obj_: a TypedArray, _index_: a Number, ): a Number, a BigInt, or *undefined*
  id: sec-typedarraygetelement

1. If IsValidIntegerIndex(_obj_, _index_) is *false*, return *undefined*.
2. Let _offset_ be _obj_.[[ByteOffset]].
3. Let _elementSize_ be TypedArrayElementSize(_obj_).
4. Let _byteIndexInBuffer_ be (ℝ(_index_) × _elementSize_) + _offset_.
5. Let _elementType_ be TypedArrayElementType(_obj_).
6. Return GetValueFromBuffer(_obj_.[[ViewedArrayBuffer]], _byteIndexInBuffer_, _elementType_, *true*, ~unordered~).

--- referenced: ToBoolean ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-toboolean

description: It converts _arg_ to a value of type Boolean.

1. If _arg_ is a Boolean, return _arg_.
2. If _arg_ is one of *undefined*, *null*, *+0*_𝔽, *-0*_𝔽, *NaN*, *0*_ℤ, or the empty String, return *false*.
3. If the host is a web browser or otherwise supports [sec-IsHTMLDDA-internal-slot], then
   a. If _arg_ is an Object and _arg_ has an [[IsHTMLDDA]] internal slot, return *false*.
4. Return *true*.

--- referenced: Call ( _func_: an ECMAScript language value, _thisValue_: an ECMAScript language value, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-call

description: It is used to call the [[Call]] internal method of a function object. _func_ is the function object, _thisValue_ is an ECMAScript language value that is the *this* value of the [[Call]], and _argList_ is the value passed to the corresponding argument of the internal method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
3. Return ? _func_.[[Call]](_thisValue_, _argList_).

--- referenced: TypedArraySpeciesCreate ( _exemplar_: a TypedArray, _argList_: a List of ECMAScript language values, ): either a normal completion containing a TypedArray or a throw completion
  id: typedarray-species-create

description: It is used to specify the creation of a new TypedArray using a constructor function that is derived from _exemplar_. Unlike ArraySpeciesCreate, which can create non-Array objects through the use of %Symbol.species%, this operation enforces that the constructor function creates an actual TypedArray.

1. Let _defaultCtor_ be the intrinsic object associated with the constructor name _exemplar_.[[TypedArrayName]] in [table-the-typedarray-constructors].
2. Let _ctor_ be ? SpeciesConstructor(_exemplar_, _defaultCtor_).
3. Let _result_ be ? TypedArrayCreateFromConstructor(_ctor_, _argList_).
4. If _result_.[[ContentType]] is not _exemplar_.[[ContentType]], throw a *TypeError* exception.
5. Return _result_.

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

- Every test262 run under built-ins/TypedArray/prototype/filter/ passes (check with `./t262 built-ins/TypedArray/prototype/filter/`). 8 pass on the main branch right now.

