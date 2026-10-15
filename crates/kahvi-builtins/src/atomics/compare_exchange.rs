//! Atomics.compareExchange ( _ta_, _index_, _expectedValue_, _replacementValue_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.compareexchange

use crate::support::atomics::{
    LITTLE_ENDIAN, coerce, element_type, revalidate_atomic_access,
    validate_atomic_access_on_integer_typed_array, viewed_buffer,
};
use kahvi_core::object::typed_array::{
    array_buffer_data_mut, numeric_to_raw_bytes, raw_bytes_to_numeric,
};
use kahvi_core::prelude::*;

/// Atomics.compareExchange
pub(crate) fn compare_exchange(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (this, new_target);
    let (ta, index, expected_value, replacement_value) = (
        args.get_or_undefined(0),
        args.get_or_undefined(1),
        args.get_or_undefined(2),
        args.get_or_undefined(3),
    );
    // 1. Let byteIndexInBuffer be ? ValidateAtomicAccessOnIntegerTypedArray(ta, index).
    let byte_index_in_buffer = validate_atomic_access_on_integer_typed_array(agent, ta, index)?;
    let Value::Object(ta) = ta else {
        unreachable!("validated by ValidateAtomicAccessOnIntegerTypedArray")
    };
    // 2. Let buffer be ta.[[ViewedArrayBuffer]].
    let buffer = viewed_buffer(agent, ta);
    // 3. Let block be buffer.[[ArrayBufferData]].
    // 4. If ta.[[ContentType]] is bigint, then
    //    a. Let expected be ? ToBigInt(expectedValue).
    //    b. Let replacement be ? ToBigInt(replacementValue).
    // 5. Else,
    //    a. Let expected be 𝔽(? ToIntegerOrInfinity(expectedValue)).
    //    b. Let replacement be 𝔽(? ToIntegerOrInfinity(replacementValue)).
    // (coerce applies the conversion for ta's content type, in this order.)
    let expected = coerce(agent, ta, expected_value)?;
    let replacement = coerce(agent, ta, replacement_value)?;
    // 6. Perform ? RevalidateAtomicAccess(ta, byteIndexInBuffer).
    revalidate_atomic_access(agent, ta, byte_index_in_buffer)?;
    // 7. Let elementType be TypedArrayElementType(ta).
    let element_type = element_type(agent, ta);
    // 8. Let elementSize be TypedArrayElementSize(ta).
    let element_size = element_type.size();
    // 9. Let agentRecord be the Agent Record of the surrounding agent.
    // 10. Let isLittleEndian be agentRecord.[[LittleEndian]].
    let is_little_endian = LITTLE_ENDIAN;
    // 11. Let expectedBytes be NumericToRawBytes(elementType, expected, isLittleEndian).
    let expected_bytes = numeric_to_raw_bytes(agent, element_type, expected, is_little_endian);
    // 12. Let replacementBytes be NumericToRawBytes(elementType, replacement, isLittleEndian).
    let replacement_bytes =
        numeric_to_raw_bytes(agent, element_type, replacement, is_little_endian);
    // 13. If IsSharedArrayBuffer(buffer) is true, then
    //     a. Let rawBytesRead be AtomicCompareExchangeInSharedBlock(block, byteIndexInBuffer, elementSize, expectedBytes, replacementBytes).
    // 14. Else,
    //     a. Let rawBytesRead be a List of length elementSize whose elements are the sequence of elementSize bytes starting with block[byteIndexInBuffer].
    //     b. If ByteListEqual(rawBytesRead, expectedBytes) is true, then
    //        i. Store the individual bytes of replacementBytes into block, starting at block[byteIndexInBuffer].
    // (One agent: a Shared Data Block is read and written like any other, so
    // steps 13 and 14 both reduce to the read, compare and conditional store below.)
    let raw_bytes_read = array_buffer_data_mut(agent, buffer)
        .and_then(|d| d.data.as_mut())
        .expect("not detached")[byte_index_in_buffer..byte_index_in_buffer + element_size]
        .to_vec();
    // ByteListEqual(xBytes, yBytes): equal lengths and equal bytes at every index.
    if raw_bytes_read == expected_bytes {
        array_buffer_data_mut(agent, buffer)
            .and_then(|d| d.data.as_mut())
            .expect("not detached")[byte_index_in_buffer..byte_index_in_buffer + element_size]
            .copy_from_slice(&replacement_bytes);
    }
    // 15. Return RawBytesToNumeric(elementType, rawBytesRead, isLittleEndian).
    Ok(raw_bytes_to_numeric(
        agent,
        element_type,
        &raw_bytes_read,
        is_little_endian,
    ))
}
