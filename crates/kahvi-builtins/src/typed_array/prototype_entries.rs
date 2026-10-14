//! %TypedArray%.prototype.entries ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.entries

use crate::support::array_iterator::{IterationKind, create_array_iterator};
use crate::support::typed_array::validate_typed_array;
use kahvi_core::prelude::*;

/// TypedArray.prototype.entries
pub(crate) fn prototype_entries(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    let obj = this;
    // 2. Perform ? ValidateTypedArray(_obj_, ~seq-cst~).
    let ta_record = validate_typed_array(agent, obj)?;
    // 3. Return CreateArrayIterator(_obj_, ~key+value~).
    Ok(Value::Object(create_array_iterator(
        agent,
        ta_record.object,
        IterationKind::KeyValue,
    )))
}
