//! get ArrayBuffer.prototype.maxByteLength
//!
//! Spec: https://tc39.es/ecma262/#sec-get-arraybuffer.prototype.maxbytelength

use kahvi_core::prelude::*;

use kahvi_core::object::typed_array::{
    array_buffer_byte_length, array_buffer_data, is_detached_buffer,
};

/// get ArrayBuffer.prototype.maxByteLength
pub(crate) fn prototype_get_max_byte_length(
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
    // IsSharedArrayBuffer is always *false* here: the only [[ArrayBufferData]]
    // constructor is `allocate_array_buffer` (kahvi-core object/typed_array.rs),
    // which sets no shared flag, and the SharedArrayBuffer constructor is still a
    // not-implemented stub, so no shared data block can reach this function.
    // 4. If IsDetachedBuffer(_obj_) is *true*, return *+0*_𝔽.
    if is_detached_buffer(agent, obj) {
        return Ok(Value::Number(0.0));
    }
    // 5. If IsFixedLengthArrayBuffer(_obj_) is *true*, then
    //    a. Let _length_ be _obj_.[[ArrayBufferByteLength]].
    // 6. Else,
    //    a. Let _length_ be _obj_.[[ArrayBufferMaxByteLength]].
    // IsFixedLengthArrayBuffer(_obj_) is true exactly when _obj_ has no
    // [[ArrayBufferMaxByteLength]] slot, so the match below mirrors it: a buffer
    // with a max length is resizable and reports that max; otherwise it is
    // fixed-length and reports its current byte length.
    let length = match array_buffer_data(agent, obj).and_then(|d| d.max_byte_length) {
        // 6. Else, a. Let _length_ be _obj_.[[ArrayBufferMaxByteLength]].
        Some(max) => max,
        // 5. If IsFixedLengthArrayBuffer(_obj_) is *true*, then
        //    a. Let _length_ be _obj_.[[ArrayBufferByteLength]].
        None => array_buffer_byte_length(agent, obj),
    };
    // 7. Return 𝔽(_length_).
    Ok(Value::Number(length as f64))
}
