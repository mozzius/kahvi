//! DataView.prototype.setInt8 ( _byteOffset_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-dataview.prototype.setint8

use kahvi_core::prelude::*;

/// DataView.prototype.setInt8
pub(crate) fn prototype_set_int8(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _view_ be the *this* value.
    // 2. Perform ? SetViewValue(_view_, _byteOffset_, *true*, ~int8~, _value_).
    // 3. Return *undefined*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("DataView.prototype.setInt8"))
}
