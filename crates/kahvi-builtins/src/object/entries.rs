//! Object.entries ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.entries

use kahvi_core::prelude::*;

/// Object.entries
pub(crate) fn entries(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _coerced_ be ? ToObject(_obj_).
    // 2. Let _entryList_ be ? EnumerableOwnProperties(_coerced_, ~key+value~).
    // 3. Return CreateArrayFromList(_entryList_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.entries"))
}
