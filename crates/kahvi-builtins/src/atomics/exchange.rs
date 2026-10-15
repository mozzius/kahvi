//! Atomics.exchange ( _ta_, _index_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.exchange

use crate::support::atomics::atomic_read_modify_write;
use kahvi_core::prelude::*;

/// Atomics.exchange
pub(crate) fn exchange(
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
    // 1. Let _second_ be a new read-modify-write modification function with parameters (_oldBytes_, _newBytes_) that captures nothing and performs the following steps atomically when called:
    let mut second = |_agent: &mut Agent, _old_bytes: &[u8], new_bytes: &[u8]| {
        //    a. Return _newBytes_.
        new_bytes.to_vec()
    };
    // 2. Return ? AtomicReadModifyWrite(_ta_, _index_, _value_, _second_).
    atomic_read_modify_write(agent, ta, index, value, &mut second)
}
