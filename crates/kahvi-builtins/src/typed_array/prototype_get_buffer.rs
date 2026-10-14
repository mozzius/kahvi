//! get %TypedArray%.prototype.buffer
//!
//! Spec: https://tc39.es/ecma262/#sec-get-%typedarray%.prototype.buffer

use kahvi_core::prelude::*;

use kahvi_core::object::typed_array::typed_array_data;

/// get TypedArray.prototype.buffer
pub(crate) fn prototype_get_buffer(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let obj be the this value.
    // 2. Perform ? RequireInternalSlot(obj, [[TypedArrayName]]).
    let obj = match this {
        Value::Object(o) if typed_array_data(agent, o).is_some() => o,
        _ => return Err(agent.throw_type_error("not a TypedArray")),
    };
    // 3-5. Return obj.[[ViewedArrayBuffer]].
    let buffer = typed_array_data(agent, obj)
        .and_then(|d| d.buffer)
        .expect("initialized");
    Ok(Value::Object(buffer))
}
