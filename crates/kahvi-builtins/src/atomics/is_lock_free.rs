//! Atomics.isLockFree ( _size_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.islockfree

use kahvi_core::prelude::*;

/// Atomics.isLockFree
pub(crate) fn is_lock_free(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _n_ be ? ToIntegerOrInfinity(_size_).
    // 2. Let _agentRecord_ be the Agent Record of the surrounding agent.
    // 3. If _n_ = 1, return _agentRecord_.[[IsLockFree1]].
    // 4. If _n_ = 2, return _agentRecord_.[[IsLockFree2]].
    // 5. If _n_ = 4, return *true*.
    // 6. If _n_ = 8, return _agentRecord_.[[IsLockFree8]].
    // 7. Return *false*.
    //
    // NOTE:
    // This function is an optimization primitive. The intuition is that if the atomic step of an atomic primitive (`compareExchange`, `load`, `store`, `add`, `sub`, `and`, `or`, `xor`, or `exchange`) on a datum of size _n_ bytes will be performed without the surrounding agent acquiring a lock outside the _n_ bytes comprising the datum, then `Atomics.isLockFree`(_n_) will return *true*. High-performance algorithms will use this function to determine whether to use locks or atomic operations in critical sections. If an atomic primitive is not lock-free then it is often more efficient for an algorithm to provide its own locking.
    // `Atomics.isLockFree`(4) always returns *true* as that can be supported on all known relevant hardware. Being able to assume this will generally simplify programs.
    // Regardless of the value returned by this function, all atomic operations are guaranteed to be atomic. For example, they will never have a visible operation take place in the middle of the operation (e.g., “tearing”).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Atomics.isLockFree"))
}
