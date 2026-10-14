# Ticket B-json-is-raw-json: JSON.isRawJSON

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

- `crates/kahvi-builtins/src/json/is_raw_json.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

JSON.isRawJSON ( _obj_ )
  id: sec-json.israwjson   type: built-in function

This function performs the following steps when called:

1. If _obj_ is an Object and _obj_ has an [[IsRawJSON]] internal slot, return *true*.
2. Return *false*.


## Done when

- Every test262 run under built-ins/JSON/isRawJSON/ passes (check with `./t262 built-ins/JSON/isRawJSON/`). 10 pass on the main branch right now.

## Previous attempts

- attempt 449: worker reported BLOCKED but changed files: `parse_json` in `crates/kahvi-builtins/src/json/parse.rs` must be made `pub(crate)` so `JSON.rawJSON` can call ParseJSON; this is needed for the two remaining `basic.js` runs.
- attempt 449: rejected by gates: no new test262 passes in target

