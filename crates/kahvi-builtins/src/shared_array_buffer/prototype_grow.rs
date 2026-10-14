//! SharedArrayBuffer.prototype.grow ( _newLength_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-sharedarraybuffer.prototype.grow

use kahvi_core::prelude::*;

/// SharedArrayBuffer.prototype.grow
pub(crate) fn prototype_grow(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[ArrayBufferMaxByteLength]]).
    // 3. If IsSharedArrayBuffer(_obj_) is *false*, throw a *TypeError* exception.
    // 4. Let _newByteLength_ be ? ToIndex(_newLength_).
    // 5. Let _hostHandled_ be ? HostGrowSharedArrayBuffer(_obj_, _newByteLength_).
    // 6. If _hostHandled_ is ~handled~, return *undefined*.
    // 7. Let _agentRecord_ be the Agent Record of the surrounding agent.
    // 8. Let _isLittleEndian_ be _agentRecord_.[[LittleEndian]].
    // 9. Let _byteLengthBlock_ be _obj_.[[ArrayBufferByteLengthData]].
    // 10. Let _currentByteLengthRawBytes_ be GetRawBytesFromSharedBlock(_byteLengthBlock_, 0, ~biguint64~, *true*, ~seq-cst~).
    // 11. Let _newByteLengthRawBytes_ be NumericToRawBytes(~biguint64~, ℤ(_newByteLength_), _isLittleEndian_).
    // 12. Repeat,
    //    a. NOTE: This is a compare-and-exchange loop to ensure that parallel, racing grows of the same buffer are totally ordered, are not lost, and do not silently do nothing. The loop exits if it was able to attempt to grow uncontended.
    //    b. Let _currentByteLength_ be ℝ(RawBytesToNumeric(~biguint64~, _currentByteLengthRawBytes_, _isLittleEndian_)).
    //    c. If _newByteLength_ = _currentByteLength_, return *undefined*.
    //    d. If _newByteLength_ < _currentByteLength_ or _newByteLength_ > _obj_.[[ArrayBufferMaxByteLength]], throw a *RangeError* exception.
    //    e. Let _byteLengthDelta_ be _newByteLength_ - _currentByteLength_.
    //    f. If it is impossible to create a new Shared Data Block value consisting of _byteLengthDelta_ bytes, throw a *RangeError* exception.
    //    g. NOTE: No new Shared Data Block is constructed and used here. The observable behaviour of growable SharedArrayBuffers is specified by allocating a max-sized Shared Data Block at construction time, and this step captures the requirement that implementations that run out of memory must throw a *RangeError*.
    //    h. Let _readByteLengthRawBytes_ be AtomicCompareExchangeInSharedBlock(_byteLengthBlock_, 0, 8, _currentByteLengthRawBytes_, _newByteLengthRawBytes_).
    //    i. If ByteListEqual(_readByteLengthRawBytes_, _currentByteLengthRawBytes_) is *true*, return *undefined*.
    //    j. Set _currentByteLengthRawBytes_ to _readByteLengthRawBytes_.
    //
    // NOTE:
    // Spurious failures of the compare-exchange to update the length are prohibited. If the bounds checking for the new length passes and the implementation is not out of memory, a ReadModifyWriteSharedMemory event (i.e. a successful compare-exchange) is always added into the candidate execution.
    // Parallel calls to SharedArrayBuffer.prototype.grow are totally ordered. For example, consider two racing calls: `sab.grow(10)` and `sab.grow(20)`. One of the two calls is guaranteed to win the race. The call to `sab.grow(10)` will never shrink `sab` even if `sab.grow(20)` happened first; in that case it will instead throw a RangeError.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("SharedArrayBuffer.prototype.grow"))
}
