//! Atomics.load ( _ta_, _index_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.load

use kahvi_core::prelude::*;

/// Atomics.load
pub(crate) fn load(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _byteIndexInBuffer_ be ? ValidateAtomicAccessOnIntegerTypedArray(_ta_, _index_).
    // 2. Perform ? RevalidateAtomicAccess(_ta_, _byteIndexInBuffer_).
    // 3. Let _buffer_ be _ta_.[[ViewedArrayBuffer]].
    // 4. Let _elementType_ be TypedArrayElementType(_ta_).
    // 5. Return GetValueFromBuffer(_buffer_, _byteIndexInBuffer_, _elementType_, *true*, ~seq-cst~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Atomics.load"))
}
