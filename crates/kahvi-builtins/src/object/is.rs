//! Object.is ( _value1_, _value2_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.is

use kahvi_core::prelude::*;

/// Object.is
pub(crate) fn is(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    let _ = (this, new_target);
    // This function performs the following steps when called:
    //
    // 1. Return SameValue(_value1_, _value2_).
    Ok(Value::Boolean(ops::same_value(
        agent,
        args.get_or_undefined(0),
        args.get_or_undefined(1),
    )))
}
