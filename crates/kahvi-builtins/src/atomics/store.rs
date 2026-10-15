//! Atomics.store ( _ta_, _index_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.store

use crate::support::atomics::{
    LITTLE_ENDIAN, coerce, element_type, revalidate_atomic_access,
    validate_atomic_access_on_integer_typed_array, viewed_buffer,
};
use kahvi_core::object::typed_array::set_value_in_buffer;
use kahvi_core::prelude::*;

/// Atomics.store
pub(crate) fn store(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (this, new_target);
    let (ta, index, value) = (
        args.get_or_undefined(0),
        args.get_or_undefined(1),
        args.get_or_undefined(2),
    );
    // This function performs the following steps when called:
    //
    // 1. Let _byteIndexInBuffer_ be ? ValidateAtomicAccessOnIntegerTypedArray(_ta_, _index_).
    let byte_index_in_buffer = validate_atomic_access_on_integer_typed_array(agent, ta, index)?;
    let Value::Object(ta) = ta else {
        unreachable!("validated by ValidateAtomicAccessOnIntegerTypedArray")
    };
    // 2. If _ta_.[[ContentType]] is ~bigint~, let _coerced_ be ? ToBigInt(_value_).
    // 3. Else, let _coerced_ be 𝔽(? ToIntegerOrInfinity(_value_)).
    let coerced = coerce(agent, ta, value)?;
    // 4. Perform ? RevalidateAtomicAccess(_ta_, _byteIndexInBuffer_).
    revalidate_atomic_access(agent, ta, byte_index_in_buffer)?;
    // 5. Let _buffer_ be _ta_.[[ViewedArrayBuffer]].
    let buffer = viewed_buffer(agent, ta);
    // 6. Let _elementType_ be TypedArrayElementType(_ta_).
    let element_type = element_type(agent, ta);
    // 7. Perform SetValueInBuffer(_buffer_, _byteIndexInBuffer_, _elementType_, _coerced_, *true*, ~seq-cst~).
    set_value_in_buffer(
        agent,
        buffer,
        byte_index_in_buffer,
        element_type,
        coerced,
        LITTLE_ENDIAN,
    );
    // 8. Return _coerced_.
    Ok(coerced)
}
