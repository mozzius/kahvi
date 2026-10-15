# Ticket B-shared-array-buffer-prototype-grow: SharedArrayBuffer.prototype.grow

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

- `crates/kahvi-builtins/src/shared_array_buffer/prototype_grow.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/typed_array.rs`
- `crates/kahvi-core/src/object/typed_array.rs`
- `crates/kahvi-builtins/src/shared_array_buffer/constructor.rs`
- `crates/kahvi-builtins/src/array_buffer/prototype_slice.rs`

## Spec

SharedArrayBuffer.prototype.grow ( _newLength_ )
  id: sec-sharedarraybuffer.prototype.grow   type: built-in function

This method performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Perform ? RequireInternalSlot(_obj_, [[ArrayBufferMaxByteLength]]).
3. If IsSharedArrayBuffer(_obj_) is *false*, throw a *TypeError* exception.
4. Let _newByteLength_ be ? ToIndex(_newLength_).
5. Let _hostHandled_ be ? HostGrowSharedArrayBuffer(_obj_, _newByteLength_).
6. If _hostHandled_ is ~handled~, return *undefined*.
7. Let _agentRecord_ be the Agent Record of the surrounding agent.
8. Let _isLittleEndian_ be _agentRecord_.[[LittleEndian]].
9. Let _byteLengthBlock_ be _obj_.[[ArrayBufferByteLengthData]].
10. Let _currentByteLengthRawBytes_ be GetRawBytesFromSharedBlock(_byteLengthBlock_, 0, ~biguint64~, *true*, ~seq-cst~).
11. Let _newByteLengthRawBytes_ be NumericToRawBytes(~biguint64~, ℤ(_newByteLength_), _isLittleEndian_).
12. Repeat,
   a. NOTE: This is a compare-and-exchange loop to ensure that parallel, racing grows of the same buffer are totally ordered, are not lost, and do not silently do nothing. The loop exits if it was able to attempt to grow uncontended.
   b. Let _currentByteLength_ be ℝ(RawBytesToNumeric(~biguint64~, _currentByteLengthRawBytes_, _isLittleEndian_)).
   c. If _newByteLength_ = _currentByteLength_, return *undefined*.
   d. If _newByteLength_ < _currentByteLength_ or _newByteLength_ > _obj_.[[ArrayBufferMaxByteLength]], throw a *RangeError* exception.
   e. Let _byteLengthDelta_ be _newByteLength_ - _currentByteLength_.
   f. If it is impossible to create a new Shared Data Block value consisting of _byteLengthDelta_ bytes, throw a *RangeError* exception.
   g. NOTE: No new Shared Data Block is constructed and used here. The observable behaviour of growable SharedArrayBuffers is specified by allocating a max-sized Shared Data Block at construction time, and this step captures the requirement that implementations that run out of memory must throw a *RangeError*.
   h. Let _readByteLengthRawBytes_ be AtomicCompareExchangeInSharedBlock(_byteLengthBlock_, 0, 8, _currentByteLengthRawBytes_, _newByteLengthRawBytes_).
   i. If ByteListEqual(_readByteLengthRawBytes_, _currentByteLengthRawBytes_) is *true*, return *undefined*.
   j. Set _currentByteLengthRawBytes_ to _readByteLengthRawBytes_.

NOTE:
Spurious failures of the compare-exchange to update the length are prohibited. If the bounds checking for the new length passes and the implementation is not out of memory, a ReadModifyWriteSharedMemory event (i.e. a successful compare-exchange) is always added into the candidate execution.
Parallel calls to SharedArrayBuffer.prototype.grow are totally ordered. For example, consider two racing calls: `sab.grow(10)` and `sab.grow(20)`. One of the two calls is guaranteed to win the race. The call to `sab.grow(10)` will never shrink `sab` even if `sab.grow(20)` happened first; in that case it will instead throw a RangeError.

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

--- referenced: GetRawBytesFromSharedBlock ( _block_: a Shared Data Block, _byteIndex_: a non-negative integer, _type_: a TypedArray element type, _isTypedArray_: a Boolean, _order_: ~seq-cst~ or ~unordered~, ): a List of byte values
  id: sec-getrawbytesfromsharedblock

1. Let _elementSize_ be the Element Size value specified in [table-the-typedarray-constructors] for Element Type _type_.
2. Let _agentRecord_ be the Agent Record of the surrounding agent.
3. Let _execution_ be _agentRecord_.[[CandidateExecution]].
4. Let _eventsRecord_ be the Agent Events Record of _execution_.[[EventsRecords]] whose [[AgentSignifier]] is AgentSignifier().
5. If _isTypedArray_ is *true* and IsNoTearConfiguration(_type_, _order_) is *true*, let _noTear_ be *true*; else let _noTear_ be *false*.
6. Let _rawValue_ be a List of length _elementSize_ whose elements are nondeterministically chosen byte values.
7. NOTE: In implementations, _rawValue_ is the result of a non-atomic or atomic read instruction on the underlying hardware. The nondeterminism is a semantic prescription of the memory model to describe observable behaviour of hardware with weak consistency.
8. Let _readEvent_ be ReadSharedMemory { [[Order]]: _order_, [[NoTear]]: _noTear_, [[Block]]: _block_, [[ByteIndex]]: _byteIndex_, [[ElementSize]]: _elementSize_ }.
9. Append _readEvent_ to _eventsRecord_.[[EventList]].
10. Append Chosen Value Record { [[Event]]: _readEvent_, [[ChosenValue]]: _rawValue_ } to _execution_.[[ChosenValues]].
11. Return _rawValue_.

--- referenced: ByteListEqual ( _xBytes_: a List of byte values, _yBytes_: a List of byte values, ): a Boolean
  id: sec-bytelistequal

1. If _xBytes_ and _yBytes_ do not have the same number of elements, return *false*.
2. Let _i_ be 0.
3. For each element _xByte_ of _xBytes_, do
   a. Let _yByte_ be _yBytes_[_i_].
   b. If _xByte_ ≠ _yByte_, return *false*.
   c. Set _i_ to _i_ + 1.
4. Return *true*.


## Done when

- Every test262 run under built-ins/SharedArrayBuffer/prototype/grow/ passes (check with `./t262 built-ins/SharedArrayBuffer/prototype/grow/`). 16 pass on the main branch right now.

