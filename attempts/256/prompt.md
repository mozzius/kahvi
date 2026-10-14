# Ticket B-object-get-own-property-descriptors: Object.getOwnPropertyDescriptors

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

- `crates/kahvi-builtins/src/object/get_own_property_descriptors.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Object.getOwnPropertyDescriptors ( _obj_ )
  id: sec-object.getownpropertydescriptors   type: built-in function

This function performs the following steps when called:

1. Let _coerced_ be ? ToObject(_obj_).
2. Let _ownKeys_ be ? _coerced_.[[OwnPropertyKeys]]().
3. Let _descs_ be OrdinaryObjectCreate(%Object.prototype%).
4. For each element _key_ of _ownKeys_, do
   a. Let _propertyDesc_ be ? _coerced_.[[GetOwnProperty]](_key_).
   b. Let _propertyDescObj_ be FromPropertyDescriptor(_propertyDesc_).
   c. If _propertyDescObj_ is not *undefined*, perform ! CreateDataPropertyOrThrow(_descs_, _key_, _propertyDescObj_).
5. Return _descs_.

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

--- referenced: OrdinaryObjectCreate ( _proto_: an Object or *null*, optional _additionalInternalSlotsList_: a List of names of internal slots, ): an Object
  id: sec-ordinaryobjectcreate

description: It is used to specify the runtime creation of new ordinary objects. _additionalInternalSlotsList_ contains the names of additional internal slots that must be defined as part of the object, beyond [[Prototype]] and [[Extensible]]. If _additionalInternalSlotsList_ is not provided, a new empty List is used.

1. Let _internalSlotsList_ be « [[Prototype]], [[Extensible]] ».
2. If _additionalInternalSlotsList_ is present, set _internalSlotsList_ to the list-concatenation of _internalSlotsList_ and _additionalInternalSlotsList_.
3. Let _obj_ be MakeBasicObject(_internalSlotsList_).
4. Set _obj_.[[Prototype]] to _proto_.
5. Return _obj_.

NOTE:
Although OrdinaryObjectCreate does little more than call MakeBasicObject, its use communicates the intention to create an ordinary object, and not an exotic one. Thus, within this specification, it is not called by any algorithm that subsequently modifies the internal methods of the object in ways that would make the result non-ordinary. Operations that create exotic objects invoke MakeBasicObject directly.

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

--- referenced: CreateDataPropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-createdatapropertyorthrow

description: It is used to create a new own property of an object. It throws a *TypeError* exception if the requested property update cannot be performed.

1. Let _success_ be ? CreateDataProperty(_obj_, _propertyKey_, _value_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.

NOTE:
This abstract operation creates a property whose attributes are set to the same defaults used for properties created by the ECMAScript language assignment operator. Normally, the property will not already exist. If it does exist and is not configurable or if _obj_ is not extensible, [[DefineOwnProperty]] will return *false* causing this operation to throw a *TypeError* exception.


## Done when

- Every test262 run under built-ins/Object/getOwnPropertyDescriptors/ passes (check with `./t262 built-ins/Object/getOwnPropertyDescriptors/`). 8 pass on the main branch right now.

