//! Atomics.sub ( _ta_, _index_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.sub

use crate::support::atomics::{LITTLE_ENDIAN, atomic_read_modify_write, element_type};
use kahvi_core::object::typed_array::{numeric_to_raw_bytes, raw_bytes_to_numeric};
use kahvi_core::prelude::*;

/// Atomics.sub
pub(crate) fn sub(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (this, new_target);
    let (ta, index, value) = (
        args.get_or_undefined(0),
        args.get_or_undefined(1),
        args.get_or_undefined(2),
    );
    // 1. Let _subtract_ be a new read-modify-write modification function with parameters (_xBytes_, _yBytes_) that captures _ta_ and performs the following steps atomically when called:
    let mut subtract = |agent: &mut Agent, x_bytes: &[u8], y_bytes: &[u8]| {
        //    a. Let _type_ be TypedArrayElementType(_ta_).
        let Value::Object(ta) = ta else {
            unreachable!("validated by AtomicReadModifyWrite")
        };
        let ty = element_type(agent, ta);
        //    b-c. Let _isLittleEndian_ be the surrounding agent's [[LittleEndian]].
        //    d. Let _x_ be RawBytesToNumeric(_type_, _xBytes_, _isLittleEndian_).
        let x = raw_bytes_to_numeric(agent, ty, x_bytes, LITTLE_ENDIAN);
        //    e. Let _y_ be RawBytesToNumeric(_type_, _yBytes_, _isLittleEndian_).
        let y = raw_bytes_to_numeric(agent, ty, y_bytes, LITTLE_ENDIAN);
        let difference = match (x, y) {
            //    f. If _x_ is a Number, then i. Let _difference_ be Number::subtract(_x_, _y_).
            (Value::Number(x), Value::Number(y)) => Value::Number(x - y),
            //    g. Else, ii. Let _difference_ be BigInt::subtract(_x_, _y_).
            (Value::BigInt(x), Value::BigInt(y)) => {
                let difference = agent.bigint(x).sub(agent.bigint(y));
                Value::BigInt(agent.new_bigint(difference))
            }
            _ => unreachable!("RawBytesToNumeric gives one content type"),
        };
        //    h. Let _differenceBytes_ be NumericToRawBytes(_type_, _difference_, _isLittleEndian_).
        //    i. Assert: _differenceBytes_, _xBytes_, and _yBytes_ have the same number of elements.
        //    j. Return _differenceBytes_.
        numeric_to_raw_bytes(agent, ty, difference, LITTLE_ENDIAN)
    };
    // 2. Return ? AtomicReadModifyWrite(_ta_, _index_, _value_, _subtract_).
    atomic_read_modify_write(agent, ta, index, value, &mut subtract)
}
