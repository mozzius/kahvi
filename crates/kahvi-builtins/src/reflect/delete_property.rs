//! Reflect.deleteProperty ( _target_, _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.deleteproperty

use kahvi_core::prelude::*;

/// Reflect.deleteProperty
pub(crate) fn delete_property(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    // 3. Return ? _target_.[[Delete]](_propertyKey_).
    let _ = (this, new_target);
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    let Value::Object(target) = args.get_or_undefined(0) else {
        return Err(agent.throw_type_error("Reflect.deleteProperty called on non-object"));
    };
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    let property_key = ops::to_property_key(agent, args.get_or_undefined(1))?;
    // 3. Return ? _target_.[[Delete]](_propertyKey_).
    let deleted = target.internal_delete(agent, property_key)?;
    Ok(Value::Boolean(deleted))
}
