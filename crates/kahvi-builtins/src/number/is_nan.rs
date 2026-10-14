//! Number.isNaN ( _number_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-number.isnan

use kahvi_core::prelude::*;

/// Number.isNaN
pub(crate) fn is_nan(
    _agent: &mut Agent,
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
    let _ = (this, new_target);
    // 1. If _number_ is not a Number, return *false*.
    let number = args.get_or_undefined(0);
    let Value::Number(n) = number else {
        return Ok(Value::Boolean(false));
    };
    // 2. If _number_ is *NaN*, return *true*.
    if n.is_nan() {
        return Ok(Value::Boolean(true));
    }
    // 3. Return *false*.
    Ok(Value::Boolean(false))
}
