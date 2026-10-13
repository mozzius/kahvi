//! Reflect.defineProperty ( _target_, _key_, _attrs_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.defineproperty

use kahvi_core::prelude::*;

/// Reflect.defineProperty
pub(crate) fn define_property(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    // 3. Let _propertyDesc_ be ? ToPropertyDescriptor(_attrs_).
    // 4. Return ? _target_.[[DefineOwnProperty]](_propertyKey_, _propertyDesc_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Reflect.defineProperty"))
}
