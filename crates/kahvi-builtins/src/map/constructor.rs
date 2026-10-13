//! Map ( [ _iterable_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-map-iterable

use crate::support::collections::{MapData, add_entries_from_iterable, create_collection};
use kahvi_core::prelude::*;

/// Map
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If NewTarget is undefined, throw a TypeError exception.
    let Some(new_target) = new_target else {
        return Err(agent.throw_type_error("Constructor Map requires 'new'"));
    };
    // 2. Let map be ? OrdinaryCreateFromConstructor(NewTarget, "%Map.prototype%", « [[MapData]] »).
    // 3. Set map.[[MapData]] to a new empty List.
    let map = create_collection(agent, new_target, "%Map.prototype%", MapData::default())?;
    // 4. If iterable is either undefined or null, return map.
    let iterable = args.get_or_undefined(0);
    if iterable.is_nullish() {
        return Ok(Value::Object(map));
    }
    // 5. Let adder be ? Get(map, "set").
    let set_key = PropertyKey::String(agent.names.set);
    let adder = ops::get(agent, map, set_key)?;
    // 6. If IsCallable(adder) is false, throw a TypeError exception.
    if !ops::is_callable(agent, adder) {
        return Err(agent.throw_type_error("Map.prototype.set is not a function"));
    }
    // 7. Return ? AddEntriesFromIterable(map, iterable, adder).
    add_entries_from_iterable(agent, map, iterable, adder)
}
