//! Number.isNaN ( _number_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-number.isnan

use kahvi_core::prelude::*;

/// Number.isNaN
pub(crate) fn is_nan(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _number_ is not a Number, return *false*.
    // 2. If _number_ is *NaN*, return *true*.
    // 3. Return *false*.
    //
    // NOTE:
    // This function differs from the global isNaN function ([sec-isnan-number]) in that it does not convert its argument to a Number before determining whether it is *NaN*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Number.isNaN"))
}
