//! Map.prototype.clear ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.clear

use kahvi_core::prelude::*;

/// Map.prototype.clear
pub(crate) fn prototype_clear(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _map_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_map_, [[MapData]]).
    // 3. For each Record { [[Key]], [[Value]] } _entry_ of _map_.[[MapData]], do
    //    a. Set _entry_.[[Key]] to ~empty~.
    //    b. Set _entry_.[[Value]] to ~empty~.
    // 4. Return *undefined*.
    //
    // NOTE:
    // The existing [[MapData]] List is preserved because there may be existing Map Iterator objects that are suspended midway through iterating over that List.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Map.prototype.clear"))
}
