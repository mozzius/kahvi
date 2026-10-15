//! Atomics.notify ( _ta_, _index_, _count_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.notify

use crate::support::atomics::{
    validate_atomic_access, validate_integer_typed_array, viewed_buffer,
};
use kahvi_core::object::typed_array::is_shared_array_buffer;
use kahvi_core::prelude::*;

/// Atomics.notify
pub(crate) fn notify(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (this, new_target);
    let (ta, index, count) = (
        args.get_or_undefined(0),
        args.get_or_undefined(1),
        args.get_or_undefined(2),
    );
    // This function notifies some agents that are sleeping in the wait queue.
    // It performs the following steps when called:
    //
    // 1. Let _taRecord_ be ? ValidateIntegerTypedArray(_ta_, *true*).
    let ta_record = validate_integer_typed_array(agent, ta, true)?;
    let ta_object = ta_record.object;
    // 2. Let _byteIndexInBuffer_ be ? ValidateAtomicAccess(_taRecord_, _index_).
    let byte_index_in_buffer = validate_atomic_access(agent, ta_record, index)?;
    // 3. If _count_ is *undefined*, then
    //    a. Set _count_ to +∞.
    // 4. Else,
    //    a. Let _intCount_ be ? ToIntegerOrInfinity(_count_).
    //    b. Set _count_ to max(_intCount_, 0).
    let count = if matches!(count, Value::Undefined) {
        f64::INFINITY
    } else {
        ops::to_integer_or_infinity(agent, count)?.max(0.0)
    };
    // 5. Let _buffer_ be _ta_.[[ViewedArrayBuffer]].
    let buffer = viewed_buffer(agent, ta_object);
    // 6. If IsSharedArrayBuffer(_buffer_) is *false*, return *+0*_𝔽.
    if !is_shared_array_buffer(agent, buffer) {
        return Ok(Value::Number(0.0));
    }
    // 7. Let _block_ be _buffer_.[[ArrayBufferData]].
    // 8. Let _waiterList_ be GetWaiterList(_block_, _byteIndexInBuffer_).
    // 9. Perform EnterCriticalSection(_waiterList_).
    // 10. Let _waiters_ be RemoveWaiters(_waiterList_, _count_).
    //     Kahvi has one agent and it can never be suspended in a WaiterList
    //     (see Atomics.wait), so every WaiterList is empty. RemoveWaiters
    //     then takes min(_count_, 0) = 0 elements, so no waiters are removed.
    let _ = byte_index_in_buffer;
    let waiters_count = count.min(0.0) as usize;
    // 11. For each element _waiterRecord_ of _waiters_, do
    //     a. Perform NotifyWaiter(_waiterList_, _waiterRecord_).
    //     (No waiters, so there is nothing to notify.)
    // 12. Perform LeaveCriticalSection(_waiterList_).
    // 13. Let _waitersCount_ be the number of elements in _waiters_.
    // 14. Return 𝔽(_waitersCount_).
    Ok(Value::Number(waiters_count as f64))
}
