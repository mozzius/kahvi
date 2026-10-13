//! Atomics.and ( _ta_, _index_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.and

use kahvi_core::prelude::*;

/// Atomics.and
pub(crate) fn and(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _and_ be a new read-modify-write modification function with parameters (_xBytes_, _yBytes_) that captures nothing and performs the following steps atomically when called:
    //    a. Return ByteListBitwiseOp(`&`, _xBytes_, _yBytes_).
    // 2. Return ? AtomicReadModifyWrite(_ta_, _index_, _value_, _and_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Atomics.and"))
}
