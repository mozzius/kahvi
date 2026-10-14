//! Atomics.exchange ( _ta_, _index_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.exchange

use kahvi_core::prelude::*;

/// Atomics.exchange
pub(crate) fn exchange(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _second_ be a new read-modify-write modification function with parameters (_oldBytes_, _newBytes_) that captures nothing and performs the following steps atomically when called:
    //    a. Return _newBytes_.
    // 2. Return ? AtomicReadModifyWrite(_ta_, _index_, _value_, _second_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Atomics.exchange"))
}
