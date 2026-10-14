# Ticket B-math-random: Math.random

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

- `crates/kahvi-builtins/src/math/random.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Math.random ( )
  id: sec-math.random   type: built-in function

This function returns a Number value with positive sign, greater than or equal to *+0*_𝔽 but strictly less than *1*_𝔽, chosen randomly or pseudo randomly with approximately uniform distribution over that range, using an implementation-defined algorithm or strategy.
Each `Math.random` function created for distinct realms must produce a distinct sequence of values from successive calls.


## Done when

- Every test262 run under built-ins/Math/random/ passes (check with `./t262 built-ins/Math/random/`). 8 pass on the main branch right now.

