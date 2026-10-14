//! %TypedArray%.prototype.keys ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.keys

use crate::support::array_iterator::{IterationKind, create_array_iterator};
use crate::support::typed_array::validate_typed_array;
use kahvi_core::prelude::*;

/// TypedArray.prototype.keys
pub(crate) fn prototype_keys(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let obj be the this value.
    // 2. Perform ? ValidateTypedArray(obj, seq-cst).
    let rec = validate_typed_array(agent, this)?;
    // 3. Return CreateArrayIterator(obj, key).
    Ok(Value::Object(create_array_iterator(
        agent,
        rec.object,
        IterationKind::Key,
    )))
}
