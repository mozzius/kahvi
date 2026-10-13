//! Array.isArray ( _arg_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.isarray

use kahvi_core::prelude::*;

/// Array.isArray
pub(crate) fn is_array(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Return ? IsArray(_arg_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.isArray"))
}
