//! %AsyncGeneratorPrototype%.next ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncgenerator-prototype-next

use kahvi_core::prelude::*;

/// AsyncGeneratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _gen_ be the *this* value.
    // 2. Let _promiseCapability_ be ! NewPromiseCapability(%Promise%).
    // 3. Let _result_ be Completion(AsyncGeneratorValidate(_gen_, ~empty~)).
    // 4. IfAbruptRejectPromise(_result_, _promiseCapability_).
    // 5. Let _state_ be _gen_.[[AsyncGeneratorState]].
    // 6. If _state_ is ~completed~, then
    //    a. Let _iteratorResult_ be CreateIteratorResultObject(*undefined*, *true*).
    //    b. Perform ! Call(_promiseCapability_.[[Resolve]], *undefined*, « _iteratorResult_ »).
    //    c. Return _promiseCapability_.[[Promise]].
    // 7. Let _completion_ be NormalCompletion(_value_).
    // 8. Perform AsyncGeneratorEnqueue(_gen_, _completion_, _promiseCapability_).
    // 9. If _state_ is either ~suspended-start~ or ~suspended-yield~, then
    //    a. Perform AsyncGeneratorResume(_gen_, _completion_).
    // 10. Else,
    //    a. Assert: _state_ is either ~executing~ or ~draining-queue~.
    // 11. Return _promiseCapability_.[[Promise]].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("AsyncGeneratorPrototype.next"))
}
