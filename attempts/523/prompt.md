# Ticket B-atomics-pause: Atomics.pause

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

- `crates/kahvi-builtins/src/atomics/pause.rs`

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

Atomics.pause ( )
  id: sec-atomics.pause   type: built-in function

This function provides a hint to the CPU that the program is spin looping while waiting on a value.
It performs the following steps when called:

1. If the execution environment of the ECMAScript implementation supports signaling to the operating system or CPU that the current executing code is in a spin-wait loop, send that signal.
2. Return *undefined*.

NOTE:
This method is designed for programs implementing spin-wait loops, such as spinlock fast paths inside of mutexes, to provide a hint to the CPU that it is spinning while waiting on a value. It has no observable behaviour other than timing.
Implementations are expected to implement a pause or yield instruction if the best practices of the underlying architecture recommends such instructions in spin loops. For example, the Intel Optimization Manual recommends the pause instruction.
Implementations are encouraged to have an internal upper bound on the maximum amount of time paused on the order of tens to hundreds of nanoseconds.
NOTE:
Due to the overhead of function calls, it is reasonable that an inlined call to this method in an optimizing compiler waits a different amount of time than a non-inlined call.


## Done when

- Every test262 run under built-ins/Atomics/pause/ passes (check with `./t262 built-ins/Atomics/pause/`). 8 pass on the main branch right now.

