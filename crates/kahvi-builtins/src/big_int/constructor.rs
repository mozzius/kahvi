//! BigInt ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-bigint-constructor-number-value

use kahvi_core::prelude::*;

/// BigInt
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is not *undefined*, throw a *TypeError* exception.
    // 2. Let _primitive_ be ? ToPrimitive(_value_, ~number~).
    // 3. If _primitive_ is a Number, return ? NumberToBigInt(_primitive_).
    // 4. Return ? ToBigInt(_primitive_).
    //
    // [subclause sec-numbertobigint: NumberToBigInt (
    // _number_: a Number,
    // ): either a normal completion containing a BigInt or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("BigInt"))
}
