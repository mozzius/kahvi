//! ArrayBuffer.isView ( _arg_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-arraybuffer.isview

use kahvi_core::prelude::*;

use kahvi_core::object::typed_array::{data_view_data, typed_array_data};

/// ArrayBuffer.isView
pub(crate) fn is_view(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If arg is not an Object, return false.
    let Value::Object(o) = args.get_or_undefined(0) else {
        return Ok(Value::Boolean(false));
    };
    // 2. If arg has a [[ViewedArrayBuffer]] internal slot, return true.
    // 3. Return false.
    Ok(Value::Boolean(
        typed_array_data(agent, o).is_some() || data_view_data(agent, o).is_some(),
    ))
}
