//! SharedArrayBuffer.prototype.slice ( _start_, _end_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-sharedarraybuffer.prototype.slice

use kahvi_core::prelude::*;

/// SharedArrayBuffer.prototype.slice
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
    // 3. If IsSharedArrayBuffer(_obj_) is *false*, throw a *TypeError* exception.
    // 4. Let _length_ be ArrayBufferByteLength(_obj_, ~seq-cst~).
    // 5. Let _first_ be ? ToClampedIndex(_start_, _length_).
    // 6. If _end_ is *undefined*, let _final_ be _length_; else let _final_ be ? ToClampedIndex(_end_, _length_).
    // 7. Let _newLength_ be max(_final_ - _first_, 0).
    // 8. Let _ctor_ be ? SpeciesConstructor(_obj_, %SharedArrayBuffer%).
    // 9. Let _new_ be ? Construct(_ctor_, « 𝔽(_newLength_) »).
    // 10. Perform ? RequireInternalSlot(_new_, [[ArrayBufferData]]).
    // 11. If IsSharedArrayBuffer(_new_) is *false*, throw a *TypeError* exception.
    // 12. If _new_.[[ArrayBufferData]] is _obj_.[[ArrayBufferData]], throw a *TypeError* exception.
    // 13. If ArrayBufferByteLength(_new_, ~seq-cst~) < _newLength_, throw a *TypeError* exception.
    // 14. Let _fromBlock_ be _obj_.[[ArrayBufferData]].
    // 15. Let _toBlock_ be _new_.[[ArrayBufferData]].
    // 16. Perform CopyDataBlockBytes(_toBlock_, 0, _fromBlock_, _first_, _newLength_).
    // 17. Return _new_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("SharedArrayBuffer.prototype.slice"))
}
