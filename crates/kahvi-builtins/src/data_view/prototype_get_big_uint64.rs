//! DataView.prototype.getBigUint64 ( _byteOffset_ [ , _littleEndian_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-dataview.prototype.getbiguint64

use kahvi_core::prelude::*;

/// DataView.prototype.getBigUint64
pub(crate) fn prototype_get_big_uint64(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _view_ be the *this* value.
    // 2. Return ? GetViewValue(_view_, _byteOffset_, _littleEndian_, ~biguint64~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("DataView.prototype.getBigUint64"))
}
