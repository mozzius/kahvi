//! get ArrayBuffer.prototype.maxByteLength
//!
//! Spec: https://tc39.es/ecma262/#sec-get-arraybuffer.prototype.maxbytelength

use kahvi_core::prelude::*;

/// get ArrayBuffer.prototype.maxByteLength
pub(crate) fn prototype_get_max_byte_length(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `ArrayBuffer.prototype.maxByteLength` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[ArrayBufferData]]).
    // 3. If IsSharedArrayBuffer(_obj_) is *true*, throw a *TypeError* exception.
    // 4. If IsDetachedBuffer(_obj_) is *true*, return *+0*_𝔽.
    // 5. If IsFixedLengthArrayBuffer(_obj_) is *true*, then
    //    a. Let _length_ be _obj_.[[ArrayBufferByteLength]].
    // 6. Else,
    //    a. Let _length_ be _obj_.[[ArrayBufferMaxByteLength]].
    // 7. Return 𝔽(_length_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get ArrayBuffer.prototype.maxByteLength"))
}
