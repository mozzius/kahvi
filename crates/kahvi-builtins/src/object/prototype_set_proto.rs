//! set Object.prototype.__proto__
//!
//! Spec: https://tc39.es/ecma262/#sec-set-object.prototype.__proto__

use kahvi_core::prelude::*;

/// set Object.prototype.__proto__
pub(crate) fn prototype_set_proto(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The value of the [[Setter]] attribute is a built-in function that takes an argument _proto_. It performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    // 3. If _proto_ is not an Object and _proto_ is not *null*, return *undefined*.
    // 4. If _thisValue_ is not an Object, return *undefined*.
    // 5. Let _status_ be ? _thisValue_.[[SetPrototypeOf]](_proto_).
    // 6. If _status_ is *false*, throw a *TypeError* exception.
    // 7. Return *undefined*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("set Object.prototype.__proto__"))
}
