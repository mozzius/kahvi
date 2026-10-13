//! Object.keys ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.keys

use kahvi_core::prelude::*;

/// Object.keys
pub(crate) fn keys(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _coerced_ be ? ToObject(_obj_).
    // 2. Let _keyList_ be ? EnumerableOwnProperties(_coerced_, ~key~).
    // 3. Return CreateArrayFromList(_keyList_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.keys"))
}
