# Ticket B-reflect-get-own-property-descriptor: Reflect.getOwnPropertyDescriptor

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

- `crates/kahvi-builtins/src/reflect/get_own_property_descriptor.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Reflect.getOwnPropertyDescriptor ( _target_, _key_ )
  id: sec-reflect.getownpropertydescriptor   type: built-in function

This function performs the following steps when called:

1. If _target_ is not an Object, throw a *TypeError* exception.
2. Let _propertyKey_ be ? ToPropertyKey(_key_).
3. Let _propertyDesc_ be ? _target_.[[GetOwnProperty]](_propertyKey_).
4. Return FromPropertyDescriptor(_propertyDesc_).

--- referenced: ToPropertyKey ( _arg_: an ECMAScript language value, ): either a normal completion containing a property key or a throw completion
  id: sec-topropertykey

description: It converts _arg_ to a value that can be used as a property key.

1. Let _key_ be ? ToPrimitive(_arg_, ~string~).
2. If _key_ is a Symbol, then
   a. Return _key_.
3. Return ! ToString(_key_).

--- referenced: FromPropertyDescriptor ( _propertyDesc_: a Property Descriptor or *undefined*, ): an Object or *undefined*
  id: sec-frompropertydescriptor

1. If _propertyDesc_ is *undefined*, return *undefined*.
2. Let _obj_ be OrdinaryObjectCreate(%Object.prototype%).
3. Assert: _obj_ is an extensible ordinary object with no own properties.
4. If _propertyDesc_ has a [[Value]] field, then
   a. Perform ! CreateDataPropertyOrThrow(_obj_, *"value"*, _propertyDesc_.[[Value]]).
5. If _propertyDesc_ has a [[Writable]] field, then
   a. Perform ! CreateDataPropertyOrThrow(_obj_, *"writable"*, _propertyDesc_.[[Writable]]).
6. If _propertyDesc_ has a [[Getter]] field, then
   a. Perform ! CreateDataPropertyOrThrow(_obj_, *"get"*, _propertyDesc_.[[Getter]]).
7. If _propertyDesc_ has a [[Setter]] field, then
   a. Perform ! CreateDataPropertyOrThrow(_obj_, *"set"*, _propertyDesc_.[[Setter]]).
8. If _propertyDesc_ has an [[Enumerable]] field, then
   a. Perform ! CreateDataPropertyOrThrow(_obj_, *"enumerable"*, _propertyDesc_.[[Enumerable]]).
9. If _propertyDesc_ has a [[Configurable]] field, then
   a. Perform ! CreateDataPropertyOrThrow(_obj_, *"configurable"*, _propertyDesc_.[[Configurable]]).
10. Return _obj_.


## Done when

- Every test262 run under built-ins/Reflect/getOwnPropertyDescriptor/ passes (check with `./t262 built-ins/Reflect/getOwnPropertyDescriptor/`). 8 pass on the main branch right now.

