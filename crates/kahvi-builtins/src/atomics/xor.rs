//! Atomics.xor ( _ta_, _index_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.xor

use crate::support::atomics::{atomic_read_modify_write, byte_list_bitwise_op};
use kahvi_core::prelude::*;

/// Atomics.xor
pub(crate) fn xor(
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
    // 1. Let _xor_ be a new read-modify-write modification function with parameters (_xBytes_, _yBytes_) that captures nothing and performs the following steps atomically when called:
    //    a. Return ByteListBitwiseOp(`^`, _xBytes_, _yBytes_).
    let mut xor = |_: &mut Agent, x: &[u8], y: &[u8]| byte_list_bitwise_op(|a, b| a ^ b, x, y);
    // 2. Return ? AtomicReadModifyWrite(_ta_, _index_, _value_, _xor_).
    atomic_read_modify_write(agent, ta, index, value, &mut xor)
}
