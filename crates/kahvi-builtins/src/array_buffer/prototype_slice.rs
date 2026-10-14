//! ArrayBuffer.prototype.slice ( _start_, _end_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-arraybuffer.prototype.slice

use kahvi_core::prelude::*;

/// ArrayBuffer.prototype.slice
pub(crate) fn prototype_slice(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[ArrayBufferData]]).
    // 3. If IsSharedArrayBuffer(_obj_) is *true*, throw a *TypeError* exception.
    // 4. If IsDetachedBuffer(_obj_) is *true*, throw a *TypeError* exception.
    // 5. Let _length_ be _obj_.[[ArrayBufferByteLength]].
    // 6. Let _first_ be ? ToClampedIndex(_start_, _length_).
    // 7. If _end_ is *undefined*, let _final_ be _length_; else let _final_ be ? ToClampedIndex(_end_, _length_).
    // 8. Let _newLength_ be max(_final_ - _first_, 0).
    // 9. Let _ctor_ be ? SpeciesConstructor(_obj_, %ArrayBuffer%).
    // 10. Let _new_ be ? Construct(_ctor_, « 𝔽(_newLength_) »).
    // 11. Perform ? RequireInternalSlot(_new_, [[ArrayBufferData]]).
    // 12. If IsSharedArrayBuffer(_new_) is *true*, throw a *TypeError* exception.
    // 13. If IsDetachedBuffer(_new_) is *true*, throw a *TypeError* exception.
    // 14. If SameValue(_new_, _obj_) is *true*, throw a *TypeError* exception.
    // 15. If _new_.[[ArrayBufferByteLength]] < _newLength_, throw a *TypeError* exception.
    // 16. NOTE: Side-effects of the above steps may have detached or resized _obj_.
    // 17. If IsDetachedBuffer(_obj_) is *true*, throw a *TypeError* exception.
    // 18. Let _fromBlock_ be _obj_.[[ArrayBufferData]].
    // 19. Let _toBlock_ be _new_.[[ArrayBufferData]].
    // 20. Let _currentLength_ be _obj_.[[ArrayBufferByteLength]].
    // 21. Let _maxCount_ be _currentLength_ - _first_.
    // 22. If _maxCount_ > 0, then
    //    a. Let _count_ be min(_newLength_, _maxCount_).
    //    b. Perform CopyDataBlockBytes(_toBlock_, 0, _fromBlock_, _first_, _count_).
    // 23. Return _new_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("ArrayBuffer.prototype.slice"))
}
