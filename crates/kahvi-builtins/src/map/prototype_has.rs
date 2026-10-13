//! Map.prototype.has ( _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.has

use crate::support::collections::{collection_key, map_table, this_map};
use kahvi_core::prelude::*;

/// Map.prototype.has
pub(crate) fn prototype_has(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let M be the this value.
    // 2. Perform ? RequireInternalSlot(M, [[MapData]]).
    let m = this_map(agent, this)?;
    // 3. Set key to CanonicalizeKeyedCollectionKey(key).
    let key = collection_key(agent, args.get_or_undefined(0));
    // 4-5. Return true if some p.[[Key]] is SameValue to key; otherwise false.
    Ok(Value::Boolean(map_table(agent, m).has(&key)))
}
