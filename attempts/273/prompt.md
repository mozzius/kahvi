# Ticket B-suppressed-error-constructor: SuppressedError

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

- `crates/kahvi-builtins/src/suppressed_error/constructor.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

SuppressedError ( _error_, _suppressed_, _message_ )
  id: sec-suppressederror   type: built-in function

This function performs the following steps when called:

1. If NewTarget is *undefined*, let _newTarget_ be the active function object; else let _newTarget_ be NewTarget.
2. Let _obj_ be ? OrdinaryCreateFromConstructor(_newTarget_, *"%SuppressedError.prototype%"*, « [[ErrorData]] »).
3. If _message_ is not *undefined*, then
   a. Let _messageString_ be ? ToString(_message_).
   b. Perform CreateNonEnumerableDataPropertyOrThrow(_obj_, *"message"*, _messageString_).
4. Perform CreateNonEnumerableDataPropertyOrThrow(_obj_, *"error"*, _error_).
5. Perform CreateNonEnumerableDataPropertyOrThrow(_obj_, *"suppressed"*, _suppressed_).
6. Return _obj_.

--- referenced: OrdinaryCreateFromConstructor ( _ctor_: a function object, _intrinsicDefaultProto_: a String, optional _internalSlotsList_: a List of names of internal slots, ): either a normal completion containing an Object or a throw completion
  id: sec-ordinarycreatefromconstructor

description: It creates an ordinary object whose [[Prototype]] value is retrieved from a constructor's *"prototype"* property, if it exists. Otherwise the intrinsic named by _intrinsicDefaultProto_ is used for [[Prototype]]. _internalSlotsList_ contains the names of additional internal slots that must be defined as part of the object. If _internalSlotsList_ is not provided, a new empty List is used.

1. Assert: _intrinsicDefaultProto_ is this specification's name of an intrinsic object. The corresponding object must be an intrinsic that is intended to be used as the [[Prototype]] value of an object.
2. Let _proto_ be ? GetPrototypeFromConstructor(_ctor_, _intrinsicDefaultProto_).
3. If _internalSlotsList_ is present, let _slots_ be _internalSlotsList_.
4. Else, let _slots_ be a new empty List.
5. Return OrdinaryObjectCreate(_proto_, _slots_).

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

--- referenced: CreateNonEnumerableDataPropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, ): ~unused~
  id: sec-createnonenumerabledatapropertyorthrow

description: It is used to create a new non-enumerable own property of an ordinary object.

1. Assert: _obj_ is an ordinary, extensible object with no non-configurable properties.
2. Let _newDesc_ be the PropertyDescriptor { [[Value]]: _value_, [[Writable]]: *true*, [[Enumerable]]: *false*, [[Configurable]]: *true* }.
3. Perform ! DefinePropertyOrThrow(_obj_, _propertyKey_, _newDesc_).
4. Return ~unused~.

NOTE:
This abstract operation creates a property whose attributes are set to the same defaults used for properties created by the ECMAScript language assignment operator except it is not enumerable. Normally, the property will not already exist. If it does exist, DefinePropertyOrThrow is guaranteed to complete normally.


## Done when

- Every test262 run under built-ins/SuppressedError/ passes (check with `./t262 built-ins/SuppressedError/`). 20 pass on the main branch right now.

