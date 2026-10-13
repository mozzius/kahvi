//! Object.defineProperty ( _obj_, _key_, _attrs_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.defineproperty

use kahvi_core::prelude::*;

/// Object.defineProperty
pub(crate) fn define_property(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If O is not an Object, throw a TypeError exception.
    let Value::Object(o) = args.get_or_undefined(0) else {
        return Err(agent.throw_type_error("Object.defineProperty called on non-object"));
    };
    // 2. Let key be ? ToPropertyKey(P).
    let key = ops::to_property_key(agent, args.get_or_undefined(1))?;
    // 3. Let desc be ? ToPropertyDescriptor(Attributes).
    let desc = ops::to_property_descriptor(agent, args.get_or_undefined(2))?;
    // 4. Perform ? DefinePropertyOrThrow(O, key, desc).
    ops::define_property_or_throw(agent, o, key, desc)?;
    // 5. Return O.
    Ok(Value::Object(o))
}
