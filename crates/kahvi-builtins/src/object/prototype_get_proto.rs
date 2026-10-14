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
    // The value of the [[Getter]] attribute is a built-in function that requires no arguments. It performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Return ? _obj_.[[GetPrototypeOf]]().
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get Object.prototype.__proto__"))
}
