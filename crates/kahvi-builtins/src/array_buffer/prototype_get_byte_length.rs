//! get ArrayBuffer.prototype.byteLength
//!
//! Spec: https://tc39.es/ecma262/#sec-get-arraybuffer.prototype.bytelength

use kahvi_core::prelude::*;

/// get ArrayBuffer.prototype.byteLength
pub(crate) fn prototype_get_byte_length(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `ArrayBuffer.prototype.byteLength` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[ArrayBufferData]]).
    // 3. If IsSharedArrayBuffer(_obj_) is *true*, throw a *TypeError* exception.
    // 4. If IsDetachedBuffer(_obj_) is *true*, return *+0*_𝔽.
    // 5. Let _length_ be _obj_.[[ArrayBufferByteLength]].
    // 6. Return 𝔽(_length_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get ArrayBuffer.prototype.byteLength"))
}
