# Ticket B-object-has-own: Object.hasOwn

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

- `crates/kahvi-builtins/src/object/has_own.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Object.hasOwn ( _obj_, _key_ )
  id: sec-object.hasown   type: built-in function

This function performs the following steps when called:

1. Let _coerced_ be ? ToObject(_obj_).
2. Let _propertyKey_ be ? ToPropertyKey(_key_).
3. Return ? HasOwnProperty(_coerced_, _propertyKey_).

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

--- referenced: ToPropertyKey ( _arg_: an ECMAScript language value, ): either a normal completion containing a property key or a throw completion
  id: sec-topropertykey

description: It converts _arg_ to a value that can be used as a property key.

1. Let _key_ be ? ToPrimitive(_arg_, ~string~).
2. If _key_ is a Symbol, then
   a. Return _key_.
3. Return ! ToString(_key_).

--- referenced: HasOwnProperty ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing a Boolean or a throw completion
  id: sec-hasownproperty

description: It is used to determine whether an object has an own property with the specified property key.

1. Let _propertyDesc_ be ? _obj_.[[GetOwnProperty]](_propertyKey_).
2. If _propertyDesc_ is *undefined*, return *false*.
3. Return *true*.


## Done when

- Every test262 run under built-ins/Object/hasOwn/ passes (check with `./t262 built-ins/Object/hasOwn/`). 10 pass on the main branch right now.

