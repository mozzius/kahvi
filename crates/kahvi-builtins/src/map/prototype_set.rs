//! Map.prototype.set ( _key_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.set

use crate::support::collections::{collection_key, map_table, this_map};
use kahvi_core::prelude::*;

/// Map.prototype.set
pub(crate) fn prototype_set(
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
    // 4. For each Record p of M.[[MapData]]: if SameValue(p.[[Key]], key), set p.[[Value]] to value.
    // 5-6. Otherwise append the Record { [[Key]]: key, [[Value]]: value } to M.[[MapData]].
    map_table(agent, m).set(key, args.get_or_undefined(1));
    // 4.a.ii / 7. Return M.
    Ok(Value::Object(m))
}
