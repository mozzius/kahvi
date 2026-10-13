# Ticket B-weak-map-constructor: WeakMap

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

- `crates/kahvi-builtins/src/weak_map/constructor.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/collections.rs`
- `crates/kahvi-builtins/src/map/prototype_set.rs`
- `crates/kahvi-builtins/src/map_iterator_prototype/next.rs`

## Spec

WeakMap ( [ _iterable_ ] )
  id: sec-weakmap-iterable   type: built-in function

This function performs the following steps when called:

1. If NewTarget is *undefined*, throw a *TypeError* exception.
2. Let _map_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%WeakMap.prototype%"*, « [[WeakMapData]] »).
3. Set _map_.[[WeakMapData]] to a new empty List.
4. If _iterable_ is either *undefined* or *null*, return _map_.
5. Let _adder_ be ? Get(_map_, *"set"*).
6. If IsCallable(_adder_) is *false*, throw a *TypeError* exception.
7. Return ? AddEntriesFromIterable(_map_, _iterable_, _adder_).

NOTE:
If the parameter _iterable_ is present, it is expected to be an object that implements a %Symbol.iterator% method that returns an iterator object that produces a two element array-like object whose first element is a value that will be used as a WeakMap key and whose second element is the value to associate with that key.

--- referenced: OrdinaryCreateFromConstructor ( _ctor_: a function object, _intrinsicDefaultProto_: a String, optional _internalSlotsList_: a List of names of internal slots, ): either a normal completion containing an Object or a throw completion
  id: sec-ordinarycreatefromconstructor

description: It creates an ordinary object whose [[Prototype]] value is retrieved from a constructor's *"prototype"* property, if it exists. Otherwise the intrinsic named by _intrinsicDefaultProto_ is used for [[Prototype]]. _internalSlotsList_ contains the names of additional internal slots that must be defined as part of the object. If _internalSlotsList_ is not provided, a new empty List is used.

1. Assert: _intrinsicDefaultProto_ is this specification's name of an intrinsic object. The corresponding object must be an intrinsic that is intended to be used as the [[Prototype]] value of an object.
2. Let _proto_ be ? GetPrototypeFromConstructor(_ctor_, _intrinsicDefaultProto_).
3. If _internalSlotsList_ is present, let _slots_ be _internalSlotsList_.
4. Else, let _slots_ be a new empty List.
5. Return OrdinaryObjectCreate(_proto_, _slots_).

--- referenced: Get ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-get-o-p

description: It is used to retrieve the value of a specific property of an object.

1. Return ? _obj_.[[Get]](_propertyKey_, _obj_).

--- referenced: IsCallable ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-iscallable

description: It determines if _arg_ is a callable function with a [[Call]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Call]] internal method, return *true*.
3. Return *false*.

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

- Every test262 run under built-ins/WeakMap/ passes (check with `./t262 built-ins/WeakMap/`). 48 pass on the main branch right now.

