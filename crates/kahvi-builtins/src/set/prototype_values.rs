//! Set.prototype.values ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-set.prototype.values

use crate::support::collections::{CollectionIterKind, create_collection_iterator, this_set};
use kahvi_core::prelude::*;

/// Set.prototype.values
pub(crate) fn prototype_values(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _set_ be the *this* value.
    // 2. Return ? CreateSetIterator(_set_, ~value~).
    let _ = (args, new_target);
    // CreateSetIterator ( _set_, _kind_ )
    // 1. Perform ? RequireInternalSlot(_set_, [[SetData]]).
    let set = this_set(agent, this)?;
    // 2. Let _closure_ be a new Abstract Closure ... (captured by the iterator state, see CollectionIterator)
    // 3. Return CreateIteratorFromClosure(_closure_, "%SetIteratorPrototype%", %SetIteratorPrototype%).
    Ok(Value::Object(create_collection_iterator(
        agent,
        set,
        CollectionIterKind::Value,
        true,
    )))
}
