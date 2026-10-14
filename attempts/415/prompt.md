# Ticket B-number-is-nan: Number.isNaN

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

- `crates/kahvi-builtins/src/number/is_nan.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Number.isNaN ( _number_ )
  id: sec-number.isnan   type: built-in function

This function performs the following steps when called:

1. If _number_ is not a Number, return *false*.
2. If _number_ is *NaN*, return *true*.
3. Return *false*.

NOTE:
This function differs from the global isNaN function ([sec-isnan-number]) in that it does not convert its argument to a Number before determining whether it is *NaN*.


## Done when

- Every test262 run under built-ins/Number/isNaN/ passes (check with `./t262 built-ins/Number/isNaN/`). 8 pass on the main branch right now.

