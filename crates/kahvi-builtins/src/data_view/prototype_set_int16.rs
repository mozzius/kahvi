//! DataView.prototype.setInt16 ( _byteOffset_, _value_ [ , _littleEndian_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-dataview.prototype.setint16

use kahvi_core::prelude::*;

/// DataView.prototype.setInt16
pub(crate) fn prototype_set_int16(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _view_ be the *this* value.
    // 2. If _littleEndian_ is not present, set _littleEndian_ to *false*.
    // 3. Perform ? SetViewValue(_view_, _byteOffset_, _littleEndian_, ~int16~, _value_).
    // 4. Return *undefined*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("DataView.prototype.setInt16"))
}
