//! Math.pow ( _base_, _exponent_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.pow

use kahvi_core::prelude::*;

/// Math.pow
pub(crate) fn pow(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Set _base_ to ? ToNumber(_base_).
    // 2. Set _exponent_ to ? ToNumber(_exponent_).
    // 3. Return Number::exponentiate(_base_, _exponent_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Math.pow"))
}
