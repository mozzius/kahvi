//! Map.prototype.values ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.values

use kahvi_core::prelude::*;

/// Map.prototype.values
pub(crate) fn prototype_values(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _map_ be the *this* value.
    // 2. Return ? CreateMapIterator(_map_, ~value~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Map.prototype.values"))
}
