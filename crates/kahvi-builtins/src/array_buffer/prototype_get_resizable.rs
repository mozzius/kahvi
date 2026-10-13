//! get ArrayBuffer.prototype.resizable
//!
//! Spec: https://tc39.es/ecma262/#sec-get-arraybuffer.prototype.resizable

use kahvi_core::prelude::*;

/// get ArrayBuffer.prototype.resizable
pub(crate) fn prototype_get_resizable(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `ArrayBuffer.prototype.resizable` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[ArrayBufferData]]).
    // 3. If IsSharedArrayBuffer(_obj_) is *true*, throw a *TypeError* exception.
    // 4. If IsFixedLengthArrayBuffer(_obj_) is *false*, return *true*.
    // 5. Return *false*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get ArrayBuffer.prototype.resizable"))
}
