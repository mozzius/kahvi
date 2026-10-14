//! Atomics.compareExchange ( _ta_, _index_, _expectedValue_, _replacementValue_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.compareexchange

use kahvi_core::prelude::*;

/// Atomics.compareExchange
pub(crate) fn compare_exchange(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _byteIndexInBuffer_ be ? ValidateAtomicAccessOnIntegerTypedArray(_ta_, _index_).
    // 2. Let _buffer_ be _ta_.[[ViewedArrayBuffer]].
    // 3. Let _block_ be _buffer_.[[ArrayBufferData]].
    // 4. If _ta_.[[ContentType]] is ~bigint~, then
    //    a. Let _expected_ be ? ToBigInt(_expectedValue_).
    //    b. Let _replacement_ be ? ToBigInt(_replacementValue_).
    // 5. Else,
    //    a. Let _expected_ be 𝔽(? ToIntegerOrInfinity(_expectedValue_)).
    //    b. Let _replacement_ be 𝔽(? ToIntegerOrInfinity(_replacementValue_)).
    // 6. Perform ? RevalidateAtomicAccess(_ta_, _byteIndexInBuffer_).
    // 7. Let _elementType_ be TypedArrayElementType(_ta_).
    // 8. Let _elementSize_ be TypedArrayElementSize(_ta_).
    // 9. Let _agentRecord_ be the Agent Record of the surrounding agent.
    // 10. Let _isLittleEndian_ be _agentRecord_.[[LittleEndian]].
    // 11. Let _expectedBytes_ be NumericToRawBytes(_elementType_, _expected_, _isLittleEndian_).
    // 12. Let _replacementBytes_ be NumericToRawBytes(_elementType_, _replacement_, _isLittleEndian_).
    // 13. If IsSharedArrayBuffer(_buffer_) is *true*, then
    //    a. Let _rawBytesRead_ be AtomicCompareExchangeInSharedBlock(_block_, _byteIndexInBuffer_, _elementSize_, _expectedBytes_, _replacementBytes_).
    // 14. Else,
    //    a. Let _rawBytesRead_ be a List of length _elementSize_ whose elements are the sequence of _elementSize_ bytes starting with _block_[_byteIndexInBuffer_].
    //    b. If ByteListEqual(_rawBytesRead_, _expectedBytes_) is *true*, then
    //       i. Store the individual bytes of _replacementBytes_ into _block_, starting at _block_[_byteIndexInBuffer_].
    // 15. Return RawBytesToNumeric(_elementType_, _rawBytesRead_, _isLittleEndian_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Atomics.compareExchange"))
}
