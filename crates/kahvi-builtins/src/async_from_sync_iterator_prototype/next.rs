//! %AsyncFromSyncIteratorPrototype%.next ( [ _value_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%asyncfromsynciteratorprototype%.next

use kahvi_core::prelude::*;

/// AsyncFromSyncIteratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _obj_ be the *this* value.
    // 2. Assert: _obj_ is an Object that has a [[SyncIteratorRecord]] internal slot.
    // 3. Let _promiseCapability_ be ! NewPromiseCapability(%Promise%).
    // 4. Let _syncIteratorRecord_ be _obj_.[[SyncIteratorRecord]].
    // 5. If _value_ is present, then
    //    a. Let _result_ be Completion(IteratorNext(_syncIteratorRecord_, _value_)).
    // 6. Else,
    //    a. Let _result_ be Completion(IteratorNext(_syncIteratorRecord_)).
    // 7. IfAbruptRejectPromise(_result_, _promiseCapability_).
    // 8. Return AsyncFromSyncIteratorContinuation(_result_, _promiseCapability_, _syncIteratorRecord_, *true*).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("AsyncFromSyncIteratorPrototype.next"))
}
