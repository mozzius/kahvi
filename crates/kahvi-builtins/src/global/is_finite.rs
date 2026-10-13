//! isFinite ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-isfinite-number

use kahvi_core::prelude::*;

/// isFinite
pub(crate) fn is_finite(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let num be ? ToNumber(number).
    let num = ops::to_number(agent, args.get_or_undefined(0))?;
    // 2. If num is not finite, return false. 3. Otherwise, return true.
    Ok(Value::Boolean(num.is_finite()))
}
