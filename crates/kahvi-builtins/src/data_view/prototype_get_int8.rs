//! DataView.prototype.getInt8 ( _byteOffset_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-dataview.prototype.getint8

use kahvi_core::prelude::*;

/// DataView.prototype.getInt8
pub(crate) fn prototype_get_int8(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _view_ be the *this* value.
    // 2. Return ? GetViewValue(_view_, _byteOffset_, *true*, ~int8~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("DataView.prototype.getInt8"))
}
