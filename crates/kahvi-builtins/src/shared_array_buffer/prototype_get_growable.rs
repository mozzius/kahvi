//! get SharedArrayBuffer.prototype.growable
//!
//! Spec: https://tc39.es/ecma262/#sec-get-sharedarraybuffer.prototype.growable

use kahvi_core::prelude::*;

/// get SharedArrayBuffer.prototype.growable
pub(crate) fn prototype_get_growable(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `SharedArrayBuffer.prototype.growable` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[ArrayBufferData]]).
    // 3. If IsSharedArrayBuffer(_obj_) is *false*, throw a *TypeError* exception.
    // 4. If IsFixedLengthArrayBuffer(_obj_) is *false*, return *true*.
    // 5. Return *false*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get SharedArrayBuffer.prototype.growable"))
}
