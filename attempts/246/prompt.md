# Ticket B-set-prototype-delete: Set.prototype.delete

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

- `crates/kahvi-builtins/src/set/prototype_delete.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/collections.rs`
- `crates/kahvi-builtins/src/map/prototype_set.rs`
- `crates/kahvi-builtins/src/map_iterator_prototype/next.rs`

## Spec

Set.prototype.delete ( _value_ )
  id: sec-set.prototype.delete   type: built-in function

This method performs the following steps when called:

1. Let _set_ be the *this* value.
2. Perform ? RequireInternalSlot(_set_, [[SetData]]).
3. Set _value_ to CanonicalizeKeyedCollectionKey(_value_).
4. For each element _entry_ of _set_.[[SetData]], do
   a. If _entry_ is not ~empty~ and SameValue(_entry_, _value_) is *true*, then
      i. Replace the element of _set_.[[SetData]] whose value is _entry_ with an element whose value is ~empty~.
      ii. Return *true*.
5. Return *false*.

NOTE:
The value ~empty~ is used as a specification device to indicate that an entry has been deleted. Actual implementations may take other actions such as physically removing the entry from internal data structures.

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

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


## Done when

- Every test262 run under built-ins/Set/prototype/delete/ passes (check with `./t262 built-ins/Set/prototype/delete/`). 8 pass on the main branch right now.

