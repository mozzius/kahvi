//! Map.prototype.getOrInsertComputed ( _key_, _callback_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.getorinsertcomputed

use crate::support::collections::{collection_key, map_table, this_map};
use kahvi_core::prelude::*;

/// Map.prototype.getOrInsertComputed
pub(crate) fn prototype_get_or_insert_computed(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let map be the this value.
    // 2. Perform ? RequireInternalSlot(map, [[MapData]]).
    let map = this_map(agent, this)?;
    let key_arg = args.get_or_undefined(0);
    let callback = args.get_or_undefined(1);
    // 3. If IsCallable(callback) is false, throw a TypeError exception.
    if !ops::is_callable(agent, callback) {
        return Err(
            agent.throw_type_error("Map.prototype.getOrInsertComputed callback is not a function")
        );
    }
    // 4. Set key to CanonicalizeKeyedCollectionKey(key).
    let key = match key_arg {
        // CanonicalizeKeyedCollectionKey: -0 becomes +0.
        Value::Number(n) => Value::Number(n + 0.0),
        k => k,
    };
    // 5. For each Record entry of map.[[MapData]], do
    //    a. If entry.[[Key]] is not ~empty~ and SameValue(entry.[[Key]], key) is true, return entry.[[Value]].
    // (The table is hash-indexed on SameValueZero, which agrees with SameValue
    // once -0 has been canonicalized and all NaNs are one key.)
    let table_key = collection_key(agent, key);
    if let Some(existing) = map_table(agent, map).get(&table_key) {
        return Ok(existing);
    }
    // The callback may collect garbage: map, callback and key stay rooted.
    agent.with_roots(&[Value::Object(map), callback, key], |agent| {
        // 6. Let value be ? Call(callback, undefined, « key »).
        let value = ops::call_rooted(agent, callback, Value::Undefined, &[key])?;
        // 7. NOTE: The Map may have been modified during execution of callback.
        // 8. For each Record entry of map.[[MapData]], do
        //    a. If entry.[[Key]] is not ~empty~ and SameValue(entry.[[Key]], key) is true, then
        //       i. Set entry.[[Value]] to value.
        //       ii. Return value.
        // 9. Let entry be the Record { [[Key]]: key, [[Value]]: value }.
        // 10. Append entry to map.[[MapData]].
        // (OrderedTable::set updates the existing entry or appends a new one.)
        let table_key = collection_key(agent, key);
        map_table(agent, map).set(table_key, value);
        // 11. Return value.
        Ok(value)
    })
}
