# Ticket B-iterator-prototype-get-constructor: get Iterator.prototype.constructor

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

- `crates/kahvi-builtins/src/iterator/prototype_get_constructor.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/iterator_helper.rs`
- `crates/kahvi-builtins/src/iterator/prototype_map.rs`

## Spec

get Iterator.prototype.constructor
  id: sec-get-iterator.prototype.constructor   type: built-in function

The value of the [[Getter]] attribute is a built-in function that requires no arguments. It performs the following steps when called:

1. Return %Iterator%.


## Done when

- Every test262 run under built-ins/Iterator/prototype/constructor/ passes (check with `./t262 built-ins/Iterator/prototype/constructor/`). 0 pass on the main branch right now.

