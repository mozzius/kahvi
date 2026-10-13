//! %AsyncFromSyncIteratorPrototype%.return ( [ _value_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%asyncfromsynciteratorprototype%.return

use kahvi_core::prelude::*;

/// AsyncFromSyncIteratorPrototype.return
pub(crate) fn r#return(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _obj_ be the *this* value.
    // 2. Assert: _obj_ is an Object that has a [[SyncIteratorRecord]] internal slot.
    // 3. Let _promiseCapability_ be ! NewPromiseCapability(%Promise%).
    // 4. Let _syncIteratorRecord_ be _obj_.[[SyncIteratorRecord]].
    // 5. Let _syncIterator_ be _syncIteratorRecord_.[[Iterator]].
    // 6. Let _return_ be Completion(GetMethod(_syncIterator_, *"return"*)).
    // 7. IfAbruptRejectPromise(_return_, _promiseCapability_).
    // 8. If _return_ is *undefined*, then
    //    a. Let _iteratorResult_ be CreateIteratorResultObject(_value_, *true*).
    //    b. Perform ! Call(_promiseCapability_.[[Resolve]], *undefined*, « _iteratorResult_ »).
    //    c. Return _promiseCapability_.[[Promise]].
    // 9. If _value_ is present, then
    //    a. Let _result_ be Completion(Call(_return_, _syncIterator_, « _value_ »)).
    // 10. Else,
    //    a. Let _result_ be Completion(Call(_return_, _syncIterator_)).
    // 11. IfAbruptRejectPromise(_result_, _promiseCapability_).
    // 12. If _result_ is not an Object, then
    //    a. Perform ! Call(_promiseCapability_.[[Reject]], *undefined*, « a newly created *TypeError* object »).
    //    b. Return _promiseCapability_.[[Promise]].
    // 13. Return AsyncFromSyncIteratorContinuation(_result_, _promiseCapability_, _syncIteratorRecord_, *false*).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("AsyncFromSyncIteratorPrototype.return"))
}
