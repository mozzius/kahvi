//! Atomics.load ( _ta_, _index_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.load

use crate::support::atomics::{
    LITTLE_ENDIAN, element_type, revalidate_atomic_access,
    validate_atomic_access_on_integer_typed_array, viewed_buffer,
};
use kahvi_core::object::typed_array::get_value_from_buffer;
use kahvi_core::prelude::*;

/// Atomics.load
pub(crate) fn load(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (this, new_target);
    let (ta, index) = (args.get_or_undefined(0), args.get_or_undefined(1));
    // This function performs the following steps when called:
    //
    // 1. Let _byteIndexInBuffer_ be ? ValidateAtomicAccessOnIntegerTypedArray(_ta_, _index_).
    let byte_index_in_buffer = validate_atomic_access_on_integer_typed_array(agent, ta, index)?;
    let Value::Object(ta) = ta else {
        unreachable!("validated by ValidateAtomicAccessOnIntegerTypedArray")
    };
    // 2. Perform ? RevalidateAtomicAccess(_ta_, _byteIndexInBuffer_).
    revalidate_atomic_access(agent, ta, byte_index_in_buffer)?;
    // 3. Let _buffer_ be _ta_.[[ViewedArrayBuffer]].
    let buffer = viewed_buffer(agent, ta);
    // 4. Let _elementType_ be TypedArrayElementType(_ta_).
    let element_type = element_type(agent, ta);
    // 5. Return GetValueFromBuffer(_buffer_, _byteIndexInBuffer_, _elementType_, *true*, ~seq-cst~).
    Ok(get_value_from_buffer(
        agent,
        buffer,
        byte_index_in_buffer,
        element_type,
        LITTLE_ENDIAN,
    ))
}
