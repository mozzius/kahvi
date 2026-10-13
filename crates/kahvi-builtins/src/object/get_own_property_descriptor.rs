//! Object.getOwnPropertyDescriptor ( _obj_, _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.getownpropertydescriptor

use kahvi_core::prelude::*;

/// Object.getOwnPropertyDescriptor
pub(crate) fn get_own_property_descriptor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _coerced_ be ? ToObject(_obj_).
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    // 3. Let _propertyDesc_ be ? _coerced_.[[GetOwnProperty]](_propertyKey_).
    // 4. Return FromPropertyDescriptor(_propertyDesc_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.getOwnPropertyDescriptor"))
}
