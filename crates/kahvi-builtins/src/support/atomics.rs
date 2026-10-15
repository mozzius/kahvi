//! The abstract operations of the Atomics object (§25.4.3). With one agent,
//! every Atomics operation is a plain read, write or read-modify-write of
//! the buffer's bytes: there is no other agent to observe its ordering.

use kahvi_core::object::typed_array::{
    ElementType, TaRecord, array_buffer_data_mut, numeric_to_raw_bytes, raw_bytes_to_numeric,
    typed_array_data, typed_array_length,
};
use kahvi_core::prelude::*;

use super::typed_array::validate_typed_array;

/// The surrounding agent's [[LittleEndian]] (as for every other buffer access).
pub(crate) const LITTLE_ENDIAN: bool = true;

/// IsNoTearConfiguration(type, ~seq-cst~)
fn is_no_tear_configuration_seq_cst(ty: ElementType) -> bool {
    // 1. If IsUnclampedIntegerElementType(type) is true, return true.
    // 2. If IsBigIntElementType(type) is true and order is neither init nor unordered, return true.
    // 3. Return false.
    !matches!(
        ty,
        ElementType::Uint8C | ElementType::Float16 | ElementType::Float32 | ElementType::Float64
    )
}

/// ValidateIntegerTypedArray(ta, waitable)
pub(crate) fn validate_integer_typed_array(
    agent: &mut Agent,
    ta: Value,
    waitable: bool,
) -> JsResult<TaRecord> {
    // 2. Let taRecord be ? ValidateTypedArray(ta, unordered).
    let ta_record = validate_typed_array(agent, ta)?;
    // 3. Let elementType be TypedArrayElementType(ta).
    let element_type = element_type(agent, ta_record.object);
    // 4. If IsNoTearConfiguration(elementType, seq-cst) is false, throw a TypeError exception.
    if !is_no_tear_configuration_seq_cst(element_type) {
        return Err(agent.throw_type_error("Atomics needs an integer TypedArray"));
    }
    // 5. If waitable is true and elementType is neither int32 nor bigint64, throw a TypeError exception.
    if waitable && !matches!(element_type, ElementType::Int32 | ElementType::BigInt64) {
        return Err(agent.throw_type_error("Atomics needs an Int32Array or BigInt64Array"));
    }
    // 6. Return taRecord.
    Ok(ta_record)
}

/// ValidateAtomicAccess(taRecord, requestIndex): the byte index in the buffer.
pub(crate) fn validate_atomic_access(
    agent: &mut Agent,
    ta_record: TaRecord,
    request_index: Value,
) -> JsResult<usize> {
    // 1. Let length be TypedArrayLength(taRecord).
    let length = typed_array_length(agent, ta_record);
    // 2. Let accessIndex be ? ToIndex(requestIndex).
    let access_index = ops::to_index(agent, request_index)?;
    // 4. If accessIndex ≥ length, throw a RangeError exception.
    if access_index >= length as f64 {
        return Err(agent.throw_range_error("Atomics access index out of range"));
    }
    // 5-7. Let ta be taRecord.[[Object]]; elementSize and offset are ta's.
    let d = typed_array_data(agent, ta_record.object).expect("a TypedArray");
    // 8. Return (accessIndex × elementSize) + offset.
    Ok(access_index as usize * d.kind.size() + d.byte_offset)
}

/// ValidateAtomicAccessOnIntegerTypedArray(ta, requestIndex)
pub(crate) fn validate_atomic_access_on_integer_typed_array(
    agent: &mut Agent,
    ta: Value,
    request_index: Value,
) -> JsResult<usize> {
    // 1. Let taRecord be ? ValidateIntegerTypedArray(ta, false).
    let ta_record = validate_integer_typed_array(agent, ta, false)?;
    // 2. Return ? ValidateAtomicAccess(taRecord, requestIndex).
    validate_atomic_access(agent, ta_record, request_index)
}

/// RevalidateAtomicAccess(ta, byteIndexInBuffer)
pub(crate) fn revalidate_atomic_access(
    agent: &mut Agent,
    ta: Object,
    byte_index_in_buffer: usize,
) -> JsResult<()> {
    // 2. Let taRecord be ? ValidateTypedArrayBounds(ta, unordered).
    let ta_record = validate_typed_array(agent, Value::Object(ta))?;
    // 4. Let elementSize be TypedArrayElementSize(ta).
    let element_size = element_type(agent, ta).size();
    // 5. Let elementLastByteIndex be byteIndexInBuffer + elementSize - 1.
    let element_last_byte_index = byte_index_in_buffer + element_size - 1;
    // 6. If elementLastByteIndex ≥ taRecord.[[CachedBufferByteLength]], throw a RangeError exception.
    if element_last_byte_index >= ta_record.cached_buffer_byte_length.unwrap_or(0) {
        return Err(agent.throw_range_error("Atomics access index out of range"));
    }
    // 7. Return unused.
    Ok(())
}

/// TypedArrayElementType(ta)
pub(crate) fn element_type(agent: &Agent, ta: Object) -> ElementType {
    typed_array_data(agent, ta).expect("a TypedArray").kind
}

/// [[ViewedArrayBuffer]]
pub(crate) fn viewed_buffer(agent: &Agent, ta: Object) -> Object {
    typed_array_data(agent, ta)
        .expect("a TypedArray")
        .buffer
        .expect("initialized")
}

/// A read-modify-write modification function: (xBytes, yBytes) -> bytes.
pub(crate) type ModifyOp<'a> = &'a mut dyn FnMut(&mut Agent, &[u8], &[u8]) -> Vec<u8>;

/// GetModifySetValueInBuffer(arrayBuffer, byteIndex, type, value, op)
pub(crate) fn get_modify_set_value_in_buffer(
    agent: &mut Agent,
    array_buffer: Object,
    byte_index: usize,
    ty: ElementType,
    value: Value,
    op: ModifyOp,
) -> Value {
    // 5. Let elementSize be the Element Size value for Element Type type.
    let element_size = ty.size();
    // 8. Let rawBytes be NumericToRawBytes(type, value, isLittleEndian).
    let raw_bytes = numeric_to_raw_bytes(agent, ty, value, LITTLE_ENDIAN);
    // 9-10. (One agent: a Shared Data Block is read and written like any other.)
    // a. Let rawBytesRead be the elementSize bytes starting with block[byteIndex].
    let raw_bytes_read = array_buffer_data_mut(agent, array_buffer)
        .and_then(|d| d.data.as_mut())
        .expect("not detached")[byte_index..byte_index + element_size]
        .to_vec();
    // b. Let rawBytesModified be op(rawBytesRead, rawBytes).
    let modified = op(agent, &raw_bytes_read, &raw_bytes);
    // c. Store the individual bytes of rawBytesModified into block, starting at block[byteIndex].
    array_buffer_data_mut(agent, array_buffer)
        .and_then(|d| d.data.as_mut())
        .expect("not detached")[byte_index..byte_index + element_size]
        .copy_from_slice(&modified);
    // 11. Return RawBytesToNumeric(type, rawBytesRead, isLittleEndian).
    raw_bytes_to_numeric(agent, ty, &raw_bytes_read, LITTLE_ENDIAN)
}

/// AtomicReadModifyWrite(ta, index, value, op)
pub(crate) fn atomic_read_modify_write(
    agent: &mut Agent,
    ta: Value,
    index: Value,
    value: Value,
    op: ModifyOp,
) -> JsResult<Value> {
    // 1. Let byteIndexInBuffer be ? ValidateAtomicAccessOnIntegerTypedArray(ta, index).
    let byte_index_in_buffer = validate_atomic_access_on_integer_typed_array(agent, ta, index)?;
    let Value::Object(ta) = ta else {
        unreachable!("validated")
    };
    // 2-3. Coerce value to the TypedArray's content type.
    let coerced = coerce(agent, ta, value)?;
    // 4. Perform ? RevalidateAtomicAccess(ta, byteIndexInBuffer).
    revalidate_atomic_access(agent, ta, byte_index_in_buffer)?;
    // 5. Let buffer be ta.[[ViewedArrayBuffer]].
    let buffer = viewed_buffer(agent, ta);
    // 6. Let elementType be TypedArrayElementType(ta).
    let element_type = element_type(agent, ta);
    // 7. Return GetModifySetValueInBuffer(buffer, byteIndexInBuffer, elementType, coerced, op).
    Ok(get_modify_set_value_in_buffer(
        agent,
        buffer,
        byte_index_in_buffer,
        element_type,
        coerced,
        op,
    ))
}

/// If ta.[[ContentType]] is bigint, ? ToBigInt(value); else 𝔽(? ToIntegerOrInfinity(value)).
pub(crate) fn coerce(agent: &mut Agent, ta: Object, value: Value) -> JsResult<Value> {
    if element_type(agent, ta).is_bigint() {
        Ok(Value::BigInt(ops::bigint::to_bigint(agent, value)?))
    } else {
        Ok(Value::Number(ops::to_integer_or_infinity(agent, value)?))
    }
}

/// ByteListBitwiseOp(op, xBytes, yBytes)
pub(crate) fn byte_list_bitwise_op(op: fn(u8, u8) -> u8, x: &[u8], y: &[u8]) -> Vec<u8> {
    // 1-4. Apply op to each pair of bytes.
    x.iter().zip(y).map(|(&a, &b)| op(a, b)).collect()
}
