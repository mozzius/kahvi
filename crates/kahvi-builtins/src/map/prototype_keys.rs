//! Map.prototype.keys ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.keys

use kahvi_core::prelude::*;

/// Map.prototype.keys
pub(crate) fn prototype_keys(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _map_ be the *this* value.
    // 2. Return ? CreateMapIterator(_map_, ~key~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Map.prototype.keys"))
}
