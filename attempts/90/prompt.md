# Ticket B-async-disposable-stack-prototype-use: AsyncDisposableStack.prototype.use

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

- `crates/kahvi-builtins/src/async_disposable_stack/prototype_use.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/disposable_stack.rs`
- `crates/kahvi-core/src/ops/dispose.rs`
- `crates/kahvi-builtins/src/disposable_stack/prototype_use.rs`

## Spec

AsyncDisposableStack.prototype.use ( _value_ )
  id: sec-asyncdisposablestack.prototype.use   type: built-in function

This method performs the following steps when called:

1. Let _asyncDisposableStack_ be the *this* value.
2. Perform ? RequireInternalSlot(_asyncDisposableStack_, [[AsyncDisposableState]]).
3. If _asyncDisposableStack_.[[AsyncDisposableState]] is ~disposed~, throw a *ReferenceError* exception.
4. Perform ? AddDisposableResource(_asyncDisposableStack_.[[DisposableResourceStack]], _value_, ~async-dispose~).
5. Return _value_.

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: AddDisposableResource ( _disposableResourceStack_: a List of DisposableResource Records, _value_: an ECMAScript language value, _kind_: ~sync-dispose~ or ~async-dispose~, optional _method_: a function object, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-adddisposableresource

1. If _method_ is present, then
   a. Assert: _value_ is *undefined*.
   b. Let _resource_ be ? CreateDisposableResource(*undefined*, _kind_, _method_).
2. Else,
   a. If _value_ is either *null* or *undefined* and _kind_ is ~sync-dispose~, return ~unused~.
   b. NOTE: When _value_ is either *null* or *undefined* and _kind_ is ~async-dispose~, we record that the resource was evaluated to ensure we will still perform an Await when resources are later disposed.
   c. Let _resource_ be ? CreateDisposableResource(_value_, _kind_).
3. Append _resource_ to _disposableResourceStack_.
4. Return ~unused~.


## Done when

- Every test262 run under built-ins/AsyncDisposableStack/prototype/use/ passes (check with `./t262 built-ins/AsyncDisposableStack/prototype/use/`). 8 pass on the main branch right now.

