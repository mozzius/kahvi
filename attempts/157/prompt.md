# Ticket B-array-prototype-shift: Array.prototype.shift

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

- `crates/kahvi-builtins/src/array/prototype_shift.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Array.prototype.shift ( )
  id: sec-array.prototype.shift   type: built-in function

This method removes the first element of the array and returns it.
It performs the following steps when called:

1. Let _obj_ be ? ToObject(*this* value).
2. Let _length_ be ? LengthOfArrayLike(_obj_).
3. If _length_ = 0, then
   a. Perform ? Set(_obj_, *"length"*, *+0*_𝔽, *true*).
   b. Return *undefined*.
4. Let _first_ be ? Get(_obj_, *"0"*).
5. Let _k_ be 1.
6. Repeat, while _k_ < _length_,
   a. Let _from_ be ! ToString(𝔽(_k_)).
   b. Let _to_ be ! ToString(𝔽(_k_ - 1)).
   c. Let _fromPresent_ be ? HasProperty(_obj_, _from_).
   d. If _fromPresent_ is *true*, then
      i. Let _fromValue_ be ? Get(_obj_, _from_).
      ii. Perform ? Set(_obj_, _to_, _fromValue_, *true*).
   e. Else,
      i. Assert: _fromPresent_ is *false*.
      ii. Perform ? DeletePropertyOrThrow(_obj_, _to_).
   f. Set _k_ to _k_ + 1.
7. Perform ? DeletePropertyOrThrow(_obj_, ! ToString(𝔽(_length_ - 1))).
8. Perform ? Set(_obj_, *"length"*, 𝔽(_length_ - 1), *true*).
9. Return _first_.

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

--- referenced: Set ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, _throw_: a Boolean, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-set-o-p-v-throw

description: It is used to set the value of a specific property of an object. _value_ is the new value for the property.

1. Let _success_ be ? _obj_.[[Set]](_propertyKey_, _value_, _obj_).
2. If _success_ is *false* and _throw_ is *true*, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: Get ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-get-o-p

description: It is used to retrieve the value of a specific property of an object.

1. Return ? _obj_.[[Get]](_propertyKey_, _obj_).

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

--- referenced: DeletePropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-deletepropertyorthrow

description: It is used to remove a specific own property of an object. It throws an exception if the property is not configurable.

1. Let _success_ be ? _obj_.[[Delete]](_propertyKey_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.


## Done when

- Every test262 run under built-ins/Array/prototype/shift/ passes (check with `./t262 built-ins/Array/prototype/shift/`). 8 pass on the main branch right now.

