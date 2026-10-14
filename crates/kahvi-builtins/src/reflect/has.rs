//! Reflect.has ( _target_, _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.has

use kahvi_core::prelude::*;

/// Reflect.has
pub(crate) fn has(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    let Value::Object(target) = args.get_or_undefined(0) else {
        return Err(agent.throw_type_error("Reflect.has called on non-object"));
    };
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    let property_key = ops::to_property_key(agent, args.get_or_undefined(1))?;
    // 3. Return ? _target_.[[HasProperty]](_propertyKey_).
    let result = ops::has_property(agent, target, property_key)?;
    Ok(Value::Boolean(result))
}
