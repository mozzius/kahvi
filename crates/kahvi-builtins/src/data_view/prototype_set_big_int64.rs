//! DataView.prototype.setBigInt64 ( _byteOffset_, _value_ [ , _littleEndian_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-dataview.prototype.setbigint64

use kahvi_core::object::typed_array::ElementType;
use kahvi_core::prelude::*;

use crate::support::typed_array::set_view_value;

/// DataView.prototype.setBigInt64
pub(crate) fn prototype_set_big_int64(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = new_target;
    // This method performs the following steps when called:
    //
    // 1. Let _view_ be the *this* value.
    let view = this;
    // 2. Perform ? SetViewValue(_view_, _byteOffset_, _littleEndian_, ~bigint64~, _value_).
    set_view_value(
        agent,
        view,
        args.get_or_undefined(0),
        args.get_or_undefined(2),
        ElementType::BigInt64,
        args.get_or_undefined(1),
    )?;
    // 3. Return *undefined*.
    Ok(Value::Undefined)
}
