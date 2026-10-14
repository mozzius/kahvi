//! get DataView.prototype.buffer
//!
//! Spec: https://tc39.es/ecma262/#sec-get-dataview.prototype.buffer

use kahvi_core::prelude::*;

use crate::support::typed_array::require_data_view;

/// get DataView.prototype.buffer
pub(crate) fn prototype_get_buffer(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // `DataView.prototype.buffer` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[DataView]]).
    let (_obj, d) = require_data_view(agent, this)?;
    // 3. Assert: _obj_ has a [[ViewedArrayBuffer]] internal slot.
    // 4. Let _buffer_ be _obj_.[[ViewedArrayBuffer]].
    // 5. Return _buffer_.
    Ok(Value::Object(d.buffer))
}
