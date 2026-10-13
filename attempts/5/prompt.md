# Ticket B-object-create: Object.create

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

- `crates/kahvi-builtins/src/object/create.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Object.create ( _proto_, _properties_ )
  id: sec-object.create   type: built-in function

This function creates a new object with a specified prototype.
It performs the following steps when called:

1. If _proto_ is not an Object and _proto_ is not *null*, throw a *TypeError* exception.
2. Let _obj_ be OrdinaryObjectCreate(_proto_).
3. If _properties_ is not *undefined*, then
   a. Return ? ObjectDefineProperties(_obj_, _properties_).
4. Return _obj_.

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

--- referenced: ObjectDefineProperties ( _obj_: an Object, _properties_: an ECMAScript language value, ): either a normal completion containing an Object or a throw completion
  id: sec-objectdefineproperties

1. Set _properties_ to ? ToObject(_properties_).
2. Let _keys_ be ? _properties_.[[OwnPropertyKeys]]().
3. Let _propertyDescs_ be a new empty List.
4. For each element _nextKey_ of _keys_, do
   a. Let _currentPropertyDesc_ be ? _properties_.[[GetOwnProperty]](_nextKey_).
   b. If _currentPropertyDesc_ is not *undefined* and _currentPropertyDesc_.[[Enumerable]] is *true*, then
      i. Let _propertyDescObj_ be ? Get(_properties_, _nextKey_).
      ii. Let _propertyDesc_ be ? ToPropertyDescriptor(_propertyDescObj_).
      iii. Append the Record { [[Key]]: _nextKey_, [[Descriptor]]: _propertyDesc_ } to _propertyDescs_.
5. For each element _property_ of _propertyDescs_, do
   a. Perform ? DefinePropertyOrThrow(_obj_, _property_.[[Key]], _property_.[[Descriptor]]).
6. Return _obj_.


## Done when

- Every test262 run under built-ins/Object/create/ passes (check with `./t262 built-ins/Object/create/`). 8 pass on the main branch right now.

