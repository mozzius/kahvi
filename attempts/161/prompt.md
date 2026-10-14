# Ticket B-object-from-entries: Object.fromEntries

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

- `crates/kahvi-builtins/src/object/from_entries.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Object.fromEntries ( _iterable_ )
  id: sec-object.fromentries   type: built-in function

This function performs the following steps when called:

1. Perform ? RequireObjectCoercible(_iterable_).
2. Let _obj_ be OrdinaryObjectCreate(%Object.prototype%).
3. Assert: _obj_ is an extensible ordinary object with no own properties.
4. Let _closure_ be a new Abstract Closure with parameters (_key_, _value_) that captures _obj_ and performs the following steps when called:
   a. Let _propertyKey_ be ? ToPropertyKey(_key_).
   b. Perform ! CreateDataPropertyOrThrow(_obj_, _propertyKey_, _value_).
   c. Return NormalCompletion(*undefined*).
5. Let _adder_ be CreateBuiltinFunction(_closure_, 2, *""*, « »).
6. Return ? AddEntriesFromIterable(_obj_, _iterable_, _adder_).

NOTE:
The function created for _adder_ is never directly accessible to ECMAScript code.

--- referenced: RequireObjectCoercible ( _arg_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireobjectcoercible

description: It throws an error if _arg_ is a value that cannot be converted to an Object using ToObject.

1. If _arg_ is either *undefined* or *null*, throw a *TypeError* exception.
2. Return ~unused~.

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

--- referenced: ToPropertyKey ( _arg_: an ECMAScript language value, ): either a normal completion containing a property key or a throw completion
  id: sec-topropertykey

description: It converts _arg_ to a value that can be used as a property key.

1. Let _key_ be ? ToPrimitive(_arg_, ~string~).
2. If _key_ is a Symbol, then
   a. Return _key_.
3. Return ! ToString(_key_).

--- referenced: CreateDataPropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-createdatapropertyorthrow

description: It is used to create a new own property of an object. It throws a *TypeError* exception if the requested property update cannot be performed.

1. Let _success_ be ? CreateDataProperty(_obj_, _propertyKey_, _value_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.

NOTE:
This abstract operation creates a property whose attributes are set to the same defaults used for properties created by the ECMAScript language assignment operator. Normally, the property will not already exist. If it does exist and is not configurable or if _obj_ is not extensible, [[DefineOwnProperty]] will return *false* causing this operation to throw a *TypeError* exception.

--- referenced: NormalCompletion ( _value_: any value except a Completion Record, ): a normal completion
  id: sec-normalcompletion

skip return checks: true

1. Return Completion Record { [[Type]]: ~normal~, [[Value]]: _value_, [[Target]]: ~empty~ }.

--- referenced: AddEntriesFromIterable ( _target_: an Object, _iterable_: an ECMAScript language value, but not *undefined* or *null*, _adder_: a function object, ): either a normal completion containing an Object or a throw completion
  id: sec-add-entries-from-iterable

description: _adder_ will be invoked, with _target_ as the receiver.

1. Let _iteratorRecord_ be ? GetIterator(_iterable_, ~sync~).
2. Repeat,
   a. Let _next_ be ? IteratorStepValue(_iteratorRecord_).
   b. If _next_ is ~done~, return _target_.
   c. If _next_ is not an Object, then
      i. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
      ii. Return ? IteratorClose(_iteratorRecord_, _error_).
   d. Let _key_ be Completion(Get(_next_, *"0"*)).
   e. IfAbruptCloseIterator(_key_, _iteratorRecord_).
   f. Let _value_ be Completion(Get(_next_, *"1"*)).
   g. IfAbruptCloseIterator(_value_, _iteratorRecord_).
   h. Let _status_ be Completion(Call(_adder_, _target_, « _key_, _value_ »)).
   i. IfAbruptCloseIterator(_status_, _iteratorRecord_).

NOTE:
The parameter _iterable_ is expected to be an object that implements a %Symbol.iterator% method that returns an iterator object that produces a two element array-like object whose first element is a value that will be used as a Map key and whose second element is the value to associate with that key.


## Done when

- Every test262 run under built-ins/Object/fromEntries/ passes (check with `./t262 built-ins/Object/fromEntries/`). 6 pass on the main branch right now.

