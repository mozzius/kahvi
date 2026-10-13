//! Number.isSafeInteger ( _number_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-number.issafeinteger

use kahvi_core::prelude::*;

/// Number.isSafeInteger
pub(crate) fn is_safe_integer(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // An integer _n_ is a “safe integer” if and only if the Number value for _n_ is not the Number value for any other integer.
    // This function performs the following steps when called:
    //
    // 1. If _number_ is an integral Number, then
    //    a. If abs(ℝ(_number_)) ≤ 2^53 - 1, return *true*.
    // 2. Return *false*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Number.isSafeInteger"))
}
