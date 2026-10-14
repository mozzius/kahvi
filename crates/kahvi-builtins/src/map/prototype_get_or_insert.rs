//! Map.prototype.getOrInsert ( _key_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.getorinsert

use crate::support::collections::{collection_key, map_table, this_map};
use kahvi_core::prelude::*;

/// Map.prototype.getOrInsert
pub(crate) fn prototype_get_or_insert(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let map be the this value.
    // 2. Perform ? RequireInternalSlot(map, [[MapData]]).
    let map = this_map(agent, this)?;
    // 3. Set key to CanonicalizeKeyedCollectionKey(key).
    let key = collection_key(agent, args.get_or_undefined(0));
    let value = args.get_or_undefined(1);
    // 4. For each Record { [[Key]], [[Value]] } entry of map.[[MapData]], do
    //    a. If entry.[[Key]] is not empty and SameValue(entry.[[Key]], key) is true, return entry.[[Value]].
    if let Some(existing) = map_table(agent, map).get(&key) {
        return Ok(existing);
    }
    // 5. Let entry be the Record { [[Key]]: key, [[Value]]: value }.
    // 6. Append entry to map.[[MapData]].
    map_table(agent, map).set(key, value);
    // 7. Return value.
    Ok(value)
}
