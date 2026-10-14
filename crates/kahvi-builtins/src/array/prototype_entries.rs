//! Array.prototype.entries ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.entries

use crate::support::array_iterator::{IterationKind, create_array_iterator};
use kahvi_core::prelude::*;

/// Array.prototype.entries
pub(crate) fn prototype_entries(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let O be ? ToObject(this value).
    let o = ops::to_object(agent, this)?;
    // 2. Return CreateArrayIterator(O, key+value).
    Ok(Value::Object(create_array_iterator(
        agent,
        o,
        IterationKind::KeyValue,
    )))
}
