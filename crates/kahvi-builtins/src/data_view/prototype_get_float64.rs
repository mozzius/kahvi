//! DataView.prototype.getFloat64 ( _byteOffset_ [ , _littleEndian_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-dataview.prototype.getfloat64

use kahvi_core::prelude::*;

/// DataView.prototype.getFloat64
pub(crate) fn prototype_get_float64(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _view_ be the *this* value.
    // 2. If _littleEndian_ is not present, set _littleEndian_ to *false*.
    // 3. Return ? GetViewValue(_view_, _byteOffset_, _littleEndian_, ~float64~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("DataView.prototype.getFloat64"))
}
