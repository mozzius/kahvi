//! Set.prototype.entries ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.entries

use kahvi_core::prelude::*;

/// Set.prototype.entries
pub(crate) fn prototype_entries(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _set_ be the *this* value.
    // 2. Return ? CreateSetIterator(_set_, ~key+value~).
    //
    // NOTE:
    // For iteration purposes, a Set appears similar to a Map where each entry has the same value for its key and value.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Set.prototype.entries"))
}
