//! Reflect.getPrototypeOf ( _target_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.getprototypeof

use kahvi_core::prelude::*;

/// Reflect.getPrototypeOf
pub(crate) fn get_prototype_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    // 2. Return ? _target_.[[GetPrototypeOf]]().
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Reflect.getPrototypeOf"))
}
