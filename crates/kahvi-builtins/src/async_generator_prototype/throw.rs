//! %AsyncGeneratorPrototype%.throw ( _exception_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncgenerator-prototype-throw

use kahvi_core::prelude::*;

/// AsyncGeneratorPrototype.throw
pub(crate) fn throw(
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
    // 6. If _state_ is ~suspended-start~, then
    //    a. Set _gen_.[[AsyncGeneratorState]] to ~completed~.
    //    b. Set _state_ to ~completed~.
    // 7. If _state_ is ~completed~, then
    //    a. Perform ! Call(_promiseCapability_.[[Reject]], *undefined*, « _exception_ »).
    //    b. Return _promiseCapability_.[[Promise]].
    // 8. Let _completion_ be ThrowCompletion(_exception_).
    // 9. Perform AsyncGeneratorEnqueue(_gen_, _completion_, _promiseCapability_).
    // 10. If _state_ is ~suspended-yield~, then
    //    a. Perform AsyncGeneratorResume(_gen_, _completion_).
    // 11. Else,
    //    a. Assert: _state_ is either ~executing~ or ~draining-queue~.
    // 12. Return _promiseCapability_.[[Promise]].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("AsyncGeneratorPrototype.throw"))
}
