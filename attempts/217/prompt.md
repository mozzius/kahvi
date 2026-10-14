# Ticket B-array-buffer-prototype-transfer: ArrayBuffer.prototype.transfer

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

- `crates/kahvi-builtins/src/array_buffer/prototype_transfer.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

ArrayBuffer.prototype.transfer ( [ _newLength_ ] )
  id: sec-arraybuffer.prototype.transfer   type: built-in function

This method performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Return ? ArrayBufferCopyAndDetach(_obj_, _newLength_, ~preserve-resizability~).

--- referenced: ArrayBufferCopyAndDetach ( _arrayBuffer_: an ECMAScript language value, _newLength_: an ECMAScript language value, _preserveResizability_: ~preserve-resizability~ or ~fixed-length~, ): either a normal completion containing an ArrayBuffer or a throw completion
  id: sec-arraybuffercopyanddetach

1. Perform ? RequireInternalSlot(_arrayBuffer_, [[ArrayBufferData]]).
2. If IsSharedArrayBuffer(_arrayBuffer_) is *true*, throw a *TypeError* exception.
3. If _newLength_ is *undefined*, then
   a. Let _newByteLength_ be _arrayBuffer_.[[ArrayBufferByteLength]].
4. Else,
   a. Let _newByteLength_ be ? ToIndex(_newLength_).
5. If IsDetachedBuffer(_arrayBuffer_) is *true*, throw a *TypeError* exception.
6. If _preserveResizability_ is ~preserve-resizability~ and IsFixedLengthArrayBuffer(_arrayBuffer_) is *false*, then
   a. Let _newMaxByteLength_ be _arrayBuffer_.[[ArrayBufferMaxByteLength]].
7. Else,
   a. Let _newMaxByteLength_ be ~empty~.
8. If _arrayBuffer_.[[ArrayBufferDetachKey]] is not *undefined*, throw a *TypeError* exception.
9. Let _newBuffer_ be ? AllocateArrayBuffer(%ArrayBuffer%, _newByteLength_, _newMaxByteLength_).
10. Let _copyLength_ be min(_newByteLength_, _arrayBuffer_.[[ArrayBufferByteLength]]).
11. Let _fromBlock_ be _arrayBuffer_.[[ArrayBufferData]].
12. Let _toBlock_ be _newBuffer_.[[ArrayBufferData]].
13. Perform CopyDataBlockBytes(_toBlock_, 0, _fromBlock_, 0, _copyLength_).
14. NOTE: Neither creation of the new Data Block nor copying from the old Data Block are observable. Implementations may implement this method as a zero-copy move or a `realloc`.
15. Perform ! DetachArrayBuffer(_arrayBuffer_).
16. Return _newBuffer_.


## Done when

- Every test262 run under built-ins/ArrayBuffer/prototype/transfer/ passes (check with `./t262 built-ins/ArrayBuffer/prototype/transfer/`). 10 pass on the main branch right now.

