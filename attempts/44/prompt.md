# Ticket B-object-seal: Object.seal

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

- `crates/kahvi-builtins/src/object/seal.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Object.seal ( _obj_ )
  id: sec-object.seal   type: built-in function

This function performs the following steps when called:

1. If _obj_ is not an Object, return _obj_.
2. Let _status_ be ? SetIntegrityLevel(_obj_, ~sealed~).
3. If _status_ is *false*, throw a *TypeError* exception.
4. Return _obj_.

--- referenced: SetIntegrityLevel ( _obj_: an Object, _level_: ~sealed~ or ~frozen~, ): either a normal completion containing a Boolean or a throw completion
  id: sec-setintegritylevel

description: It is used to fix the set of own properties of an object.

1. Let _status_ be ? _obj_.[[PreventExtensions]]().
2. If _status_ is *false*, return *false*.
3. Let _keys_ be ? _obj_.[[OwnPropertyKeys]]().
4. If _level_ is ~sealed~, then
   a. For each element _key_ of _keys_, do
      i. Perform ? DefinePropertyOrThrow(_obj_, _key_, PropertyDescriptor { [[Configurable]]: *false* }).
5. Else,
   a. Assert: _level_ is ~frozen~.
   b. For each element _key_ of _keys_, do
      i. Let _currentDesc_ be ? _obj_.[[GetOwnProperty]](_key_).
      ii. If _currentDesc_ is not *undefined*, then
         1. If IsAccessorDescriptor(_currentDesc_) is *true*, then
            a. Let _propertyDesc_ be the PropertyDescriptor { [[Configurable]]: *false* }.
         2. Else,
            a. Let _propertyDesc_ be the PropertyDescriptor { [[Configurable]]: *false*, [[Writable]]: *false* }.
         3. Perform ? DefinePropertyOrThrow(_obj_, _key_, _propertyDesc_).
6. Return *true*.


## Done when

- Every test262 run under built-ins/Object/seal/ passes (check with `./t262 built-ins/Object/seal/`). 8 pass on the main branch right now.

