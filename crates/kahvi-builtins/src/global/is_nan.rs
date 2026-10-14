//! isNaN ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-isnan-number

use kahvi_core::prelude::*;

/// isNaN
pub(crate) fn is_nan(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let num be ? ToNumber(number).
    let num = ops::to_number(agent, args.get_or_undefined(0))?;
    // 2. If num is NaN, return true. 3. Otherwise, return false.
    Ok(Value::Boolean(num.is_nan()))
}
