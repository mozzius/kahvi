//! get Object.prototype.__proto__
//!
//! Spec: https://tc39.es/ecma262/#sec-get-object.prototype.__proto__

use kahvi_core::prelude::*;

/// get Object.prototype.__proto__
pub(crate) fn prototype_get_proto(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (args, new_target);
    // The value of the [[Getter]] attribute is a built-in function that requires no arguments. It performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    let obj = ops::to_object(agent, this)?;
    // 2. Return ? _obj_.[[GetPrototypeOf]]().
    match obj.internal_get_prototype_of(agent)? {
        Some(proto) => Ok(Value::Object(proto)),
        None => Ok(Value::Null),
    }
}
