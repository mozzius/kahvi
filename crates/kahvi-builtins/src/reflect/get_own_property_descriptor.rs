//! Reflect.getOwnPropertyDescriptor ( _target_, _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.getownpropertydescriptor

use kahvi_core::prelude::*;

/// Reflect.getOwnPropertyDescriptor
pub(crate) fn get_own_property_descriptor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    // 3. Let _propertyDesc_ be ? _target_.[[GetOwnProperty]](_propertyKey_).
    // 4. Return FromPropertyDescriptor(_propertyDesc_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Reflect.getOwnPropertyDescriptor"))
}
