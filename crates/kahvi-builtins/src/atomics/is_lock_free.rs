//! Atomics.isLockFree ( _size_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.islockfree

use kahvi_core::prelude::*;

/// The surrounding agent's [[IsLockFree1]]. The spec leaves this implementation-defined.
/// Atomic operations here are plain byte accesses with a single agent, so no lock is taken.
const IS_LOCK_FREE_1: bool = true;

/// The surrounding agent's [[IsLockFree2]]. The spec leaves this implementation-defined.
const IS_LOCK_FREE_2: bool = true;

/// The surrounding agent's [[IsLockFree8]]. The spec leaves this implementation-defined.
const IS_LOCK_FREE_8: bool = true;

/// Atomics.isLockFree
pub(crate) fn is_lock_free(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (this, new_target);
    let size = args.get_or_undefined(0);
    // 1. Let _n_ be ? ToIntegerOrInfinity(_size_).
    let n = ops::to_integer_or_infinity(agent, size)?;
    // 2. Let _agentRecord_ be the Agent Record of the surrounding agent.
    // 3. If _n_ = 1, return _agentRecord_.[[IsLockFree1]].
    if n == 1.0 {
        return Ok(Value::Boolean(IS_LOCK_FREE_1));
    }
    // 4. If _n_ = 2, return _agentRecord_.[[IsLockFree2]].
    if n == 2.0 {
        return Ok(Value::Boolean(IS_LOCK_FREE_2));
    }
    // 5. If _n_ = 4, return *true*.
    if n == 4.0 {
        return Ok(Value::Boolean(true));
    }
    // 6. If _n_ = 8, return _agentRecord_.[[IsLockFree8]].
    if n == 8.0 {
        return Ok(Value::Boolean(IS_LOCK_FREE_8));
    }
    // 7. Return *false*.
    Ok(Value::Boolean(false))
}
