//! Object.setPrototypeOf ( _obj_, _proto_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.setprototypeof

use kahvi_core::prelude::*;

/// Object.setPrototypeOf
pub(crate) fn set_prototype_of(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let obj = args.get_or_undefined(0);
    let proto = args.get_or_undefined(1);
    // 1. Perform ? RequireObjectCoercible(_obj_).
    ops::require_object_coercible(agent, obj.clone())?;
    // 2. If _proto_ is not an Object and _proto_ is not *null*, throw a *TypeError* exception.
    let proto = match proto {
        Value::Object(p) => Some(p.clone()),
        Value::Null => None,
        _ => {
            return Err(agent.throw_type_error("Object prototype may only be an Object or null"));
        }
    };
    // 3. If _obj_ is not an Object, return _obj_.
    let Value::Object(o) = obj else {
        return Ok(obj.clone());
    };
    // 4. Let _status_ be ? _obj_.[[SetPrototypeOf]](_proto_).
    let status = o.internal_set_prototype_of(agent, proto)?;
    // 5. If _status_ is *false*, throw a *TypeError* exception.
    if !status {
        return Err(agent.throw_type_error("Object.setPrototypeOf failed to set the prototype"));
    }
    // 6. Return _obj_.
    Ok(obj.clone())
}
