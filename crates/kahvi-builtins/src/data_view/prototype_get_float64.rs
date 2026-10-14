//! DataView.prototype.getFloat64 ( _byteOffset_ [ , _littleEndian_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-dataview.prototype.getfloat64

use kahvi_core::object::typed_array::ElementType;
use kahvi_core::prelude::*;

use crate::support::typed_array::get_view_value;

/// DataView.prototype.getFloat64
pub(crate) fn prototype_get_float64(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _view_ be the *this* value.
    let view = this;
    // 2. If _littleEndian_ is not present, set _littleEndian_ to *false*.
    //    (An absent argument is `undefined`, and ToBoolean(undefined) is *false*,
    //    so GetViewValue's ToBoolean step yields the same result.)
    let little_endian = args.get_or_undefined(1);
    // 3. Return ? GetViewValue(_view_, _byteOffset_, _littleEndian_, ~float64~).
    get_view_value(
        agent,
        view,
        args.get_or_undefined(0),
        little_endian,
        ElementType::Float64,
    )
}
