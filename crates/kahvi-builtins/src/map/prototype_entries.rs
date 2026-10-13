//! Map.prototype.entries ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.entries

use kahvi_core::prelude::*;

/// Map.prototype.entries
pub(crate) fn prototype_entries(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _map_ be the *this* value.
    // 2. Return ? CreateMapIterator(_map_, ~key+value~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Map.prototype.entries"))
}
