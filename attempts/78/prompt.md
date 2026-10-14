# Ticket B-object-assign: Object.assign

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

- `crates/kahvi-builtins/src/object/assign.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Object.assign ( _target_, ..._sources_ )
  id: sec-object.assign   type: built-in function

This function copies the values of all of the enumerable own properties from one or more source objects to a _target_ object.
It performs the following steps when called:

1. Let _targetObj_ be ? ToObject(_target_).
2. If only one argument was passed, return _targetObj_.
3. For each element _nextSource_ of _sources_, do
   a. If _nextSource_ is neither *undefined* nor *null*, then
      i. Let _from_ be ! ToObject(_nextSource_).
      ii. Let _keys_ be ? _from_.[[OwnPropertyKeys]]().
      iii. For each element _nextKey_ of _keys_, do
         1. Let _propertyDesc_ be ? _from_.[[GetOwnProperty]](_nextKey_).
         2. If _propertyDesc_ is not *undefined* and _propertyDesc_.[[Enumerable]] is *true*, then
            a. Let _propertyValue_ be ? Get(_from_, _nextKey_).
            b. Perform ? Set(_targetObj_, _nextKey_, _propertyValue_, *true*).
4. Return _targetObj_.

The *"length"* property of this function is *2*_𝔽.

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

--- referenced: Get ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-get-o-p

description: It is used to retrieve the value of a specific property of an object.

1. Return ? _obj_.[[Get]](_propertyKey_, _obj_).

--- referenced: Set ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, _throw_: a Boolean, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-set-o-p-v-throw

description: It is used to set the value of a specific property of an object. _value_ is the new value for the property.

1. Let _success_ be ? _obj_.[[Set]](_propertyKey_, _value_, _obj_).
2. If _success_ is *false* and _throw_ is *true*, throw a *TypeError* exception.
3. Return ~unused~.


## Done when

- Every test262 run under built-ins/Object/assign/ passes (check with `./t262 built-ins/Object/assign/`). 10 pass on the main branch right now.

