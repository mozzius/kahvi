//! get SharedArrayBuffer.prototype.growable
//!
//! Spec: https://tc39.es/ecma262/#sec-get-sharedarraybuffer.prototype.growable

use kahvi_core::object::typed_array::{
    array_buffer_data, is_fixed_length_array_buffer, is_shared_array_buffer,
};
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
    let obj = match this {
        Value::Object(o) if array_buffer_data(agent, o).is_some() => o,
        _ => {
            return Err(agent.throw_type_error(
                "SharedArrayBuffer.prototype.growable called on a non-ArrayBuffer",
            ));
        }
    };
    // 3. If IsSharedArrayBuffer(_obj_) is *false*, throw a *TypeError* exception.
    if !is_shared_array_buffer(agent, obj) {
        return Err(agent.throw_type_error(
            "SharedArrayBuffer.prototype.growable called on a non-SharedArrayBuffer",
        ));
    }
    // 4. If IsFixedLengthArrayBuffer(_obj_) is *false*, return *true*.
    if !is_fixed_length_array_buffer(agent, obj) {
        return Ok(Value::Boolean(true));
    }
    // 5. Return *false*.
    let _ = (args, new_target);
    Ok(Value::Boolean(false))
}
