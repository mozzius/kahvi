# Ticket B-array-buffer-prototype-resize: ArrayBuffer.prototype.resize

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

- `crates/kahvi-builtins/src/array_buffer/prototype_resize.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/typed_array/prototype_get_length.rs`

## Spec

ArrayBuffer.prototype.resize ( _newLength_ )
  id: sec-arraybuffer.prototype.resize   type: built-in function

This method performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Perform ? RequireInternalSlot(_obj_, [[ArrayBufferMaxByteLength]]).
3. If IsSharedArrayBuffer(_obj_) is *true*, throw a *TypeError* exception.
4. Let _newByteLength_ be ? ToIndex(_newLength_).
5. If IsDetachedBuffer(_obj_) is *true*, throw a *TypeError* exception.
6. If _newByteLength_ > _obj_.[[ArrayBufferMaxByteLength]], throw a *RangeError* exception.
7. Let _hostHandled_ be ? HostResizeArrayBuffer(_obj_, _newByteLength_).
8. If _hostHandled_ is ~handled~, return *undefined*.
9. Let _oldBlock_ be _obj_.[[ArrayBufferData]].
10. Let _newBlock_ be ? CreateByteDataBlock(_newByteLength_).
11. Let _copyLength_ be min(_newByteLength_, _obj_.[[ArrayBufferByteLength]]).
12. Perform CopyDataBlockBytes(_newBlock_, 0, _oldBlock_, 0, _copyLength_).
13. NOTE: Neither creation of the new Data Block nor copying from the old Data Block are observable. Implementations may implement this method as in-place growth or shrinkage.
14. Set _obj_.[[ArrayBufferData]] to _newBlock_.
15. Set _obj_.[[ArrayBufferByteLength]] to _newByteLength_.
16. Return *undefined*.

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

--- referenced: ToIndex ( _arg_: an ECMAScript language value, ): either a normal completion containing a non-negative integer or a throw completion
  id: sec-toindex

description: It converts _arg_ to an integer and returns that integer if it is non-negative and corresponds with an integer index. Otherwise, it throws an exception.

1. Let _int_ be ? ToIntegerOrInfinity(_arg_).
2. If _int_ is not in the inclusive interval from 0 to 2^53 - 1, throw a *RangeError* exception.
3. Return _int_.

--- referenced: IsDetachedBuffer ( _arrayBuffer_: an ArrayBuffer or a SharedArrayBuffer, ): a Boolean
  id: sec-isdetachedbuffer

1. If _arrayBuffer_.[[ArrayBufferData]] is *null*, return *true*.
2. Return *false*.

--- referenced: CreateByteDataBlock ( _size_: a non-negative integer, ): either a normal completion containing a Data Block or a throw completion
  id: sec-createbytedatablock

1. If _size_ > 2^53 - 1, throw a *RangeError* exception.
2. Let _dataBlock_ be a new Data Block value consisting of _size_ bytes. If it is impossible to create such a Data Block, throw a *RangeError* exception.
3. Set all of the bytes of _dataBlock_ to 0.
4. Return _dataBlock_.


## Done when

- Every test262 run under built-ins/ArrayBuffer/prototype/resize/ passes (check with `./t262 built-ins/ArrayBuffer/prototype/resize/`). 10 pass on the main branch right now.

