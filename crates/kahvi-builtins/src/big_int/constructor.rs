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
    let _ = this;
    let value = args.get_or_undefined(0);
    // 1. If NewTarget is not *undefined*, throw a *TypeError* exception.
    if new_target.is_some() {
        return Err(agent.throw_type_error("BigInt is not a constructor"));
    }
    // 2. Let _primitive_ be ? ToPrimitive(_value_, ~number~).
    let primitive = ops::to_primitive(agent, value, ops::PreferredType::Number)?;
    // 3. If _primitive_ is a Number, return ? NumberToBigInt(_primitive_).
    if let Value::Number(n) = primitive {
        return Ok(Value::BigInt(ops::bigint::number_to_bigint(agent, n)?));
    }
    // 4. Return ? ToBigInt(_primitive_).
    Ok(Value::BigInt(ops::bigint::to_bigint(agent, primitive)?))
    //
    // [subclause sec-numbertobigint: NumberToBigInt (
    // _number_: a Number,
    // ): either a normal completion containing a BigInt or a throw completion]
}
