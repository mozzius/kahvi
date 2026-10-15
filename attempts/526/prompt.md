# Ticket B-atomics-wait: Atomics.wait

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

Kahvi's agent cannot suspend: there is no other agent that could ever wake
it, and the host's [[CanBlock]] is false. So AgentCanSuspend() is false and
DoWait with mode ~sync~ throws the TypeError at that step (after the earlier
validation and coercion steps, in spec order). Tests that need blocking are
flagged CanBlockIsTrue and are not run. Write DoWait in this file.

## Files you may change

- `crates/kahvi-builtins/src/atomics/wait.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/atomics.rs`
- `crates/kahvi-builtins/src/atomics/add.rs`
- `crates/kahvi-builtins/src/atomics/and.rs`
- `crates/kahvi-core/src/object/typed_array.rs`

## Spec

Atomics.wait ( _ta_, _index_, _value_, _timeout_ )
  id: sec-atomics.wait   type: built-in function

This function puts the surrounding agent in a wait queue and suspends it until notified or until the wait times out, returning a String differentiating those cases.
It performs the following steps when called:

1. Return ? DoWait(~sync~, _ta_, _index_, _value_, _timeout_).


## Done when

- Every test262 run under built-ins/Atomics/wait/ passes (check with `./t262 built-ins/Atomics/wait/`). 6 pass on the main branch right now.

