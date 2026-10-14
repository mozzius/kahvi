//! Reflect.getOwnPropertyDescriptor ( _target_, _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.getownpropertydescriptor

use kahvi_core::prelude::*;

/// Reflect.getOwnPropertyDescriptor
pub(crate) fn get_own_property_descriptor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    let Value::Object(target) = args.get_or_undefined(0) else {
        return Err(agent.throw_type_error("Reflect.getOwnPropertyDescriptor called on non-object"));
    };
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    let property_key = ops::to_property_key(agent, args.get_or_undefined(1))?;
    // 3. Let _propertyDesc_ be ? _target_.[[GetOwnProperty]](_propertyKey_).
    let property_desc = target.internal_get_own_property(agent, property_key)?;
    // 4. Return FromPropertyDescriptor(_propertyDesc_).
    Ok(ops::from_property_descriptor(agent, property_desc))
}
