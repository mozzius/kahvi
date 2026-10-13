//! %AsyncFromSyncIteratorPrototype%.throw ( [ _value_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%asyncfromsynciteratorprototype%.throw

use kahvi_core::prelude::*;

/// AsyncFromSyncIteratorPrototype.throw
pub(crate) fn throw(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE: In this specification, _value_ is always provided, but is left optional for consistency with .
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Assert: _obj_ is an Object that has a [[SyncIteratorRecord]] internal slot.
    // 3. Let _promiseCapability_ be ! NewPromiseCapability(%Promise%).
    // 4. Let _syncIteratorRecord_ be _obj_.[[SyncIteratorRecord]].
    // 5. Let _syncIterator_ be _syncIteratorRecord_.[[Iterator]].
    // 6. Let _throw_ be Completion(GetMethod(_syncIterator_, *"throw"*)).
    // 7. IfAbruptRejectPromise(_throw_, _promiseCapability_).
    // 8. If _throw_ is *undefined*, then
    //    a. NOTE: If _syncIterator_ does not have a `throw` method, close it to give it a chance to clean up before we reject the capability.
    //    b. Let _closeCompletion_ be NormalCompletion(~empty~).
    //    c. Let _result_ be Completion(IteratorClose(_syncIteratorRecord_, _closeCompletion_)).
    //    d. IfAbruptRejectPromise(_result_, _promiseCapability_).
    //    e. NOTE: The next step throws a *TypeError* to indicate that there was a protocol violation: _syncIterator_ does not have a `throw` method.
    //    f. NOTE: If closing _syncIterator_ does not throw then the result of that operation is ignored, even if it yields a rejected promise.
    //    g. Perform ! Call(_promiseCapability_.[[Reject]], *undefined*, « a newly created *TypeError* object »).
    //    h. Return _promiseCapability_.[[Promise]].
    // 9. If _value_ is present, then
    //    a. Let _result_ be Completion(Call(_throw_, _syncIterator_, « _value_ »)).
    // 10. Else,
    //    a. Let _result_ be Completion(Call(_throw_, _syncIterator_)).
    // 11. IfAbruptRejectPromise(_result_, _promiseCapability_).
    // 12. If _result_ is not an Object, then
    //    a. Perform ! Call(_promiseCapability_.[[Reject]], *undefined*, « a newly created *TypeError* object »).
    //    b. Return _promiseCapability_.[[Promise]].
    // 13. Return AsyncFromSyncIteratorContinuation(_result_, _promiseCapability_, _syncIteratorRecord_, *true*).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("AsyncFromSyncIteratorPrototype.throw"))
}
