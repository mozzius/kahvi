//! ArrayBuffer ( _length_ [ , _options_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-arraybuffer-length

use kahvi_core::prelude::*;

use crate::support::typed_array::{allocate_buffer, get_array_buffer_max_byte_length_option};

/// ArrayBuffer
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If NewTarget is undefined, throw a TypeError exception.
    let Some(new_target) = new_target else {
        return Err(agent.throw_type_error("Constructor ArrayBuffer requires 'new'"));
    };
    // 2. Let byteLength be ? ToIndex(length).
    let byte_length = ops::to_index(agent, args.get_or_undefined(0))?;
    // 3. Let requestedMaxByteLength be ? GetArrayBufferMaxByteLengthOption(options).
    let max = get_array_buffer_max_byte_length_option(agent, args.get_or_undefined(1))?;
    // 4. Return ? AllocateArrayBuffer(NewTarget, byteLength, requestedMaxByteLength).
    Ok(Value::Object(allocate_buffer(
        agent,
        new_target,
        byte_length,
        max,
    )?))
}
