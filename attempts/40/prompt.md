# Ticket B-object-set-prototype-of: Object.setPrototypeOf

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

- `crates/kahvi-builtins/src/object/set_prototype_of.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Object.setPrototypeOf ( _obj_, _proto_ )
  id: sec-object.setprototypeof   type: built-in function

This function performs the following steps when called:

1. Perform ? RequireObjectCoercible(_obj_).
2. If _proto_ is not an Object and _proto_ is not *null*, throw a *TypeError* exception.
3. If _obj_ is not an Object, return _obj_.
4. Let _status_ be ? _obj_.[[SetPrototypeOf]](_proto_).
5. If _status_ is *false*, throw a *TypeError* exception.
6. Return _obj_.

--- referenced: RequireObjectCoercible ( _arg_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireobjectcoercible

description: It throws an error if _arg_ is a value that cannot be converted to an Object using ToObject.

1. If _arg_ is either *undefined* or *null*, throw a *TypeError* exception.
2. Return ~unused~.


## Done when

- Every test262 run under built-ins/Object/setPrototypeOf/ passes (check with `./t262 built-ins/Object/setPrototypeOf/`). 8 pass on the main branch right now.

