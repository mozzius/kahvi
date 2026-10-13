//! Atomics.notify ( _ta_, _index_, _count_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.notify

use kahvi_core::prelude::*;

/// Atomics.notify
pub(crate) fn notify(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function notifies some agents that are sleeping in the wait queue.
    // It performs the following steps when called:
    //
    // 1. Let _taRecord_ be ? ValidateIntegerTypedArray(_ta_, *true*).
    // 2. Let _byteIndexInBuffer_ be ? ValidateAtomicAccess(_taRecord_, _index_).
    // 3. If _count_ is *undefined*, then
    //    a. Set _count_ to +∞.
    // 4. Else,
    //    a. Let _intCount_ be ? ToIntegerOrInfinity(_count_).
    //    b. Set _count_ to max(_intCount_, 0).
    // 5. Let _buffer_ be _ta_.[[ViewedArrayBuffer]].
    // 6. If IsSharedArrayBuffer(_buffer_) is *false*, return *+0*_𝔽.
    // 7. Let _block_ be _buffer_.[[ArrayBufferData]].
    // 8. Let _waiterList_ be GetWaiterList(_block_, _byteIndexInBuffer_).
    // 9. Perform EnterCriticalSection(_waiterList_).
    // 10. Let _waiters_ be RemoveWaiters(_waiterList_, _count_).
    // 11. For each element _waiterRecord_ of _waiters_, do
    //    a. Perform NotifyWaiter(_waiterList_, _waiterRecord_).
    // 12. Perform LeaveCriticalSection(_waiterList_).
    // 13. Let _waitersCount_ be the number of elements in _waiters_.
    // 14. Return 𝔽(_waitersCount_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Atomics.notify"))
}
