# Ticket B-object-is-frozen: Object.isFrozen

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

- `crates/kahvi-builtins/src/object/is_frozen.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Object.isFrozen ( _obj_ )
  id: sec-object.isfrozen   type: built-in function

This function performs the following steps when called:

1. If _obj_ is not an Object, return *true*.
2. Return ? TestIntegrityLevel(_obj_, ~frozen~).

--- referenced: TestIntegrityLevel ( _obj_: an Object, _level_: ~sealed~ or ~frozen~, ): either a normal completion containing a Boolean or a throw completion
  id: sec-testintegritylevel

description: It is used to determine if the set of own properties of an object are fixed.

1. Let _extensible_ be ? IsExtensible(_obj_).
2. If _extensible_ is *true*, return *false*.
3. NOTE: If the object is extensible, none of its properties are examined.
4. Let _keys_ be ? _obj_.[[OwnPropertyKeys]]().
5. For each element _key_ of _keys_, do
   a. Let _currentDesc_ be ? _obj_.[[GetOwnProperty]](_key_).
   b. If _currentDesc_ is not *undefined*, then
      i. If _currentDesc_.[[Configurable]] is *true*, return *false*.
      ii. If _level_ is ~frozen~ and IsDataDescriptor(_currentDesc_) is *true*, then
         1. If _currentDesc_.[[Writable]] is *true*, return *false*.
6. Return *true*.


## Done when

- Every test262 run under built-ins/Object/isFrozen/ passes (check with `./t262 built-ins/Object/isFrozen/`). 8 pass on the main branch right now.

