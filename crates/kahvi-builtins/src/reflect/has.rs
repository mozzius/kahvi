//! Reflect.has ( _target_, _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.has

use kahvi_core::prelude::*;

/// Reflect.has
pub(crate) fn has(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    // 3. Return ? _target_.[[HasProperty]](_propertyKey_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Reflect.has"))
}
