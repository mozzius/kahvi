//! Object.getOwnPropertyDescriptor ( _obj_, _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.getownpropertydescriptor

use kahvi_core::prelude::*;

/// Object.getOwnPropertyDescriptor
pub(crate) fn get_own_property_descriptor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let obj be ? ToObject(O).
    let obj = ops::to_object(agent, args.get_or_undefined(0))?;
    // 2. Let key be ? ToPropertyKey(P).
    let key = ops::to_property_key(agent, args.get_or_undefined(1))?;
    // 3. Let desc be ? obj.[[GetOwnProperty]](key).
    let desc = obj.internal_get_own_property(agent, key)?;
    // 4. Return FromPropertyDescriptor(desc).
    Ok(ops::from_property_descriptor(agent, desc))
}
