//! SharedArrayBuffer ( _length_ [ , _options_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-sharedarraybuffer-length

use crate::support::typed_array::get_array_buffer_max_byte_length_option;
use kahvi_core::object::typed_array::allocate_shared_array_buffer;
use kahvi_core::prelude::*;

/// SharedArrayBuffer
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = this;
    let (length, options) = (args.get_or_undefined(0), args.get_or_undefined(1));
    // 1. If NewTarget is undefined, throw a TypeError exception.
    let Some(new_target) = new_target else {
        return Err(agent.throw_type_error("Constructor SharedArrayBuffer requires 'new'"));
    };
    // 2. Let byteLength be ? ToIndex(length).
    let byte_length = ops::to_index(agent, length)?;
    // 3. Let requestedMaxByteLength be ? GetArrayBufferMaxByteLengthOption(options).
    let requested_max_byte_length = get_array_buffer_max_byte_length_option(agent, options)?;
    // 4. Return ? AllocateSharedArrayBuffer(NewTarget, byteLength, requestedMaxByteLength).
    allocate_shared_array_buffer(agent, new_target, byte_length, requested_max_byte_length)
        .map(Value::Object)
}
