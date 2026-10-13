//! Object.getPrototypeOf ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.getprototypeof

use kahvi_core::prelude::*;

/// Object.getPrototypeOf
pub(crate) fn get_prototype_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _coerced_ be ? ToObject(_obj_).
    // 2. Return ? _coerced_.[[GetPrototypeOf]]().
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.getPrototypeOf"))
}
