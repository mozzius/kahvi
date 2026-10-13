//! Reflect.set ( _target_, _key_, _value_ [ , _receiver_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.set

use kahvi_core::prelude::*;

/// Reflect.set
pub(crate) fn set(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    // 3. If _receiver_ is not present, then
    //    a. Set _receiver_ to _target_.
    // 4. Return ? _target_.[[Set]](_propertyKey_, _value_, _receiver_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Reflect.set"))
}
