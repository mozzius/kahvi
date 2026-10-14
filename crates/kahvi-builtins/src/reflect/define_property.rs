//! Reflect.defineProperty ( _target_, _key_, _attrs_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.defineproperty

use kahvi_core::prelude::*;

/// Reflect.defineProperty
pub(crate) fn define_property(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    let Value::Object(target) = args.get_or_undefined(0) else {
        return Err(agent.throw_type_error("Reflect.defineProperty called on non-object"));
    };
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    let property_key = ops::to_property_key(agent, args.get_or_undefined(1))?;
    // 3. Let _propertyDesc_ be ? ToPropertyDescriptor(_attrs_).
    let property_desc = ops::to_property_descriptor(agent, args.get_or_undefined(2))?;
    // 4. Return ? _target_.[[DefineOwnProperty]](_propertyKey_, _propertyDesc_).
    let result = target.internal_define_own_property(agent, property_key, property_desc)?;
    Ok(Value::Boolean(result))
}
