//! ArrayBuffer.prototype.slice ( _start_, _end_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-arraybuffer.prototype.slice

use kahvi_core::object::typed_array::{
    array_buffer_byte_length, array_buffer_data, array_buffer_data_mut, is_detached_buffer,
};
use kahvi_core::prelude::*;

/// ToClampedIndex(_value_, _length_)
///
/// Spec: https://tc39.es/ecma262/#sec-toclampedindex
///
/// Not yet in `ops`; only the TypedArray and Array built-ins need it so far.
fn to_clamped_index(agent: &mut Agent, value: Value, length: f64) -> JsResult<f64> {
    // 1. Let _index_ be ? ToAbsoluteIndex(_value_, _length_).
    //    ToAbsoluteIndex ( _start_, _length_ ):
    //    1. Let _relativeIndex_ be ? ToIntegerOrInfinity(_start_).
    let relative_index = ops::to_integer_or_infinity(agent, value)?;
    //    2. If _relativeIndex_ is -∞, return +0𝔽.
    //    3. Else if _relativeIndex_ < 0, return max(_length_ + _relativeIndex_, 0).
    //    4. Else, return min(_relativeIndex_, _length_).
    let index = if relative_index == f64::NEG_INFINITY {
        0.0
    } else if relative_index < 0.0 {
        (length + relative_index).max(0.0)
    } else {
        relative_index.min(length)
    };
    // 2. Return the result of clamping _index_ between 0 and _length_.
    Ok(index.clamp(0.0, length))
}

/// CopyDataBlockBytes(toBlock, toIndex, fromBlock, fromIndex, count)
///
/// Spec: https://tc39.es/ecma262/#sec-copydatablockbytes
///
/// The data blocks are the `[[ArrayBufferData]]` of the two buffers. Not yet in
/// `ops`; only ArrayBuffer.prototype.slice needs it so far.
fn copy_data_block_bytes(
    agent: &mut Agent,
    to_block: Object,
    to_index: usize,
    from_block: Object,
    from_index: usize,
    count: usize,
) {
    // Copy the source bytes out first, so the two buffers are never borrowed at once.
    let bytes = array_buffer_data(agent, from_block)
        .and_then(|d| d.data.as_ref())
        .expect("source buffer is not detached")[from_index..from_index + count]
        .to_vec();
    array_buffer_data_mut(agent, to_block)
        .and_then(|d| d.data.as_mut())
        .expect("target buffer is not detached")[to_index..to_index + count]
        .copy_from_slice(&bytes);
}

/// ArrayBuffer.prototype.slice
pub(crate) fn prototype_slice(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[ArrayBufferData]]).
    let obj = match this {
        Value::Object(o) if array_buffer_data(agent, o).is_some() => o,
        _ => {
            return Err(
                agent.throw_type_error("ArrayBuffer.prototype.slice called on a non-ArrayBuffer")
            );
        }
    };
    // 3. If IsSharedArrayBuffer(_obj_) is *true*, throw a *TypeError* exception.
    if kahvi_core::object::typed_array::is_shared_array_buffer(agent, obj) {
        return Err(agent.throw_type_error("not an ArrayBuffer (a SharedArrayBuffer)"));
    }
    // 4. If IsDetachedBuffer(_obj_) is *true*, throw a *TypeError* exception.
    if is_detached_buffer(agent, obj) {
        return Err(agent.throw_type_error("ArrayBuffer is detached"));
    }
    // 5. Let _length_ be _obj_.[[ArrayBufferByteLength]].
    let length = array_buffer_byte_length(agent, obj) as f64;
    // 6. Let _first_ be ? ToClampedIndex(_start_, _length_).
    let first = to_clamped_index(agent, args.get_or_undefined(0), length)?;
    // 7. If _end_ is *undefined*, let _final_ be _length_; else let _final_ be ? ToClampedIndex(_end_, _length_).
    let end = args.get_or_undefined(1);
    let final_index = if end.is_undefined() {
        length
    } else {
        to_clamped_index(agent, end, length)?
    };
    // 8. Let _newLength_ be max(_final_ - _first_, 0).
    let new_length = (final_index - first).max(0.0);
    // 9. Let _ctor_ be ? SpeciesConstructor(_obj_, %ArrayBuffer%).
    let default_ctor = agent.intrinsic("%ArrayBuffer%");
    let ctor = ops::species_constructor(agent, obj, default_ctor)?;
    // 10. Let _new_ be ? Construct(_ctor_, « 𝔽(_newLength_) »).
    let new = ops::construct(agent, ctor, &[Value::Number(new_length)], None)?;
    // 11. Perform ? RequireInternalSlot(_new_, [[ArrayBufferData]]).
    if array_buffer_data(agent, new).is_none() {
        return Err(agent.throw_type_error("species constructor did not return an ArrayBuffer"));
    }
    // 12. If IsSharedArrayBuffer(_new_) is *true*, throw a *TypeError* exception.
    if kahvi_core::object::typed_array::is_shared_array_buffer(agent, new) {
        return Err(agent.throw_type_error("species constructor returned a SharedArrayBuffer"));
    }
    // 13. If IsDetachedBuffer(_new_) is *true*, throw a *TypeError* exception.
    if is_detached_buffer(agent, new) {
        return Err(agent.throw_type_error("new ArrayBuffer is detached"));
    }
    // 14. If SameValue(_new_, _obj_) is *true*, throw a *TypeError* exception.
    if ops::same_value(agent, Value::Object(new), Value::Object(obj)) {
        return Err(agent.throw_type_error("species constructor returned the same ArrayBuffer"));
    }
    // 15. If _new_.[[ArrayBufferByteLength]] < _newLength_, throw a *TypeError* exception.
    if (array_buffer_byte_length(agent, new) as f64) < new_length {
        return Err(agent.throw_type_error("new ArrayBuffer is too small"));
    }
    // 16. NOTE: Side-effects of the above steps may have detached or resized _obj_.
    // 17. If IsDetachedBuffer(_obj_) is *true*, throw a *TypeError* exception.
    if is_detached_buffer(agent, obj) {
        return Err(agent.throw_type_error("ArrayBuffer is detached"));
    }
    // 18. Let _fromBlock_ be _obj_.[[ArrayBufferData]].
    // 19. Let _toBlock_ be _new_.[[ArrayBufferData]].
    // 20. Let _currentLength_ be _obj_.[[ArrayBufferByteLength]].
    let current_length = array_buffer_byte_length(agent, obj) as f64;
    // 21. Let _maxCount_ be _currentLength_ - _first_.
    let max_count = current_length - first;
    // 22. If _maxCount_ > 0, then
    if max_count > 0.0 {
        // a. Let _count_ be min(_newLength_, _maxCount_).
        let count = new_length.min(max_count);
        // b. Perform CopyDataBlockBytes(_toBlock_, 0, _fromBlock_, _first_, _count_).
        copy_data_block_bytes(agent, new, 0, obj, first as usize, count as usize);
    }
    // 23. Return _new_.
    Ok(Value::Object(new))
}
