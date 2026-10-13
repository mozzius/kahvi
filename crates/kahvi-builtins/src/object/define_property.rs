//! Object.defineProperty ( _obj_, _key_, _attrs_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.defineproperty

use kahvi_core::prelude::*;

/// Object.defineProperty
pub(crate) fn define_property(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function adds an own property and/or updates the attributes of an existing own property of an object.
    // It performs the following steps when called:
    //
    // 1. If _obj_ is not an Object, throw a *TypeError* exception.
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    // 3. Let _propertyDesc_ be ? ToPropertyDescriptor(_attrs_).
    // 4. Perform ? DefinePropertyOrThrow(_obj_, _propertyKey_, _propertyDesc_).
    // 5. Return _obj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.defineProperty"))
}
