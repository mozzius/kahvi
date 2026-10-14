//! DataView.prototype.setFloat16 ( _byteOffset_, _value_ [ , _littleEndian_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-dataview.prototype.setfloat16

use kahvi_core::object::typed_array::ElementType;
use kahvi_core::prelude::*;

use crate::support::typed_array::set_view_value;

/// DataView.prototype.setFloat16
pub(crate) fn prototype_set_float16(
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
    // 2. If _littleEndian_ is not present, set _littleEndian_ to *false*.
    let little_endian = if args.len() > 2 {
        args.get_or_undefined(2)
    } else {
        Value::Boolean(false)
    };
    // 3. Perform ? SetViewValue(_view_, _byteOffset_, _littleEndian_, ~float16~, _value_).
    set_view_value(
        agent,
        view,
        args.get_or_undefined(0),
        little_endian,
        ElementType::Float16,
        args.get_or_undefined(1),
    )?;
    // 4. Return *undefined*.
    Ok(Value::Undefined)
}
