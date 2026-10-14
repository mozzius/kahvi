# Ticket B-async-disposable-stack-prototype-get-disposed: get AsyncDisposableStack.prototype.disposed

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

- `crates/kahvi-builtins/src/async_disposable_stack/prototype_get_disposed.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/disposable_stack.rs`
- `crates/kahvi-core/src/ops/dispose.rs`
- `crates/kahvi-builtins/src/disposable_stack/prototype_use.rs`

## Spec

get AsyncDisposableStack.prototype.disposed
  id: sec-get-asyncdisposablestack.prototype.disposed   type: built-in function

`AsyncDisposableStack.prototype.disposed` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:

1. Let _asyncDisposableStack_ be the *this* value.
2. Perform ? RequireInternalSlot(_asyncDisposableStack_, [[AsyncDisposableState]]).
3. If _asyncDisposableStack_.[[AsyncDisposableState]] is ~disposed~, return *true*.
4. Return *false*.

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.


## Done when

- Every test262 run under built-ins/AsyncDisposableStack/prototype/disposed/ passes (check with `./t262 built-ins/AsyncDisposableStack/prototype/disposed/`). 6 pass on the main branch right now.

