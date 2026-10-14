# Ticket B-typed-array-from: TypedArray.from

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

- `crates/kahvi-builtins/src/typed_array/from.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

%TypedArray%.from ( _source_ [ , _mapper_ [ , _thisArg_ ] ] )
  id: sec-%typedarray%.from   type: built-in function

This method performs the following steps when called:

1. Let _ctor_ be the *this* value.
2. If IsConstructor(_ctor_) is *false*, throw a *TypeError* exception.
3. If _mapper_ is *undefined*, then
   a. Let _mapping_ be *false*.
4. Else,
   a. If IsCallable(_mapper_) is *false*, throw a *TypeError* exception.
   b. Let _mapping_ be *true*.
5. Let _usingIterator_ be ? GetMethod(_source_, %Symbol.iterator%).
6. If _usingIterator_ is not *undefined*, then
   a. Let _values_ be ? IteratorToList(? GetIteratorFromMethod(_source_, _usingIterator_)).
   b. Let _length_ be the number of elements in _values_.
   c. Let _targetObj_ be ? TypedArrayCreateFromConstructor(_ctor_, « 𝔽(_length_) »).
   d. Let _k_ be 0.
   e. Repeat, while _k_ < _length_,
      i. Let _kValue_ be _values_[_k_].
      ii. If _mapping_ is *true*, then
         1. Let _mappedValue_ be ? Call(_mapper_, _thisArg_, « _kValue_, 𝔽(_k_) »).
      iii. Else,
         1. Let _mappedValue_ be _kValue_.
      iv. Perform ? TypedArraySetElement(_targetObj_, 𝔽(_k_), _mappedValue_).
      v. Set _k_ to _k_ + 1.
   f. Return _targetObj_.
7. NOTE: _source_ is not an iterable object, so assume it is already an array-like object.
8. Let _arrayLike_ be ! ToObject(_source_).
9. Let _length_ be ? LengthOfArrayLike(_arrayLike_).
10. Let _targetObj_ be ? TypedArrayCreateFromConstructor(_ctor_, « 𝔽(_length_) »).
11. Let _k_ be 0.
12. Repeat, while _k_ < _length_,
   a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
   b. Let _kValue_ be ? Get(_arrayLike_, _propertyKey_).
   c. If _mapping_ is *true*, then
      i. Let _mappedValue_ be ? Call(_mapper_, _thisArg_, « _kValue_, 𝔽(_k_) »).
   d. Else,
      i. Let _mappedValue_ be _kValue_.
   e. Perform ? TypedArraySetElement(_targetObj_, 𝔽(_k_), _mappedValue_).
   f. Set _k_ to _k_ + 1.
13. Return _targetObj_.

--- referenced: IsConstructor ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-isconstructor

description: It determines if _arg_ is a function object with a [[Construct]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Construct]] internal method, return *true*.
3. Return *false*.

--- referenced: IsCallable ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-iscallable

description: It determines if _arg_ is a callable function with a [[Call]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Call]] internal method, return *true*.
3. Return *false*.

--- referenced: GetMethod ( _value_: an ECMAScript language value, _propertyKey_: a property key, ): either a normal completion containing either a function object or *undefined*, or a throw completion
  id: sec-getmethod

description: It is used to get the value of a specific property of an ECMAScript language value when the value of the property is expected to be a function.

1. Let _func_ be ? GetV(_value_, _propertyKey_).
2. If _func_ is either *undefined* or *null*, return *undefined*.
3. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
4. Return _func_.

--- referenced: IteratorToList ( _iteratorRecord_: an Iterator Record, ): either a normal completion containing a List of ECMAScript language values or a throw completion
  id: sec-iteratortolist

1. Let _values_ be a new empty List.
2. Repeat,
   a. Let _next_ be ? IteratorStepValue(_iteratorRecord_).
   b. If _next_ is ~done~, then
      i. Return _values_.
   c. Append _next_ to _values_.

--- referenced: GetIteratorFromMethod ( _obj_: an ECMAScript language value, _method_: a function object, ): either a normal completion containing an Iterator Record or a throw completion
  id: sec-getiteratorfrommethod

1. Let _iterator_ be ? Call(_method_, _obj_).
2. If _iterator_ is not an Object, throw a *TypeError* exception.
3. Return ? GetIteratorDirect(_iterator_).

--- referenced: TypedArrayCreateFromConstructor ( _ctor_: a constructor, _argList_: a List of ECMAScript language values, ): either a normal completion containing a TypedArray or a throw completion
  id: sec-typedarraycreatefromconstructor

description: It is used to specify the creation of a new TypedArray using a constructor function.

1. Let _ta_ be ? Construct(_ctor_, _argList_).
2. Let _taRecord_ be ? ValidateTypedArray(_ta_, ~seq-cst~).
3. Assert: _ta_ has all the internal slots mentioned in [sec-properties-of-typedarray-instances].
4. If the number of elements in _argList_ is 1 and _argList_[0] is a Number, then
   a. If IsTypedArrayOutOfBounds(_taRecord_) is *true*, throw a *TypeError* exception.
   b. Let _length_ be TypedArrayLength(_taRecord_).
   c. If _length_ < ℝ(_argList_[0]), throw a *TypeError* exception.
5. Return _ta_.

--- referenced: Call ( _func_: an ECMAScript language value, _thisValue_: an ECMAScript language value, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-call

description: It is used to call the [[Call]] internal method of a function object. _func_ is the function object, _thisValue_ is an ECMAScript language value that is the *this* value of the [[Call]], and _argList_ is the value passed to the corresponding argument of the internal method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
3. Return ? _func_.[[Call]](_thisValue_, _argList_).

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

--- referenced: ToObject ( _arg_: an ECMAScript language value, ): either a normal completion containing an Object or a throw completion
  id: sec-toobject

description: It converts _arg_ to a value of type Object.

1. If _arg_ is either *undefined* or *null*, throw a *TypeError* exception.
2. If _arg_ is a Boolean, return a new Boolean object whose [[BooleanData]] internal slot is set to _arg_. See [sec-boolean-objects] for a description of Boolean objects.
3. If _arg_ is a Number, return a new Number object whose [[NumberData]] internal slot is set to _arg_. See [sec-number-objects] for a description of Number objects.
4. If _arg_ is a String, return a new String object whose [[StringData]] internal slot is set to _arg_. See [sec-string-objects] for a description of String objects.
5. If _arg_ is a Symbol, return a new Symbol object whose [[SymbolData]] internal slot is set to _arg_. See [sec-symbol-objects] for a description of Symbol objects.
6. If _arg_ is a BigInt, return a new BigInt object whose [[BigIntData]] internal slot is set to _arg_. See [sec-bigint-objects] for a description of BigInt objects.
7. Assert: _arg_ is an Object.
8. Return _arg_.

--- referenced: LengthOfArrayLike ( _obj_: an Object, ): either a normal completion containing a non-negative integer or a throw completion
  id: sec-lengthofarraylike

description: It returns the value of the *"length"* property of an array-like object.

1. Return ℝ(? ToLength(? Get(_obj_, *"length"*))).

An array-like object is any object for which this operation returns a normal completion.
NOTE:
Typically, an array-like object would also have some properties with integer index names. However, that is not a requirement of this definition.
NOTE:
Arrays and String objects are examples of array-like objects.


## Done when

- Every test262 run under built-ins/TypedArray/from/ passes (check with `./t262 built-ins/TypedArray/from/`). 8 pass on the main branch right now.

