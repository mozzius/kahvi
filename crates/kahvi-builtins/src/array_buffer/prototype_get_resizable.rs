//! get ArrayBuffer.prototype.resizable
//!
//! Spec: https://tc39.es/ecma262/#sec-get-arraybuffer.prototype.resizable

use kahvi_core::prelude::*;

use kahvi_core::object::typed_array::{array_buffer_data, is_fixed_length_array_buffer};

/// get ArrayBuffer.prototype.resizable
pub(crate) fn prototype_get_resizable(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[ArrayBufferData]]).
    let obj = match this {
        Value::Object(o) if array_buffer_data(agent, o).is_some() => o,
        _ => return Err(agent.throw_type_error("not an ArrayBuffer")),
    };
    // 3. If IsSharedArrayBuffer(_obj_) is *true*, throw a *TypeError* exception.
    if kahvi_core::object::typed_array::is_shared_array_buffer(agent, obj) {
        return Err(agent.throw_type_error("not an ArrayBuffer (a SharedArrayBuffer)"));
    }
    // 4. If IsFixedLengthArrayBuffer(_obj_) is *false*, return *true*.
    if !is_fixed_length_array_buffer(agent, obj) {
        return Ok(Value::Boolean(true));
    }
    // 5. Return *false*.
    Ok(Value::Boolean(false))
}
