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
    // 1. Set _bits_ to ? ToIndex(_bits_).
    // 2. Set _bigint_ to ? ToBigInt(_bigint_).
    // 3. Return ℤ(ℝ(_bigint_) modulo 2^_bits_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("BigInt.asUintN"))
}
