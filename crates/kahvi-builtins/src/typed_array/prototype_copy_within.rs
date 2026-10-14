//! %TypedArray%.prototype.copyWithin ( _target_, _start_ [ , _end_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.copywithin

use kahvi_core::object::typed_array::{
    ElementType, get_value_from_buffer, set_value_in_buffer, typed_array_data, typed_array_length,
};
use kahvi_core::prelude::*;

use crate::support::typed_array::validate_typed_array;

/// TypedArray.prototype.copyWithin
pub(crate) fn prototype_copy_within(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.copyWithin` as defined in [sec-array.prototype.copywithin].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    // 4. Let _targetIndex_ be ? ToClampedIndex(_target_, _length_).
    // 5. Let _startIndex_ be ? ToClampedIndex(_start_, _length_).
    // 6. If _end_ is *undefined*, let _endIndex_ be _length_; else let _endIndex_ be ? ToClampedIndex(_end_, _length_).
    // 7. Let _count_ be min(_endIndex_ - _startIndex_, _length_ - _targetIndex_).
    // 8. If _count_ > 0, then
    //    a. NOTE: The copying must be performed in a manner that preserves the bit-level encoding of the source data.
    //    b. Let _buffer_ be _obj_.[[ViewedArrayBuffer]].
    //    c. Set _taRecord_ to ? ValidateTypedArrayBounds(_obj_, ~seq-cst~).
    //    d. Set _length_ to TypedArrayLength(_taRecord_).
    //    e. NOTE: Side-effects of the above steps may have reduced the size of _obj_, in which case copying should proceed with the longest still-applicable prefix.
    //    f. Set _count_ to min(_count_, _length_ - _startIndex_, _length_ - _targetIndex_).
    //    g. Let _elementSize_ be TypedArrayElementSize(_obj_).
    //    h. Let _byteOffset_ be _obj_.[[ByteOffset]].
    //    i. Let _toByteIndex_ be (_targetIndex_ × _elementSize_) + _byteOffset_.
    //    j. Let _fromByteIndex_ be (_startIndex_ × _elementSize_) + _byteOffset_.
    //    k. Let _countBytes_ be _count_ × _elementSize_.
    //    l. If _fromByteIndex_ < _toByteIndex_ and _toByteIndex_ < _fromByteIndex_ + _countBytes_, then
    //       i. Let _direction_ be -1.
    //       ii. Set _fromByteIndex_ to _fromByteIndex_ + _countBytes_ - 1.
    //       iii. Set _toByteIndex_ to _toByteIndex_ + _countBytes_ - 1.
    //    m. Else,
    //       i. Let _direction_ be 1.
    //    n. Repeat, while _countBytes_ > 0,
    //       i. Assert: _fromByteIndex_ ≥ 0 and _toByteIndex_ ≥ 0.
    //       ii. Let _value_ be GetValueFromBuffer(_buffer_, _fromByteIndex_, ~uint8~, *true*, ~unordered~).
    //       iii. Perform SetValueInBuffer(_buffer_, _toByteIndex_, ~uint8~, _value_, *true*, ~unordered~).
    //       iv. Set _fromByteIndex_ to _fromByteIndex_ + _direction_.
    //       v. Set _toByteIndex_ to _toByteIndex_ + _direction_.
    //       vi. Set _countBytes_ to _countBytes_ - 1.
    // 9. Return _obj_.
    let _ = new_target;
    // 1. Let obj be the this value.
    // 2. Let taRecord be ? ValidateTypedArray(obj, seq-cst).
    let ta_record = validate_typed_array(agent, this)?;
    let obj = ta_record.object;
    // 3. Let length be TypedArrayLength(taRecord).
    let length = typed_array_length(agent, ta_record) as f64;
    // 4. Let targetIndex be ? ToClampedIndex(target, length).
    let target_index = to_clamped_index(agent, args.get_or_undefined(0), length)?;
    // 5. Let startIndex be ? ToClampedIndex(start, length).
    let start_index = to_clamped_index(agent, args.get_or_undefined(1), length)?;
    // 6. If end is undefined, let endIndex be length; else let endIndex be ? ToClampedIndex(end, length).
    let end = args.get_or_undefined(2);
    let end_index = if end.is_undefined() {
        length
    } else {
        to_clamped_index(agent, end, length)?
    };
    // 7. Let count be min(endIndex - startIndex, length - targetIndex).
    let count = (end_index - start_index).min(length - target_index);
    // 8. If count > 0, then
    if count > 0.0 {
        // a. NOTE: The copying must be performed in a manner that preserves the bit-level encoding of the source data.
        // b. Let buffer be obj.[[ViewedArrayBuffer]].
        let Some(buffer) = typed_array_data(agent, obj).and_then(|d| d.buffer) else {
            return Err(agent.throw_type_error("TypedArray has no buffer"));
        };
        // c. Set taRecord to ? ValidateTypedArrayBounds(obj, seq-cst).
        let ta_record = validate_typed_array(agent, Value::Object(obj))?;
        // d. Set length to TypedArrayLength(taRecord).
        let length = typed_array_length(agent, ta_record) as f64;
        // e. NOTE: Side-effects of the above steps may have reduced the size of obj, in which case copying should proceed with the longest still-applicable prefix.
        // f. Set count to min(count, length - startIndex, length - targetIndex).
        let count = count.min(length - start_index).min(length - target_index);
        // g. Let elementSize be TypedArrayElementSize(obj).
        let Some((element_size, byte_offset)) =
            typed_array_data(agent, obj).map(|d| (d.kind.size(), d.byte_offset))
        else {
            return Err(agent.throw_type_error("not a TypedArray"));
        };
        // h. Let byteOffset be obj.[[ByteOffset]].
        // i. Let toByteIndex be (targetIndex × elementSize) + byteOffset.
        let mut to_byte_index = target_index as i64 * element_size as i64 + byte_offset as i64;
        // j. Let fromByteIndex be (startIndex × elementSize) + byteOffset.
        let mut from_byte_index = start_index as i64 * element_size as i64 + byte_offset as i64;
        // k. Let countBytes be count × elementSize.
        let mut count_bytes = count as i64 * element_size as i64;
        // l. If fromByteIndex < toByteIndex and toByteIndex < fromByteIndex + countBytes, then
        let direction =
            if from_byte_index < to_byte_index && to_byte_index < from_byte_index + count_bytes {
                // i. Let direction be -1.
                // ii. Set fromByteIndex to fromByteIndex + countBytes - 1.
                from_byte_index += count_bytes - 1;
                // iii. Set toByteIndex to toByteIndex + countBytes - 1.
                to_byte_index += count_bytes - 1;
                -1
            } else {
                // m. Else,
                //    i. Let direction be 1.
                1
            };
        // n. Repeat, while countBytes > 0,
        while count_bytes > 0 {
            // i. Assert: fromByteIndex ≥ 0 and toByteIndex ≥ 0.
            if from_byte_index < 0 || to_byte_index < 0 {
                return Err(agent.throw_range_error("copyWithin byte index out of range"));
            }
            // ii. Let value be GetValueFromBuffer(buffer, fromByteIndex, uint8, true, unordered).
            let value = get_value_from_buffer(
                agent,
                buffer,
                from_byte_index as usize,
                ElementType::Uint8,
                true,
            );
            // iii. Perform SetValueInBuffer(buffer, toByteIndex, uint8, value, true, unordered).
            set_value_in_buffer(
                agent,
                buffer,
                to_byte_index as usize,
                ElementType::Uint8,
                value,
                true,
            );
            // iv. Set fromByteIndex to fromByteIndex + direction.
            from_byte_index += direction;
            // v. Set toByteIndex to toByteIndex + direction.
            to_byte_index += direction;
            // vi. Set countBytes to countBytes - 1.
            count_bytes -= 1;
        }
    }
    // 9. Return obj.
    Ok(Value::Object(obj))
}

/// ToClampedIndex ( value, length )
fn to_clamped_index(agent: &mut Agent, value: Value, length: f64) -> JsResult<f64> {
    // 1. Let index be ? ToAbsoluteIndex(value, length).
    let relative_index = ops::to_integer_or_infinity(agent, value)?;
    let index = if relative_index == f64::NEG_INFINITY {
        0.0
    } else if relative_index < 0.0 {
        (length + relative_index).max(0.0)
    } else {
        relative_index.min(length)
    };
    // 2. Return the result of clamping index between 0 and length.
    Ok(index.clamp(0.0, length))
}
