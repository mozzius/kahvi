//! Reflect.isExtensible ( _target_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.isextensible

use kahvi_core::prelude::*;

/// Reflect.isExtensible
pub(crate) fn is_extensible(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    // 2. Return ? _target_.[[IsExtensible]]().
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Reflect.isExtensible"))
}
