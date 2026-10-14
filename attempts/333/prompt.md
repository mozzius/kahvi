# Ticket B-array-buffer-prototype-get-detached: get ArrayBuffer.prototype.detached

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

- `crates/kahvi-builtins/src/array_buffer/prototype_get_detached.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

get ArrayBuffer.prototype.detached
  id: sec-get-arraybuffer.prototype.detached   type: built-in function

`ArrayBuffer.prototype.detached` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Perform ? RequireInternalSlot(_obj_, [[ArrayBufferData]]).
3. If IsSharedArrayBuffer(_obj_) is *true*, throw a *TypeError* exception.
4. Return IsDetachedBuffer(_obj_).

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: IsSharedArrayBuffer ( _obj_: an ArrayBuffer or a SharedArrayBuffer, ): a Boolean
  id: sec-issharedarraybuffer

description: It tests whether an object is a SharedArrayBuffer.

1. If _obj_.[[ArrayBufferData]] is a Shared Data Block, return *true*.
2. Return *false*.

--- referenced: IsDetachedBuffer ( _arrayBuffer_: an ArrayBuffer or a SharedArrayBuffer, ): a Boolean
  id: sec-isdetachedbuffer

1. If _arrayBuffer_.[[ArrayBufferData]] is *null*, return *true*.
2. Return *false*.


## Done when

- Every test262 run under built-ins/ArrayBuffer/prototype/detached/ passes (check with `./t262 built-ins/ArrayBuffer/prototype/detached/`). 6 pass on the main branch right now.

