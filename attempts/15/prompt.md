# Ticket B-array-prototype-index-of: Array.prototype.indexOf

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

- `crates/kahvi-builtins/src/array/prototype_index_of.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Array.prototype.indexOf ( _searchElement_ [ , _fromIndex_ ] )
  id: sec-array.prototype.indexof   type: built-in function

This method compares _searchElement_ to the elements of the array, in ascending order, using the IsStrictlyEqual algorithm, and if found at one or more indices, returns the smallest such index; otherwise, it returns *-1*_𝔽.
NOTE:
The optional second argument _fromIndex_ defaults to *+0*_𝔽 (i.e. the whole array is searched). If it is greater than or equal to the length of the array, *-1*_𝔽 is returned, i.e. the array will not be searched. If it is less than *-0*_𝔽, it is used to compute an offset from the end of the array. If the computed index is less than or equal to *+0*_𝔽, the whole array will be searched.
This method performs the following steps when called:

1. Let _obj_ be ? ToObject(*this* value).
2. Let _length_ be ? LengthOfArrayLike(_obj_).
3. If _length_ = 0, return *-1*_𝔽.
4. Let _k_ be ? ToClampedIndex(_fromIndex_, _length_).
5. Repeat, while _k_ < _length_,
   a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
   b. Let _kPresent_ be ? HasProperty(_obj_, _propertyKey_).
   c. If _kPresent_ is *true*, then
      i. Let _elementK_ be ? Get(_obj_, _propertyKey_).
      ii. If IsStrictlyEqual(_searchElement_, _elementK_) is *true*, return 𝔽(_k_).
   d. Set _k_ to _k_ + 1.
6. Return *-1*_𝔽.

NOTE:
This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.

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

--- referenced: Get ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-get-o-p

description: It is used to retrieve the value of a specific property of an object.

1. Return ? _obj_.[[Get]](_propertyKey_, _obj_).

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

- Every test262 run under built-ins/Array/prototype/indexOf/ passes (check with `./t262 built-ins/Array/prototype/indexOf/`). 8 pass on the main branch right now.

