//! Map.prototype.values ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype.values

use crate::support::collections::{CollectionIterKind, create_collection_iterator, this_map};
use kahvi_core::prelude::*;

/// Map.prototype.values
pub(crate) fn prototype_values(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (args, new_target);
    // 1. Let _map_ be the *this* value.
    // 2. Return ? CreateMapIterator(_map_, ~value~).
    // CreateMapIterator: 1. Perform ? RequireInternalSlot(_map_, [[MapData]]).
    let map = this_map(agent, this)?;
    // 2-3. The closure and CreateIteratorFromClosure(_closure_, *"%MapIteratorPrototype%"*,
    //      %MapIteratorPrototype%) are the iterator state kept by create_collection_iterator:
    //      it walks [[MapData]] by index, so entries added during iteration are visited
    //      and deleted entries are skipped.
    let iterator = create_collection_iterator(agent, map, CollectionIterKind::Value, false);
    Ok(Value::Object(iterator))
}
