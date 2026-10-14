//! Map.prototype.get ( _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.get

use crate::support::collections::{collection_key, map_table, this_map};
use kahvi_core::prelude::*;

/// Map.prototype.get
pub(crate) fn prototype_get(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let M be the this value.
    // 2. Perform ? RequireInternalSlot(M, [[MapData]]).
    let m = this_map(agent, this)?;
    // 3. Set key to CanonicalizeKeyedCollectionKey(key).
    // 4. For each Record { [[Key]], [[Value]] } p of M.[[MapData]], do
    //    a. If p.[[Key]] is not empty and SameValue(p.[[Key]], key) is true, return p.[[Value]].
    // 5. Return undefined.
    let key = collection_key(agent, args.get_or_undefined(0));
    Ok(map_table(agent, m).get(&key).unwrap_or(Value::Undefined))
}
