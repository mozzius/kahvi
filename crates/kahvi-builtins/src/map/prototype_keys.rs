//! Map.prototype.keys ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.keys

use crate::support::collections::{CollectionIterKind, create_collection_iterator, this_map};
use kahvi_core::prelude::*;

/// Map.prototype.keys
pub(crate) fn prototype_keys(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let map be the this value.
    // 2. Return ? CreateMapIterator(map, key).
    // CreateMapIterator 1. Perform ? RequireInternalSlot(map, [[MapData]]).
    let map = this_map(agent, this)?;
    Ok(Value::Object(create_collection_iterator(
        agent,
        map,
        CollectionIterKind::Key,
        false,
    )))
}
