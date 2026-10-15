//! ArrayBuffer.prototype.resize ( _newLength_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-arraybuffer.prototype.resize

use kahvi_core::object::typed_array::{
    array_buffer_byte_length, array_buffer_data, array_buffer_data_mut, create_byte_data_block,
    is_detached_buffer,
};
use kahvi_core::prelude::*;

/// ArrayBuffer.prototype.resize
pub(crate) fn prototype_resize(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let obj be the this value.
    let obj = this;
    // 2. Perform ? RequireInternalSlot(obj, [[ArrayBufferMaxByteLength]]).
    let Value::Object(o) = obj else {
        return Err(agent.throw_type_error("ArrayBuffer.prototype.resize called on a non-object"));
    };
    let max_byte_length = match array_buffer_data(agent, o).and_then(|d| d.max_byte_length) {
        Some(max) => max,
        None => {
            return Err(agent.throw_type_error(
                "ArrayBuffer.prototype.resize called on a non-resizable ArrayBuffer",
            ));
        }
    };
    // 3. If IsSharedArrayBuffer(obj) is true, throw a TypeError exception.
    if kahvi_core::object::typed_array::is_shared_array_buffer(agent, o) {
        return Err(agent.throw_type_error("not an ArrayBuffer (a SharedArrayBuffer)"));
    }
    // 4. Let newByteLength be ? ToIndex(newLength).
    let new_byte_length = ops::to_index(agent, args.get_or_undefined(0))?;
    // 5. If IsDetachedBuffer(obj) is true, throw a TypeError exception.
    if is_detached_buffer(agent, o) {
        return Err(agent.throw_type_error("ArrayBuffer is detached"));
    }
    // 6. If newByteLength > obj.[[ArrayBufferMaxByteLength]], throw a RangeError exception.
    if new_byte_length > max_byte_length as f64 {
        return Err(agent.throw_range_error("new length exceeds the maximum byte length"));
    }
    // 7. Let hostHandled be ? HostResizeArrayBuffer(obj, newByteLength).
    // 8. If hostHandled is handled, return undefined.
    // This engine's host does not handle resizing, so HostResizeArrayBuffer returns unhandled.
    // 9. Let oldBlock be obj.[[ArrayBufferData]].
    // 10. Let newBlock be ? CreateByteDataBlock(newByteLength).
    let mut new_block = create_byte_data_block(agent, new_byte_length)?;
    // 11. Let copyLength be min(newByteLength, obj.[[ArrayBufferByteLength]]).
    let old_byte_length = array_buffer_byte_length(agent, o) as f64;
    let copy_length = new_byte_length.min(old_byte_length) as usize;
    // 12. Perform CopyDataBlockBytes(newBlock, 0, oldBlock, 0, copyLength).
    let old_block = array_buffer_data(agent, o)
        .and_then(|d| d.data.as_ref())
        .expect("not detached: checked in step 5");
    new_block[..copy_length].copy_from_slice(&old_block[..copy_length]);
    // 13. NOTE: Neither creation of the new Data Block nor copying from the old Data Block are observable.
    // 14. Set obj.[[ArrayBufferData]] to newBlock.
    // 15. Set obj.[[ArrayBufferByteLength]] to newByteLength.
    // The byte length is the length of the data block, so storing newBlock sets both.
    let data = array_buffer_data_mut(agent, o).expect("an ArrayBuffer");
    data.data = Some(new_block);
    // 16. Return undefined.
    Ok(Value::Undefined)
}
