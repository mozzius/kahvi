//! BigInt.asIntN ( _bits_, _bigint_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-bigint.asintn

use kahvi_core::prelude::*;

/// BigInt.asIntN
pub(crate) fn as_int_n(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Set _bits_ to ? ToIndex(_bits_).
    // 2. Set _bigint_ to ? ToBigInt(_bigint_).
    // 3. Let _remainder_ be ℝ(_bigint_) modulo 2^_bits_.
    // 4. If _remainder_ ≥ 2^_bits_ - 1, return ℤ(_remainder_ - 2^_bits_).
    // 5. Return ℤ(_remainder_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("BigInt.asIntN"))
}
