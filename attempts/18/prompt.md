# Ticket B-object-is-extensible: Object.isExtensible

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

- `crates/kahvi-builtins/src/object/is_extensible.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Object.isExtensible ( _obj_ )
  id: sec-object.isextensible   type: built-in function

This function performs the following steps when called:

1. If _obj_ is not an Object, return *false*.
2. Return ? IsExtensible(_obj_).

--- referenced: IsExtensible ( _obj_: an Object, ): either a normal completion containing a Boolean or a throw completion
  id: sec-isextensible-o

description: It is used to determine whether additional properties can be added to _obj_.

1. Return ? _obj_.[[IsExtensible]]().


## Done when

- Every test262 run under built-ins/Object/isExtensible/ passes (check with `./t262 built-ins/Object/isExtensible/`). 8 pass on the main branch right now.

