//! Array.prototype.entries ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.entries

use kahvi_core::prelude::*;

/// Array.prototype.entries
pub(crate) fn prototype_entries(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Return CreateArrayIterator(_obj_, ~key+value~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.entries"))
}
