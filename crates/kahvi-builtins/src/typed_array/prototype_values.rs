//! %TypedArray%.prototype.values ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.values

use crate::support::array_iterator::{IterationKind, create_array_iterator};
use crate::support::typed_array::validate_typed_array;
use kahvi_core::prelude::*;

/// TypedArray.prototype.values
pub(crate) fn prototype_values(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let obj be the this value.
    let obj = this;
    // 2. Perform ? ValidateTypedArray(obj, seq-cst).
    validate_typed_array(agent, obj)?;
    // 3. Return CreateArrayIterator(obj, value).
    let Value::Object(obj) = obj else {
        // ValidateTypedArray has already rejected every non-object value.
        return Err(agent.throw_type_error("not a TypedArray"));
    };
    Ok(Value::Object(create_array_iterator(
        agent,
        obj,
        IterationKind::Value,
    )))
}
