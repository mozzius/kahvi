//! Atomics.add ( _ta_, _index_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.add

use crate::support::atomics::{LITTLE_ENDIAN, atomic_read_modify_write, element_type};
use kahvi_core::object::typed_array::{numeric_to_raw_bytes, raw_bytes_to_numeric};
use kahvi_core::prelude::*;

/// Atomics.add
pub(crate) fn add(
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
    // 1. Let add be a new read-modify-write modification function with parameters (xBytes, yBytes) that captures ta and performs the following steps atomically when called:
    let mut add = |agent: &mut Agent, x_bytes: &[u8], y_bytes: &[u8]| {
        //    a. Let type be TypedArrayElementType(ta).
        let Value::Object(ta) = ta else {
            unreachable!("validated by AtomicReadModifyWrite")
        };
        let ty = element_type(agent, ta);
        //    b-c. Let isLittleEndian be the surrounding agent's [[LittleEndian]].
        //    d. Let x be RawBytesToNumeric(type, xBytes, isLittleEndian).
        let x = raw_bytes_to_numeric(agent, ty, x_bytes, LITTLE_ENDIAN);
        //    e. Let y be RawBytesToNumeric(type, yBytes, isLittleEndian).
        let y = raw_bytes_to_numeric(agent, ty, y_bytes, LITTLE_ENDIAN);
        let sum = match (x, y) {
            //    f. If x is a Number, then i. Let sum be Number::add(x, y).
            (Value::Number(x), Value::Number(y)) => Value::Number(x + y),
            //    g. Else, ii. Let sum be BigInt::add(x, y).
            (Value::BigInt(x), Value::BigInt(y)) => {
                let sum = agent.bigint(x).add(agent.bigint(y));
                Value::BigInt(agent.new_bigint(sum))
            }
            _ => unreachable!("RawBytesToNumeric gives one content type"),
        };
        //    h. Let sumBytes be NumericToRawBytes(type, sum, isLittleEndian).
        //    j. Return sumBytes.
        numeric_to_raw_bytes(agent, ty, sum, LITTLE_ENDIAN)
    };
    // 2. Return ? AtomicReadModifyWrite(ta, index, value, add).
    atomic_read_modify_write(agent, ta, index, value, &mut add)
}
