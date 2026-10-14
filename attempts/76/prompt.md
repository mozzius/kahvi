# Ticket B-iterator-zip-keyed: Iterator.zipKeyed

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

- `crates/kahvi-builtins/src/iterator/zip_keyed.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/iterator_helper.rs`
- `crates/kahvi-builtins/src/iterator/prototype_map.rs`

## Spec

Iterator.zipKeyed ( _iterables_ [ , _options_ ] )
  id: sec-iterator.zipkeyed   type: built-in function

This function performs the following steps when called:

1. If _iterables_ is not an Object, throw a *TypeError* exception.
2. Set _options_ to ? GetOptionsObject(_options_).
3. Let _mode_ be ? Get(_options_, *"mode"*).
4. If _mode_ is *undefined*, set _mode_ to *"shortest"*.
5. If _mode_ is not one of *"shortest"*, *"longest"*, or *"strict"*, throw a *TypeError* exception.
6. Let _paddingOption_ be *undefined*.
7. If _mode_ is *"longest"*, then
   a. Set _paddingOption_ to ? Get(_options_, *"padding"*).
   b. If _paddingOption_ is not *undefined* and _paddingOption_ is not an Object, throw a *TypeError* exception.
8. Let _iters_ be a new empty List.
9. Let _padding_ be a new empty List.
10. Let _allKeys_ be ? _iterables_.[[OwnPropertyKeys]]().
11. Let _keys_ be a new empty List.
12. For each element _key_ of _allKeys_, do
   a. Let _propertyDesc_ be Completion(_iterables_.[[GetOwnProperty]](_key_)).
   b. IfAbruptCloseIterators(_propertyDesc_, _iters_).
   c. If _propertyDesc_ is not *undefined* and _propertyDesc_.[[Enumerable]] is *true*, then
      i. Let _value_ be Completion(Get(_iterables_, _key_)).
      ii. IfAbruptCloseIterators(_value_, _iters_).
      iii. If _value_ is not *undefined*, then
         1. Append _key_ to _keys_.
         2. Let _iter_ be Completion(GetIteratorFlattenable(_value_, ~reject-primitives~)).
         3. IfAbruptCloseIterators(_iter_, _iters_).
         4. Append _iter_ to _iters_.
13. Let _iterCount_ be the number of elements in _iters_.
14. If _mode_ is *"longest"*, then
   a. If _paddingOption_ is *undefined*, then
      i. Repeat _iterCount_ times:
         1. Append *undefined* to _padding_.
   b. Else,
      i. For each element _key_ of _keys_, do
         1. Let _value_ be Completion(Get(_paddingOption_, _key_)).
         2. IfAbruptCloseIterators(_value_, _iters_).
         3. Append _value_ to _padding_.
15. Let _finishResults_ be a new Abstract Closure with parameters (_results_) that captures _keys_ and _iterCount_ and performs the following steps when called:
   a. Let _obj_ be OrdinaryObjectCreate(*null*).
   b. For each integer _i_ such that 0 ≤ _i_ < _iterCount_, in ascending order, do
      i. Perform ! CreateDataPropertyOrThrow(_obj_, _keys_[_i_], _results_[_i_]).
   c. Return _obj_.
16. Return IteratorZip(_iters_, _mode_, _padding_, _finishResults_).

--- referenced: GetOptionsObject ( _options_: an ECMAScript language value, ): either a normal completion containing an Object or a throw completion
  id: sec-getoptionsobject

1. If _options_ is *undefined*, then
   a. Return OrdinaryObjectCreate(*null*).
2. If _options_ is an Object, then
   a. Return _options_.
3. Throw a *TypeError* exception.

--- referenced: Get ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-get-o-p

description: It is used to retrieve the value of a specific property of an object.

1. Return ? _obj_.[[Get]](_propertyKey_, _obj_).

--- referenced: Completion ( _completionRecord_: a Completion Record, ): a Completion Record
  id: sec-completion-ao

description: It is used to emphasize that a Completion Record is being returned.
skip return checks: true

1. Assert: _completionRecord_ is a Completion Record.
2. Return _completionRecord_.

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

--- referenced: CreateDataPropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-createdatapropertyorthrow

description: It is used to create a new own property of an object. It throws a *TypeError* exception if the requested property update cannot be performed.

1. Let _success_ be ? CreateDataProperty(_obj_, _propertyKey_, _value_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.

NOTE:
This abstract operation creates a property whose attributes are set to the same defaults used for properties created by the ECMAScript language assignment operator. Normally, the property will not already exist. If it does exist and is not configurable or if _obj_ is not extensible, [[DefineOwnProperty]] will return *false* causing this operation to throw a *TypeError* exception.


## Done when

- Every test262 run under built-ins/Iterator/zipKeyed/ passes (check with `./t262 built-ins/Iterator/zipKeyed/`). 12 pass on the main branch right now.

