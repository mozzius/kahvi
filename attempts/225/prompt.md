# Ticket B-map-prototype-get-or-insert-computed: Map.prototype.getOrInsertComputed

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

- `crates/kahvi-builtins/src/map/prototype_get_or_insert_computed.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/collections.rs`
- `crates/kahvi-builtins/src/map/prototype_set.rs`
- `crates/kahvi-builtins/src/map_iterator_prototype/next.rs`

## Spec

Map.prototype.getOrInsertComputed ( _key_, _callback_ )
  id: sec-map.prototype.getorinsertcomputed   type: built-in function

This method performs the following steps when called:

1. Let _map_ be the *this* value.
2. Perform ? RequireInternalSlot(_map_, [[MapData]]).
3. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
4. Set _key_ to CanonicalizeKeyedCollectionKey(_key_).
5. For each Record { [[Key]], [[Value]] } _entry_ of _map_.[[MapData]], do
   a. If _entry_.[[Key]] is not ~empty~ and SameValue(_entry_.[[Key]], _key_) is *true*, return _entry_.[[Value]].
6. Let _value_ be ? Call(_callback_, *undefined*, « _key_ »).
7. NOTE: The Map may have been modified during execution of _callback_.
8. For each Record { [[Key]], [[Value]] } _entry_ of _map_.[[MapData]], do
   a. If _entry_.[[Key]] is not ~empty~ and SameValue(_entry_.[[Key]], _key_) is *true*, then
      i. Set _entry_.[[Value]] to _value_.
      ii. Return _value_.
9. Let _entry_ be the Record { [[Key]]: _key_, [[Value]]: _value_ }.
10. Append _entry_ to _map_.[[MapData]].
11. Return _value_.

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

--- referenced: CanonicalizeKeyedCollectionKey ( _key_: an ECMAScript language value, ): an ECMAScript language value
  id: sec-canonicalizekeyedcollectionkey

1. If _key_ is *-0*_𝔽, return *+0*_𝔽.
2. Return _key_.

--- referenced: SameValue ( _x_: an ECMAScript language value, _y_: an ECMAScript language value, ): a Boolean
  id: sec-samevalue

description: It determines whether or not the two arguments are the same value.

1. If SameType(_x_, _y_) is *false*, return *false*.
2. If _x_ is a Number, then
   a. Return Number::sameValue(_x_, _y_).
3. Return SameValueNonNumber(_x_, _y_).

NOTE:
This algorithm differs from the IsStrictlyEqual Algorithm by treating all *NaN* values as equivalent and by differentiating *+0*_𝔽 from *-0*_𝔽.

--- referenced: Call ( _func_: an ECMAScript language value, _thisValue_: an ECMAScript language value, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-call

description: It is used to call the [[Call]] internal method of a function object. _func_ is the function object, _thisValue_ is an ECMAScript language value that is the *this* value of the [[Call]], and _argList_ is the value passed to the corresponding argument of the internal method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
3. Return ? _func_.[[Call]](_thisValue_, _argList_).


## Done when

- Every test262 run under built-ins/Map/prototype/getOrInsertComputed/ passes (check with `./t262 built-ins/Map/prototype/getOrInsertComputed/`). 4 pass on the main branch right now.

