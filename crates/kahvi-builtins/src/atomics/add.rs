//! Atomics.add ( _ta_, _index_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.add

use kahvi_core::prelude::*;

/// Atomics.add
pub(crate) fn add(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _add_ be a new read-modify-write modification function with parameters (_xBytes_, _yBytes_) that captures _ta_ and performs the following steps atomically when called:
    //    a. Let _type_ be TypedArrayElementType(_ta_).
    //    b. Let _agentRecord_ be the Agent Record of the surrounding agent.
    //    c. Let _isLittleEndian_ be _agentRecord_.[[LittleEndian]].
    //    d. Let _x_ be RawBytesToNumeric(_type_, _xBytes_, _isLittleEndian_).
    //    e. Let _y_ be RawBytesToNumeric(_type_, _yBytes_, _isLittleEndian_).
    //    f. If _x_ is a Number, then
    //       i. Let _sum_ be Number::add(_x_, _y_).
    //    g. Else,
    //       i. Assert: _x_ is a BigInt.
    //       ii. Let _sum_ be BigInt::add(_x_, _y_).
    //    h. Let _sumBytes_ be NumericToRawBytes(_type_, _sum_, _isLittleEndian_).
    //    i. Assert: _sumBytes_, _xBytes_, and _yBytes_ have the same number of elements.
    //    j. Return _sumBytes_.
    // 2. Return ? AtomicReadModifyWrite(_ta_, _index_, _value_, _add_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Atomics.add"))
}
