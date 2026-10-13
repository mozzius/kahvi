//! isFinite ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-isfinite-number

use kahvi_core::prelude::*;

/// isFinite
pub(crate) fn is_finite(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function is the %isFinite% intrinsic object.
    // It performs the following steps when called:
    //
    // 1. Let _number_ be ? ToNumber(_value_).
    // 2. If _number_ is a finite Number, return *true*.
    // 3. Return *false*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("isFinite"))
}
