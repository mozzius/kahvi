//! Reflect.get ( _target_, _key_ [ , _receiver_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.get

use kahvi_core::prelude::*;

/// Reflect.get
pub(crate) fn get(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    let Value::Object(target) = args.get_or_undefined(0) else {
        return Err(agent.throw_type_error("Reflect.get called on non-object"));
    };
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    let property_key = ops::to_property_key(agent, args.get_or_undefined(1))?;
    // 3. If _receiver_ is not present, then
    let receiver = if args.len() < 3 {
        // a. Set _receiver_ to _target_.
        Value::Object(target)
    } else {
        args.get_or_undefined(2)
    };
    // 4. Return ? _target_.[[Get]](_propertyKey_, _receiver_).
    target.internal_get(agent, property_key, receiver)
}
