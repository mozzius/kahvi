//! Reflect.setPrototypeOf ( _target_, _proto_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.setprototypeof

use kahvi_core::prelude::*;

/// Reflect.setPrototypeOf
pub(crate) fn set_prototype_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    // 2. If _proto_ is not an Object and _proto_ is not *null*, throw a *TypeError* exception.
    // 3. Return ? _target_.[[SetPrototypeOf]](_proto_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Reflect.setPrototypeOf"))
}
