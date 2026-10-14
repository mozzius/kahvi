//! DataView.prototype.getBigUint64 ( _byteOffset_ [ , _littleEndian_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-dataview.prototype.getbiguint64

use kahvi_core::object::typed_array::ElementType;
use kahvi_core::prelude::*;

use crate::support::typed_array::get_view_value;

/// DataView.prototype.getBigUint64
pub(crate) fn prototype_get_big_uint64(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _view_ be the *this* value.
    let view = this;
    // 2. Return ? GetViewValue(_view_, _byteOffset_, _littleEndian_, ~biguint64~).
    get_view_value(
        agent,
        view,
        args.get_or_undefined(0),
        args.get_or_undefined(1),
        ElementType::BigUint64,
    )
}
