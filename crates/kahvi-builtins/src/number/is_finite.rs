//! Number.isFinite ( _number_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-number.isfinite

use kahvi_core::prelude::*;

/// Number.isFinite
pub(crate) fn is_finite(
    _agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _number_ is a finite Number, return *true*.
    if let Value::Number(number) = args.get_or_undefined(0) {
        if number.is_finite() {
            return Ok(Value::Boolean(true));
        }
    }
    // 2. Return *false*.
    Ok(Value::Boolean(false))
}
