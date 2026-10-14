//! set Object.prototype.__proto__
//!
//! Spec: https://tc39.es/ecma262/#sec-set-object.prototype.__proto__

use kahvi_core::prelude::*;

/// set Object.prototype.__proto__
pub(crate) fn prototype_set_proto(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
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
    // 1. Let _thisValue_ be the *this* value.
    let this_value = this;
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    ops::require_object_coercible(agent, this_value)?;
    // 3. If _proto_ is not an Object and _proto_ is not *null*, return *undefined*.
    let proto = match args.get_or_undefined(0) {
        Value::Object(p) => Some(p),
        Value::Null => None,
        _ => return Ok(Value::Undefined),
    };
    // 4. If _thisValue_ is not an Object, return *undefined*.
    let Value::Object(o) = this_value else {
        return Ok(Value::Undefined);
    };
    // 5. Let _status_ be ? _thisValue_.[[SetPrototypeOf]](_proto_).
    let status = o.internal_set_prototype_of(agent, proto)?;
    // 6. If _status_ is *false*, throw a *TypeError* exception.
    if !status {
        return Err(agent.throw_type_error("Object.prototype.__proto__ setter failed"));
    }
    // 7. Return *undefined*.
    Ok(Value::Undefined)
}
