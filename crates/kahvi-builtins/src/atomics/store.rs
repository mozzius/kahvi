//! Atomics.store ( _ta_, _index_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.store

use kahvi_core::prelude::*;

/// Atomics.store
pub(crate) fn store(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _byteIndexInBuffer_ be ? ValidateAtomicAccessOnIntegerTypedArray(_ta_, _index_).
    // 2. If _ta_.[[ContentType]] is ~bigint~, let _coerced_ be ? ToBigInt(_value_).
    // 3. Else, let _coerced_ be 𝔽(? ToIntegerOrInfinity(_value_)).
    // 4. Perform ? RevalidateAtomicAccess(_ta_, _byteIndexInBuffer_).
    // 5. Let _buffer_ be _ta_.[[ViewedArrayBuffer]].
    // 6. Let _elementType_ be TypedArrayElementType(_ta_).
    // 7. Perform SetValueInBuffer(_buffer_, _byteIndexInBuffer_, _elementType_, _coerced_, *true*, ~seq-cst~).
    // 8. Return _coerced_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Atomics.store"))
}
