//! ArrayBuffer.prototype.resize ( _newLength_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-arraybuffer.prototype.resize

use kahvi_core::prelude::*;

/// ArrayBuffer.prototype.resize
pub(crate) fn prototype_resize(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[ArrayBufferMaxByteLength]]).
    // 3. If IsSharedArrayBuffer(_obj_) is *true*, throw a *TypeError* exception.
    // 4. Let _newByteLength_ be ? ToIndex(_newLength_).
    // 5. If IsDetachedBuffer(_obj_) is *true*, throw a *TypeError* exception.
    // 6. If _newByteLength_ > _obj_.[[ArrayBufferMaxByteLength]], throw a *RangeError* exception.
    // 7. Let _hostHandled_ be ? HostResizeArrayBuffer(_obj_, _newByteLength_).
    // 8. If _hostHandled_ is ~handled~, return *undefined*.
    // 9. Let _oldBlock_ be _obj_.[[ArrayBufferData]].
    // 10. Let _newBlock_ be ? CreateByteDataBlock(_newByteLength_).
    // 11. Let _copyLength_ be min(_newByteLength_, _obj_.[[ArrayBufferByteLength]]).
    // 12. Perform CopyDataBlockBytes(_newBlock_, 0, _oldBlock_, 0, _copyLength_).
    // 13. NOTE: Neither creation of the new Data Block nor copying from the old Data Block are observable. Implementations may implement this method as in-place growth or shrinkage.
    // 14. Set _obj_.[[ArrayBufferData]] to _newBlock_.
    // 15. Set _obj_.[[ArrayBufferByteLength]] to _newByteLength_.
    // 16. Return *undefined*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("ArrayBuffer.prototype.resize"))
}
