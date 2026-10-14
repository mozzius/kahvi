//! ArrayBuffer.prototype.transfer ( [ _newLength_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-arraybuffer.prototype.transfer

use kahvi_core::object::typed_array::{
    allocate_array_buffer, array_buffer_byte_length, array_buffer_data, array_buffer_data_mut,
    detach_array_buffer, is_detached_buffer, is_fixed_length_array_buffer,
};
use kahvi_core::prelude::*;

/// ArrayBufferCopyAndDetach(_arrayBuffer_, _newLength_, _preserveResizability_)
///
/// Spec: https://tc39.es/ecma262/#sec-arraybuffercopyanddetach
///
/// `preserve_resizability` is ~preserve-resizability~ when true and ~fixed-length~ when
/// false. Not yet in `ops`; ArrayBuffer.prototype.transferToFixedLength needs it too.
fn array_buffer_copy_and_detach(
    agent: &mut Agent,
    array_buffer: Value,
    new_length: Value,
    preserve_resizability: bool,
) -> JsResult<Object> {
    // 1. Perform ? RequireInternalSlot(_arrayBuffer_, [[ArrayBufferData]]).
    let obj = match array_buffer {
        Value::Object(o) if array_buffer_data(agent, o).is_some() => o,
        _ => {
            return Err(agent
                .throw_type_error("ArrayBuffer.prototype.transfer called on a non-ArrayBuffer"));
        }
    };
    // 2. If IsSharedArrayBuffer(_arrayBuffer_) is *true*, throw a *TypeError* exception.
    //    The engine has no Shared Data Blocks, so no ArrayBuffer is ever shared.
    // 3. If _newLength_ is *undefined*, then
    let new_byte_length = if new_length.is_undefined() {
        // a. Let _newByteLength_ be _arrayBuffer_.[[ArrayBufferByteLength]].
        array_buffer_byte_length(agent, obj) as f64
    } else {
        // 4.a. Let _newByteLength_ be ? ToIndex(_newLength_).
        ops::to_index(agent, new_length)?
    };
    // 5. If IsDetachedBuffer(_arrayBuffer_) is *true*, throw a *TypeError* exception.
    if is_detached_buffer(agent, obj) {
        return Err(agent.throw_type_error("ArrayBuffer is detached"));
    }
    // 6. If _preserveResizability_ is ~preserve-resizability~ and IsFixedLengthArrayBuffer(_arrayBuffer_) is *false*, then
    let new_max_byte_length = if preserve_resizability && !is_fixed_length_array_buffer(agent, obj)
    {
        // a. Let _newMaxByteLength_ be _arrayBuffer_.[[ArrayBufferMaxByteLength]].
        array_buffer_data(agent, obj)
            .and_then(|d| d.max_byte_length)
            .map(|m| m as f64)
    } else {
        // 7. Else, let _newMaxByteLength_ be ~empty~.
        None
    };
    // 8. If _arrayBuffer_.[[ArrayBufferDetachKey]] is not *undefined*, throw a *TypeError* exception.
    let detach_key = array_buffer_data(agent, obj)
        .expect("an ArrayBuffer")
        .detach_key;
    if !detach_key.is_undefined() {
        return Err(agent.throw_type_error("ArrayBuffer detach key is not undefined"));
    }
    // 9. Let _newBuffer_ be ? AllocateArrayBuffer(%ArrayBuffer%, _newByteLength_, _newMaxByteLength_).
    let ctor = agent.intrinsic("%ArrayBuffer%");
    let new_buffer = allocate_array_buffer(agent, ctor, new_byte_length, new_max_byte_length)?;
    // 10. Let _copyLength_ be min(_newByteLength_, _arrayBuffer_.[[ArrayBufferByteLength]]).
    let copy_length = new_byte_length.min(array_buffer_byte_length(agent, obj) as f64) as usize;
    // 11. Let _fromBlock_ be _arrayBuffer_.[[ArrayBufferData]].
    // 12. Let _toBlock_ be _newBuffer_.[[ArrayBufferData]].
    // 13. Perform CopyDataBlockBytes(_toBlock_, 0, _fromBlock_, 0, _copyLength_).
    //     Copy the source bytes out first, so the two buffers are never borrowed at once.
    let bytes = array_buffer_data(agent, obj)
        .and_then(|d| d.data.as_ref())
        .expect("source buffer is not detached")[..copy_length]
        .to_vec();
    array_buffer_data_mut(agent, new_buffer)
        .and_then(|d| d.data.as_mut())
        .expect("new buffer is not detached")[..copy_length]
        .copy_from_slice(&bytes);
    // 14. NOTE: Neither creation of the new Data Block nor copying from the old Data Block are observable.
    // 15. Perform ! DetachArrayBuffer(_arrayBuffer_).
    detach_array_buffer(agent, obj, Value::Undefined)?;
    // 16. Return _newBuffer_.
    Ok(new_buffer)
}

/// ArrayBuffer.prototype.transfer
pub(crate) fn prototype_transfer(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _obj_ be the *this* value.
    // 2. Return ? ArrayBufferCopyAndDetach(_obj_, _newLength_, ~preserve-resizability~).
    let new_buffer = array_buffer_copy_and_detach(agent, this, args.get_or_undefined(0), true)?;
    Ok(Value::Object(new_buffer))
}
