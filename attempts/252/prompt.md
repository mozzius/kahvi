# Ticket B-map-prototype-for-each: Map.prototype.forEach

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

- `crates/kahvi-builtins/src/map/prototype_for_each.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/collections.rs`
- `crates/kahvi-builtins/src/map/prototype_set.rs`
- `crates/kahvi-builtins/src/map_iterator_prototype/next.rs`

## Spec

Map.prototype.forEach ( _callback_ [ , _thisArg_ ] )
  id: sec-map.prototype.foreach   type: built-in function

This method performs the following steps when called:

1. Let _map_ be the *this* value.
2. Perform ? RequireInternalSlot(_map_, [[MapData]]).
3. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
4. Let _entries_ be _map_.[[MapData]].
5. Let _entriesCount_ be the number of elements in _entries_.
6. Let _index_ be 0.
7. Repeat, while _index_ < _entriesCount_,
   a. Let _entry_ be _entries_[_index_].
   b. Set _index_ to _index_ + 1.
   c. If _entry_.[[Key]] is not ~empty~, then
      i. Perform ? Call(_callback_, _thisArg_, « _entry_.[[Value]], _entry_.[[Key]], _map_ »).
      ii. NOTE: The number of elements in _entries_ may have increased during execution of _callback_.
      iii. Set _entriesCount_ to the number of elements in _entries_.
8. Return *undefined*.

NOTE:
_callback_ should be a function that accepts three arguments. `forEach` calls _callback_ once for each key/value pair present in the Map, in key insertion order. _callback_ is called only for keys of the Map which actually exist; it is not called for keys that have been deleted from the Map.
If a _thisArg_ parameter is provided, it will be used as the *this* value for each invocation of _callback_. If it is not provided, *undefined* is used instead.
_callback_ is called with three arguments: the value of the item, the key of the item, and the Map being traversed.
`forEach` does not directly mutate the object on which it is called but the object may be mutated by the calls to _callback_. Each entry of a map's [[MapData]] is only visited once. New keys added after the call to `forEach` begins are visited. A key will be revisited if it is deleted after it has been visited and then re-added before the `forEach` call completes. Keys that are deleted after the call to `forEach` begins and before being visited are not visited unless the key is added again before the `forEach` call completes.

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: IsCallable ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-iscallable

description: It determines if _arg_ is a callable function with a [[Call]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Call]] internal method, return *true*.
3. Return *false*.

--- referenced: Call ( _func_: an ECMAScript language value, _thisValue_: an ECMAScript language value, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-call

description: It is used to call the [[Call]] internal method of a function object. _func_ is the function object, _thisValue_ is an ECMAScript language value that is the *this* value of the [[Call]], and _argList_ is the value passed to the corresponding argument of the internal method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
3. Return ? _func_.[[Call]](_thisValue_, _argList_).


## Done when

- Every test262 run under built-ins/Map/prototype/forEach/ passes (check with `./t262 built-ins/Map/prototype/forEach/`). 8 pass on the main branch right now.

