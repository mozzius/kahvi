//! Math.pow ( _base_, _exponent_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.pow

use kahvi_core::prelude::*;
use kahvi_core::vm;

/// Math.pow
pub(crate) fn pow(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Set base to ? ToNumber(base).
    let base = ops::to_number(agent, args.get_or_undefined(0))?;
    // 2. Set exponent to ? ToNumber(exponent).
    let exponent = ops::to_number(agent, args.get_or_undefined(1))?;
    // 3. Return Number::exponentiate(base, exponent).
    Ok(Value::Number(vm::number_exponentiate(base, exponent)))
}
