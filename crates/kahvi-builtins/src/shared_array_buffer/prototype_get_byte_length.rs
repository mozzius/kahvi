//! get SharedArrayBuffer.prototype.byteLength
//!
//! Spec: https://tc39.es/ecma262/#sec-get-sharedarraybuffer.prototype.bytelength

use kahvi_core::prelude::*;

/// get SharedArrayBuffer.prototype.byteLength
pub(crate) fn prototype_get_byte_length(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `SharedArrayBuffer.prototype.byteLength` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[ArrayBufferData]]).
    // 3. If IsSharedArrayBuffer(_obj_) is *false*, throw a *TypeError* exception.
    // 4. Let _length_ be ArrayBufferByteLength(_obj_, ~seq-cst~).
    // 5. Return 𝔽(_length_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get SharedArrayBuffer.prototype.byteLength"))
}
