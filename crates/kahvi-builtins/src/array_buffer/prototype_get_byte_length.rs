//! get ArrayBuffer.prototype.byteLength
//!
//! Spec: https://tc39.es/ecma262/#sec-get-arraybuffer.prototype.bytelength

use kahvi_core::prelude::*;

use kahvi_core::object::typed_array::{array_buffer_byte_length, array_buffer_data};

/// get ArrayBuffer.prototype.byteLength
pub(crate) fn prototype_get_byte_length(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let O be the this value.
    // 2. Perform ? RequireInternalSlot(O, [[ArrayBufferData]]).
    // 3. If IsSharedArrayBuffer(O) is true, throw a TypeError exception.
    let o = match this {
        Value::Object(o) if array_buffer_data(agent, o).is_some() => o,
        _ => return Err(agent.throw_type_error("not an ArrayBuffer")),
    };
    // 4-6. Return 𝔽(O.[[ArrayBufferByteLength]]), which is 0 once detached.
    Ok(Value::Number(array_buffer_byte_length(agent, o) as f64))
}
