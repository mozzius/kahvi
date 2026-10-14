# Ticket B-set-constructor: Set

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

- `crates/kahvi-builtins/src/set/constructor.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/collections.rs`
- `crates/kahvi-builtins/src/map/prototype_set.rs`
- `crates/kahvi-builtins/src/map_iterator_prototype/next.rs`

## Spec

Set ( [ _iterable_ ] )
  id: sec-set-iterable   type: built-in function

This function performs the following steps when called:

1. If NewTarget is *undefined*, throw a *TypeError* exception.
2. Let _set_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Set.prototype%"*, « [[SetData]] »).
3. Set _set_.[[SetData]] to a new empty List.
4. If _iterable_ is either *undefined* or *null*, return _set_.
5. Let _adder_ be ? Get(_set_, *"add"*).
6. If IsCallable(_adder_) is *false*, throw a *TypeError* exception.
7. Let _iteratorRecord_ be ? GetIterator(_iterable_, ~sync~).
8. Repeat,
   a. Let _next_ be ? IteratorStepValue(_iteratorRecord_).
   b. If _next_ is ~done~, return _set_.
   c. Let _status_ be Completion(Call(_adder_, _set_, « _next_ »)).
   d. IfAbruptCloseIterator(_status_, _iteratorRecord_).

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

--- referenced: GetIterator ( _obj_: an ECMAScript language value, _kind_: ~sync~ or ~async~, ): either a normal completion containing an Iterator Record or a throw completion
  id: sec-getiterator

1. If _kind_ is ~async~, then
   a. Let _method_ be ? GetMethod(_obj_, %Symbol.asyncIterator%).
   b. If _method_ is *undefined*, then
      i. Let _syncMethod_ be ? GetMethod(_obj_, %Symbol.iterator%).
      ii. If _syncMethod_ is *undefined*, throw a *TypeError* exception.
      iii. Let _syncIteratorRecord_ be ? GetIteratorFromMethod(_obj_, _syncMethod_).
      iv. Return CreateAsyncFromSyncIterator(_syncIteratorRecord_).
2. Else,
   a. Let _method_ be ? GetMethod(_obj_, %Symbol.iterator%).
3. If _method_ is *undefined*, throw a *TypeError* exception.
4. Return ? GetIteratorFromMethod(_obj_, _method_).

--- referenced: IteratorStepValue ( _iteratorRecord_: an Iterator Record, ): either a normal completion containing either an ECMAScript language value or ~done~, or a throw completion
  id: sec-iteratorstepvalue

description: It requests the next value from _iteratorRecord_.[[Iterator]] by calling _iteratorRecord_.[[NextMethod]] and returns either ~done~ indicating that the iterator has reached its end or the value from the IteratorResult object if a next value is available.

1. Let _result_ be ? IteratorStep(_iteratorRecord_).
2. If _result_ is ~done~, then
   a. Return ~done~.
3. Let _value_ be Completion(IteratorValue(_result_)).
4. If _value_ is a throw completion, then
   a. Set _iteratorRecord_.[[Done]] to *true*.
5. Return ? _value_.

--- referenced: Completion ( _completionRecord_: a Completion Record, ): a Completion Record
  id: sec-completion-ao

description: It is used to emphasize that a Completion Record is being returned.
skip return checks: true

1. Assert: _completionRecord_ is a Completion Record.
2. Return _completionRecord_.

--- referenced: Call ( _func_: an ECMAScript language value, _thisValue_: an ECMAScript language value, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-call

description: It is used to call the [[Call]] internal method of a function object. _func_ is the function object, _thisValue_ is an ECMAScript language value that is the *this* value of the [[Call]], and _argList_ is the value passed to the corresponding argument of the internal method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
3. Return ? _func_.[[Call]](_thisValue_, _argList_).


## Done when

- Every test262 run under built-ins/Set/ passes (check with `./t262 built-ins/Set/`). 280 pass on the main branch right now.

## Previous attempts

- attempt 13: worker reported BLOCKED but changed files: Set.prototype.add (and the other Set.prototype methods, plus `WeakSet` and `get Set.prototype.size`) are not implemented, so the constructor's adder call and most Set tests cannot pass yet.
- attempt 13: merged partial progress. Still failing:
built-ins/Set/bigint-number-same-value.js:sloppy  Uncaught InternalError: not implemented: Set.prototype.add
built-ins/Set/bigint-number-same-value.js:strict  Uncaught InternalError: not implemented: Set.prototype.add
built-ins/Set/proto-from-ctor-realm.js:sloppy  Uncaught InternalError: not implemented: Function
built-ins/Set/proto-from-ctor-realm.js:strict  Uncaught InternalError: not implemented: Function
built-ins/Set/prototype/add/does-not-have-setdata-internal-slot-array.js:sloppy  Uncaught InternalError: not implemented: Set.prototype.add
built-ins/Set/prototype/add/does-not-have-setdata-internal-slot-array.js:strict  Uncaught InternalError: not implemented: Set.prototype.add
built-ins/Set/prototype/add/does-not-have-setdata-internal-slot-map.js:sloppy  Uncaught InternalError: not implemented: Set.prototype.add
built-ins/Set/prototype/add/does-not-have-setdata-internal-slot-map.js:strict  Uncaught InternalError: not implemented: Set.prototype.add
built-ins/Set/prototype/add/does-not-have-setdata-internal-slot-object.js:sloppy  Uncaught InternalError: not implemented: Set.prototype.add
built-ins/Set/prototype/add/does-not-have-setdata-internal-slot-object.js:strict  Uncaught InternalError: not implemented: Set.prototype.add
- attempt 14: BLOCKED: `Set.prototype.add` (src/set/prototype_add.rs), which every constructor test with an iterable depends on, plus the other Set.prototype methods, `get Set.prototype.size`, `WeakSet`, and the `Function` constructor needed by `proto-from-ctor-realm.js`.
- attempt None: architect: Set.prototype.add is implemented now (merged); retry. Remaining failures in other Set methods are not this ticket.

