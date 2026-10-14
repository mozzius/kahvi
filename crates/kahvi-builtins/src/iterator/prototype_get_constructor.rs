//! get Iterator.prototype.constructor
//!
//! Spec: https://tc39.es/ecma262/#sec-get-iterator.prototype.constructor

use kahvi_core::prelude::*;

/// get Iterator.prototype.constructor
pub(crate) fn prototype_get_constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The value of the [[Getter]] attribute is a built-in function that requires no arguments. It performs the following steps when called:
    //
    let _ = (this, args, new_target);
    // 1. Return %Iterator%.
    Ok(Value::Object(agent.intrinsic("%Iterator%")))
}
