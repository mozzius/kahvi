//! isNaN ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-isnan-number

use kahvi_core::prelude::*;

/// isNaN
pub(crate) fn is_nan(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function is the %isNaN% intrinsic object.
    // It performs the following steps when called:
    //
    // 1. Let _number_ be ? ToNumber(_value_).
    // 2. If _number_ is *NaN*, return *true*.
    // 3. Return *false*.
    //
    // NOTE:
    // A reliable way for ECMAScript code to test if a value `X` is *NaN* is an expression of the form `X !== X`. The result will be *true* if and only if `X` is *NaN*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("isNaN"))
}
