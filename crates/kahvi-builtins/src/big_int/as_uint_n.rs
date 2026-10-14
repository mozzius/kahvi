//! BigInt.asUintN ( _bits_, _bigint_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-bigint.asuintn

use kahvi_core::prelude::*;

/// BigInt.asUintN
pub(crate) fn as_uint_n(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    let _ = (this, new_target);
    // 1. Set _bits_ to ? ToIndex(_bits_).
    let bits = ops::to_index(agent, args.get_or_undefined(0))? as u64;
    // 2. Set _bigint_ to ? ToBigInt(_bigint_).
    let bigint = ops::bigint::to_bigint(agent, args.get_or_undefined(1))?;
    // 3. Return ℤ(ℝ(_bigint_) modulo 2^_bits_).
    Ok(Value::BigInt(ops::bigint::as_uint_n(agent, bits, bigint)?))
}
