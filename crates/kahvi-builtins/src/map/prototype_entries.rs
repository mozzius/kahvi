//! Map.prototype.entries ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.entries

use crate::support::collections::{CollectionIterKind, create_collection_iterator, this_map};
use kahvi_core::prelude::*;

/// Map.prototype.entries
pub(crate) fn prototype_entries(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let M be the this value.
    // 2. Return ? CreateMapIterator(M, key+value).
    // CreateMapIterator 1. Perform ? RequireInternalSlot(map, [[MapData]]).
    let m = this_map(agent, this)?;
    Ok(Value::Object(create_collection_iterator(
        agent,
        m,
        CollectionIterKind::KeyValue,
        false,
    )))
}
