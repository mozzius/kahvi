//! get %TypedArray%.prototype.byteLength
//!
//! Spec: https://tc39.es/ecma262/#sec-get-%typedarray%.prototype.bytelength

use kahvi_core::prelude::*;

use kahvi_core::object::typed_array::{make_ta_record, typed_array_byte_length, typed_array_data};

/// get TypedArray.prototype.byteLength
pub(crate) fn prototype_get_byte_length(
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
    // 4. Let taRecord be MakeTypedArrayWithBufferWitnessRecord(obj, seq-cst).
    let rec = make_ta_record(agent, obj);
    // 5-7. Return 𝔽(TypedArrayByteLength(taRecord)), which is 0 when out of bounds.
    Ok(Value::Number(typed_array_byte_length(agent, rec) as f64))
}
