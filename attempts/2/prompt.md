# Ticket B-array-prototype-concat: Array.prototype.concat

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

- `crates/kahvi-builtins/src/array/prototype_concat.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Array.prototype.concat ( ..._items_ )
  id: sec-array.prototype.concat   type: built-in function

This method returns an array containing the array elements of the object followed by the array elements of each argument.
It performs the following steps when called:

1. Let _obj_ be ? ToObject(*this* value).
2. Let _array_ be ? ArraySpeciesCreate(_obj_, 0).
3. Let _nextIndex_ be 0.
4. Prepend _obj_ to _items_.
5. For each element _item_ of _items_, do
   a. Let _spreadable_ be ? IsConcatSpreadable(_item_).
   b. If _spreadable_ is *true*, then
      i. Let _length_ be ? LengthOfArrayLike(_item_).
      ii. If _nextIndex_ + _length_ > 2^53 - 1, throw a *TypeError* exception.
      iii. Let _sourceIndex_ be 0.
      iv. Repeat, while _sourceIndex_ < _length_,
         1. Let _propertyKey_ be ! ToString(𝔽(_sourceIndex_)).
         2. Let _exists_ be ? HasProperty(_item_, _propertyKey_).
         3. If _exists_ is *true*, then
            a. Let _subElement_ be ? Get(_item_, _propertyKey_).
            b. Perform ? CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_nextIndex_)), _subElement_).
         4. Set _nextIndex_ to _nextIndex_ + 1.
         5. Set _sourceIndex_ to _sourceIndex_ + 1.
   c. Else,
      i. NOTE: _item_ is added as a single item rather than spread.
      ii. If _nextIndex_ ≥ 2^53 - 1, throw a *TypeError* exception.
      iii. Perform ? CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_nextIndex_)), _item_).
      iv. Set _nextIndex_ to _nextIndex_ + 1.
6. Perform ? Set(_array_, *"length"*, 𝔽(_nextIndex_), *true*).
7. Return _array_.

The *"length"* property of this method is *1*_𝔽.
NOTE:
The explicit setting of the *"length"* property in step [step-array-proto-concat-set-length] is intended to ensure the length is correct when the final non-empty element of _items_ has trailing holes or when _array_ is not a built-in Array.
NOTE:
This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
[subclause sec-isconcatspreadable: IsConcatSpreadable (
_obj_: an ECMAScript language value,
): either a normal completion containing a Boolean or a throw completion]

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

--- referenced: ArraySpeciesCreate ( _originalArray_: an Object, _length_: a non-negative integer, ): either a normal completion containing an Object or a throw completion
  id: sec-arrayspeciescreate

description: It is used to specify the creation of a new Array or similar object using a constructor function that is derived from _originalArray_. It does not enforce that the constructor function returns an Array.

1. Let _isArray_ be ? IsArray(_originalArray_).
2. If _isArray_ is *false*, return ? ArrayCreate(_length_).
3. Let _ctor_ be ? Get(_originalArray_, *"constructor"*).
4. If IsConstructor(_ctor_) is *true*, then
   a. Let _thisRealm_ be the current Realm Record.
   b. Let _ctorRealm_ be ? GetFunctionRealm(_ctor_).
   c. If _thisRealm_ and _ctorRealm_ are not the same Realm Record, then
      i. If SameValue(_ctor_, _ctorRealm_.[[Intrinsics]].[[%Array%]]) is *true*, set _ctor_ to *undefined*.
5. If _ctor_ is an Object, then
   a. Set _ctor_ to ? Get(_ctor_, %Symbol.species%).
   b. If _ctor_ is *null*, set _ctor_ to *undefined*.
6. If _ctor_ is *undefined*, return ? ArrayCreate(_length_).
7. If IsConstructor(_ctor_) is *false*, throw a *TypeError* exception.
8. Return ? Construct(_ctor_, « 𝔽(_length_) »).

NOTE:
If _originalArray_ was created using the standard built-in Array constructor for a realm that is not the realm of the running execution context, then a new Array is created using the realm of the running execution context. This maintains compatibility with Web browsers that have historically had that behaviour for the `Array.prototype` methods that now are defined using ArraySpeciesCreate.

--- referenced: IsConcatSpreadable ( _obj_: an ECMAScript language value, ): either a normal completion containing a Boolean or a throw completion
  id: sec-isconcatspreadable

1. If _obj_ is not an Object, return *false*.
2. Let _spreadable_ be ? Get(_obj_, %Symbol.isConcatSpreadable%).
3. If _spreadable_ is not *undefined*, return ToBoolean(_spreadable_).
4. Return ? IsArray(_obj_).

--- referenced: LengthOfArrayLike ( _obj_: an Object, ): either a normal completion containing a non-negative integer or a throw completion
  id: sec-lengthofarraylike

description: It returns the value of the *"length"* property of an array-like object.

1. Return ℝ(? ToLength(? Get(_obj_, *"length"*))).

An array-like object is any object for which this operation returns a normal completion.
NOTE:
Typically, an array-like object would also have some properties with integer index names. However, that is not a requirement of this definition.
NOTE:
Arrays and String objects are examples of array-like objects.

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

--- referenced: CreateDataPropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-createdatapropertyorthrow

description: It is used to create a new own property of an object. It throws a *TypeError* exception if the requested property update cannot be performed.

1. Let _success_ be ? CreateDataProperty(_obj_, _propertyKey_, _value_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.

NOTE:
This abstract operation creates a property whose attributes are set to the same defaults used for properties created by the ECMAScript language assignment operator. Normally, the property will not already exist. If it does exist and is not configurable or if _obj_ is not extensible, [[DefineOwnProperty]] will return *false* causing this operation to throw a *TypeError* exception.

--- referenced: Set ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, _throw_: a Boolean, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-set-o-p-v-throw

description: It is used to set the value of a specific property of an object. _value_ is the new value for the property.

1. Let _success_ be ? _obj_.[[Set]](_propertyKey_, _value_, _obj_).
2. If _success_ is *false* and _throw_ is *true*, throw a *TypeError* exception.
3. Return ~unused~.


## Done when

- Every test262 run under built-ins/Array/prototype/concat/ passes (check with `./t262 built-ins/Array/prototype/concat/`). 8 pass on the main branch right now.

