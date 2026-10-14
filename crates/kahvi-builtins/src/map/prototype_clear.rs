//! Map.prototype.clear ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.clear

use crate::support::collections::{map_table, this_map};
use kahvi_core::prelude::*;

/// Map.prototype.clear
pub(crate) fn prototype_clear(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let map be the this value.
    // 2. Perform ? RequireInternalSlot(map, [[MapData]]).
    let map = this_map(agent, this)?;
    // 3. For each Record { [[Key]], [[Value]] } entry of map.[[MapData]], do
    //    a. Set entry.[[Key]] to ~empty~.
    //    b. Set entry.[[Value]] to ~empty~.
    // NOTE: The existing [[MapData]] List is preserved because there may be
    // existing Map Iterator objects that are suspended midway through iterating
    // over that List. OrderedTable::clear empties each entry in place.
    map_table(agent, map).clear();
    // 4. Return undefined.
    Ok(Value::Undefined)
}
