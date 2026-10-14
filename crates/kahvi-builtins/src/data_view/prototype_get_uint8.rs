//! DataView.prototype.getUint8 ( _byteOffset_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-dataview.prototype.getuint8

use kahvi_core::prelude::*;

/// DataView.prototype.getUint8
pub(crate) fn prototype_get_uint8(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _view_ be the *this* value.
    // 2. Return ? GetViewValue(_view_, _byteOffset_, *true*, ~uint8~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("DataView.prototype.getUint8"))
}
