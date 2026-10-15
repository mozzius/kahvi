//! Atomics.and ( _ta_, _index_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.and

use crate::support::atomics::{atomic_read_modify_write, byte_list_bitwise_op};
use kahvi_core::prelude::*;

/// Atomics.and
pub(crate) fn and(
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
    // 1. Let and be a new read-modify-write modification function with parameters (xBytes, yBytes) that captures nothing and performs the following steps atomically when called:
    //    a. Return ByteListBitwiseOp(&, xBytes, yBytes).
    let mut and = |_: &mut Agent, x: &[u8], y: &[u8]| byte_list_bitwise_op(|a, b| a & b, x, y);
    // 2. Return ? AtomicReadModifyWrite(ta, index, value, and).
    atomic_read_modify_write(agent, ta, index, value, &mut and)
}
