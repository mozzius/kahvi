//! Number.isInteger ( _number_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-number.isinteger

use kahvi_core::prelude::*;

/// Number.isInteger
pub(crate) fn is_integer(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (agent, this, new_target);
    // This function performs the following steps when called:
    //
    // 1. If _number_ is an integral Number, return *true*.
    let number = args.get_or_undefined(0);
    if ops::is_integral_number(number) {
        return Ok(Value::Boolean(true));
    }
    // 2. Return *false*.
    Ok(Value::Boolean(false))
}
