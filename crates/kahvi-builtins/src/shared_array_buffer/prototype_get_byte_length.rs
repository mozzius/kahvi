//! get SharedArrayBuffer.prototype.byteLength
//!
//! Spec: https://tc39.es/ecma262/#sec-get-sharedarraybuffer.prototype.bytelength

use kahvi_core::object::typed_array::{
    array_buffer_byte_length, array_buffer_data, is_shared_array_buffer,
};
use kahvi_core::prelude::*;

/// get SharedArrayBuffer.prototype.byteLength
pub(crate) fn prototype_get_byte_length(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // `SharedArrayBuffer.prototype.byteLength` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[ArrayBufferData]]).
    let obj = match this {
        Value::Object(o) if array_buffer_data(agent, o).is_some() => o,
        _ => {
            return Err(agent.throw_type_error(
                "get SharedArrayBuffer.prototype.byteLength called on a non-SharedArrayBuffer",
            ));
        }
    };
    // 3. If IsSharedArrayBuffer(_obj_) is *false*, throw a *TypeError* exception.
    if !is_shared_array_buffer(agent, obj) {
        return Err(agent.throw_type_error("not a SharedArrayBuffer"));
    }
    // 4. Let _length_ be ArrayBufferByteLength(_obj_, ~seq-cst~).
    let length = array_buffer_byte_length(agent, obj);
    // 5. Return 𝔽(_length_).
    Ok(Value::Number(length as f64))
}
