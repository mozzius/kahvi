//! DataView ( _buffer_ [ , _byteOffset_ [ , _byteLength_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-dataview-buffer-byteoffset-bytelength

use kahvi_core::prelude::*;

/// DataView
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. Perform ? RequireInternalSlot(_buffer_, [[ArrayBufferData]]).
    // 3. Let _offset_ be ? ToIndex(_byteOffset_).
    // 4. If IsDetachedBuffer(_buffer_) is *true*, throw a *TypeError* exception.
    // 5. Let _bufferByteLength_ be ArrayBufferByteLength(_buffer_, ~seq-cst~).
    // 6. If _offset_ > _bufferByteLength_, throw a *RangeError* exception.
    // 7. Let _bufferIsFixedLength_ be IsFixedLengthArrayBuffer(_buffer_).
    // 8. If _byteLength_ is *undefined*, then
    //    a. If _bufferIsFixedLength_ is *true*, then
    //       i. Let _viewByteLength_ be _bufferByteLength_ - _offset_.
    //    b. Else,
    //       i. Let _viewByteLength_ be ~auto~.
    // 9. Else,
    //    a. Let _viewByteLength_ be ? ToIndex(_byteLength_).
    //    b. If _offset_ + _viewByteLength_ > _bufferByteLength_, throw a *RangeError* exception.
    // 10. Let _obj_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%DataView.prototype%"*, « [[DataView]], [[ViewedArrayBuffer]], [[ByteLength]], [[ByteOffset]] »).
    // 11. If IsDetachedBuffer(_buffer_) is *true*, throw a *TypeError* exception.
    // 12. Set _bufferByteLength_ to ArrayBufferByteLength(_buffer_, ~seq-cst~).
    // 13. If _offset_ > _bufferByteLength_, throw a *RangeError* exception.
    // 14. If _byteLength_ is not *undefined*, then
    //    a. If _offset_ + _viewByteLength_ > _bufferByteLength_, throw a *RangeError* exception.
    // 15. Set _obj_.[[ViewedArrayBuffer]] to _buffer_.
    // 16. Set _obj_.[[ByteLength]] to _viewByteLength_.
    // 17. Set _obj_.[[ByteOffset]] to _offset_.
    // 18. Return _obj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("DataView"))
}
