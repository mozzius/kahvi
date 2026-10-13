//! Object.isExtensible ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.isextensible

use kahvi_core::prelude::*;

/// Object.isExtensible
pub(crate) fn is_extensible(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _obj_ is not an Object, return *false*.
    // 2. Return ? IsExtensible(_obj_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.isExtensible"))
}
