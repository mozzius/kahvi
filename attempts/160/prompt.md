# Ticket B-iterator-from: Iterator.from

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

- `crates/kahvi-builtins/src/iterator/from.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/iterator_helper.rs`
- `crates/kahvi-builtins/src/iterator/prototype_map.rs`

## Spec

Iterator.from ( _obj_ )
  id: sec-iterator.from   type: built-in function

1. Let _iteratorRecord_ be ? GetIteratorFlattenable(_obj_, ~iterate-string-primitives~).
2. Let _hasInstance_ be ? OrdinaryHasInstance(%Iterator%, _iteratorRecord_.[[Iterator]]).
3. If _hasInstance_ is *true*, then
   a. Return _iteratorRecord_.[[Iterator]].
4. Let _wrapper_ be OrdinaryObjectCreate(%WrapForValidIteratorPrototype%, « [[Iterated]] »).
5. Set _wrapper_.[[Iterated]] to _iteratorRecord_.
6. Return _wrapper_.

[subclause sec-%wrapforvaliditeratorprototype%-object: The %WrapForValidIteratorPrototype% Object]

--- referenced: GetIteratorFlattenable ( _obj_: an ECMAScript language value, _primitiveHandling_: ~iterate-string-primitives~ or ~reject-primitives~, ): either a normal completion containing an Iterator Record or a throw completion
  id: sec-getiteratorflattenable

1. If _obj_ is not an Object, then
   a. If _primitiveHandling_ is ~reject-primitives~, throw a *TypeError* exception.
   b. Assert: _primitiveHandling_ is ~iterate-string-primitives~.
   c. If _obj_ is not a String, throw a *TypeError* exception.
2. Let _method_ be ? GetMethod(_obj_, %Symbol.iterator%).
3. If _method_ is *undefined*, then
   a. Let _iterator_ be _obj_.
4. Else,
   a. Let _iterator_ be ? Call(_method_, _obj_).
5. If _iterator_ is not an Object, throw a *TypeError* exception.
6. Return ? GetIteratorDirect(_iterator_).

--- referenced: OrdinaryHasInstance ( _ctor_: an ECMAScript language value, _instance_: an ECMAScript language value, ): either a normal completion containing a Boolean or a throw completion
  id: sec-ordinaryhasinstance

description: It implements the default algorithm for determining if _instance_ inherits from the instance object inheritance path provided by _ctor_.

1. If IsCallable(_ctor_) is *false*, return *false*.
2. If _ctor_ has a [[BoundTargetFunction]] internal slot, then
   a. Let _boundCtor_ be _ctor_.[[BoundTargetFunction]].
   b. Return ? InstanceofOperator(_instance_, _boundCtor_).
3. If _instance_ is not an Object, return *false*.
4. Let _proto_ be ? Get(_ctor_, *"prototype"*).
5. If _proto_ is not an Object, throw a *TypeError* exception.
6. Repeat,
   a. Set _instance_ to ? _instance_.[[GetPrototypeOf]]().
   b. If _instance_ is *null*, return *false*.
   c. If SameValue(_proto_, _instance_) is *true*, return *true*.

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


## Done when

- Every test262 run under built-ins/Iterator/from/ passes (check with `./t262 built-ins/Iterator/from/`). 12 pass on the main branch right now.

