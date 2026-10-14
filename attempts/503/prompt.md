# Ticket B-typed-array-prototype-symbol-iterator: TypedArray.prototype[Symbol.iterator]

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

- `crates/kahvi-builtins/src/typed_array/prototype_symbol_iterator.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

%TypedArray%.prototype [ %Symbol.iterator% ] ( )
  id: sec-%typedarray%.prototype-%symbol.iterator%   type: built-in function

The initial value of the %Symbol.iterator% property is %TypedArray.prototype.values%, defined in [sec-%typedarray%.prototype.values].


## Done when

- Every test262 run under built-ins/TypedArray/prototype/Symbol.iterator/ passes (check with `./t262 built-ins/TypedArray/prototype/Symbol.iterator/`). 2 pass on the main branch right now.

## Previous attempts

- attempt 501: rejected by gates: no new test262 passes in target
- attempt 501: escalated from haiku to sonnet
- attempt 502: worker reported BLOCKED but changed files: same-function-object aliasing of `%TypedArray.prototype%[@@iterator]` to `values` needs an alias mechanism in `table.rs` and `generated.rs` (from `harness/gen/builtins.py`).
- attempt 502: rejected by gates: no new test262 passes in target

