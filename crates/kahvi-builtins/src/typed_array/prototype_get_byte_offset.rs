//! get %TypedArray%.prototype.byteOffset
//!
//! Spec: https://tc39.es/ecma262/#sec-get-%typedarray%.prototype.byteoffset

use kahvi_core::prelude::*;

use kahvi_core::object::typed_array::{
    is_typed_array_out_of_bounds, make_ta_record, typed_array_data,
};

/// get TypedArray.prototype.byteOffset
pub(crate) fn prototype_get_byte_offset(
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
    // 5. If IsTypedArrayOutOfBounds(taRecord) is true, return +0𝔽.
    if is_typed_array_out_of_bounds(agent, rec) {
        return Ok(Value::Number(0.0));
    }
    // 6-7. Return 𝔽(obj.[[ByteOffset]]).
    let offset = typed_array_data(agent, obj)
        .expect("a TypedArray")
        .byte_offset;
    Ok(Value::Number(offset as f64))
}
