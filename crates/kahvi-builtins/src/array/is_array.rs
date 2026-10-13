//! Array.isArray ( _arg_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.isarray

use kahvi_core::prelude::*;

/// Array.isArray
pub(crate) fn is_array(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? IsArray(arg).
    Ok(Value::Boolean(ops::is_array(
        agent,
        args.get_or_undefined(0),
    )?))
}
