//! Atomics.sub ( _ta_, _index_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.sub

use kahvi_core::prelude::*;

/// Atomics.sub
pub(crate) fn sub(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _subtract_ be a new read-modify-write modification function with parameters (_xBytes_, _yBytes_) that captures _ta_ and performs the following steps atomically when called:
    //    a. Let _type_ be TypedArrayElementType(_ta_).
    //    b. Let _agentRecord_ be the Agent Record of the surrounding agent.
    //    c. Let _isLittleEndian_ be _agentRecord_.[[LittleEndian]].
    //    d. Let _x_ be RawBytesToNumeric(_type_, _xBytes_, _isLittleEndian_).
    //    e. Let _y_ be RawBytesToNumeric(_type_, _yBytes_, _isLittleEndian_).
    //    f. If _x_ is a Number, then
    //       i. Let _difference_ be Number::subtract(_x_, _y_).
    //    g. Else,
    //       i. Assert: _x_ is a BigInt.
    //       ii. Let _difference_ be BigInt::subtract(_x_, _y_).
    //    h. Let _differenceBytes_ be NumericToRawBytes(_type_, _difference_, _isLittleEndian_).
    //    i. Assert: _differenceBytes_, _xBytes_, and _yBytes_ have the same number of elements.
    //    j. Return _differenceBytes_.
    // 2. Return ? AtomicReadModifyWrite(_ta_, _index_, _value_, _subtract_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Atomics.sub"))
}
