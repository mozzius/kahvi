# Ticket B-array-prototype-flat-map: Array.prototype.flatMap

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

- `crates/kahvi-builtins/src/array/prototype_flat_map.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Array.prototype.flatMap ( _mapperFunc_ [ , _thisArg_ ] )
  id: sec-array.prototype.flatmap   type: built-in function

This method performs the following steps when called:

1. Let _obj_ be ? ToObject(*this* value).
2. Let _sourceLength_ be ? LengthOfArrayLike(_obj_).
3. If IsCallable(_mapperFunc_) is *false*, throw a *TypeError* exception.
4. Let _array_ be ? ArraySpeciesCreate(_obj_, 0).
5. Perform ? FlattenIntoArray(_array_, _obj_, _sourceLength_, 0, 1, _mapperFunc_, _thisArg_).
6. Return _array_.

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

--- referenced: IsCallable ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-iscallable

description: It determines if _arg_ is a callable function with a [[Call]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Call]] internal method, return *true*.
3. Return *false*.

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

--- referenced: FlattenIntoArray ( _target_: an Object, _source_: an Object, _sourceLength_: a non-negative integer, _start_: a non-negative integer, _depth_: a non-negative integer or +∞, optional _mapperFunc_: a function object, optional _thisArg_: an ECMAScript language value, ): either a normal completion containing a non-negative integer or a throw completion
  id: sec-flattenintoarray

1. Assert: If _mapperFunc_ is present, then IsCallable(_mapperFunc_) is *true*, _thisArg_ is present, and _depth_ is 1.
2. Let _targetIndex_ be _start_.
3. Let _sourceIndex_ be 0.
4. Repeat, while _sourceIndex_ < _sourceLength_,
   a. Let _propertyKey_ be ! ToString(𝔽(_sourceIndex_)).
   b. Let _exists_ be ? HasProperty(_source_, _propertyKey_).
   c. If _exists_ is *true*, then
      i. Let _element_ be ? Get(_source_, _propertyKey_).
      ii. If _mapperFunc_ is present, then
         1. Set _element_ to ? Call(_mapperFunc_, _thisArg_, « _element_, 𝔽(_sourceIndex_), _source_ »).
      iii. Let _shouldFlatten_ be *false*.
      iv. If _depth_ > 0, then
         1. Set _shouldFlatten_ to ? IsArray(_element_).
      v. If _shouldFlatten_ is *true*, then
         1. If _depth_ = +∞, let _newDepth_ be +∞.
         2. Else, let _newDepth_ be _depth_ - 1.
         3. Let _elementLength_ be ? LengthOfArrayLike(_element_).
         4. Set _targetIndex_ to ? FlattenIntoArray(_target_, _element_, _elementLength_, _targetIndex_, _newDepth_).
      vi. Else,
         1. If _targetIndex_ ≥ 2^53 - 1, throw a *TypeError* exception.
         2. Perform ? CreateDataPropertyOrThrow(_target_, ! ToString(𝔽(_targetIndex_)), _element_).
         3. Set _targetIndex_ to _targetIndex_ + 1.
   d. Set _sourceIndex_ to _sourceIndex_ + 1.
5. Return _targetIndex_.


## Done when

- Every test262 run under built-ins/Array/prototype/flatMap/ passes (check with `./t262 built-ins/Array/prototype/flatMap/`). 8 pass on the main branch right now.

## Previous attempts

- attempt 190: worker reported BLOCKED but changed files: `Reflect.get` is not implemented (needed by `built-ins/Array/prototype/flatMap/proxy-access-count.js`)
- attempt 190: rejected by gates: lint: lint suppression is architect-only at crates/kahvi-builtins/src/array/prototype_flat_map.rs:58: #[allow(clippy::too_many_arguments)]

