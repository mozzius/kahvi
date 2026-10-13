//! %AsyncGeneratorPrototype%.return ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncgenerator-prototype-return

use kahvi_core::prelude::*;

/// AsyncGeneratorPrototype.return
pub(crate) fn r#return(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _gen_ be the *this* value.
    // 2. Let _promiseCapability_ be ! NewPromiseCapability(%Promise%).
    // 3. Let _result_ be Completion(AsyncGeneratorValidate(_gen_, ~empty~)).
    // 4. IfAbruptRejectPromise(_result_, _promiseCapability_).
    // 5. Let _completion_ be ReturnCompletion(_value_).
    // 6. Perform AsyncGeneratorEnqueue(_gen_, _completion_, _promiseCapability_).
    // 7. Let _state_ be _gen_.[[AsyncGeneratorState]].
    // 8. If _state_ is either ~suspended-start~ or ~completed~, then
    //    a. Set _gen_.[[AsyncGeneratorState]] to ~draining-queue~.
    //    b. Perform AsyncGeneratorAwaitReturn(_gen_).
    // 9. Else if _state_ is ~suspended-yield~, then
    //    a. Perform AsyncGeneratorResume(_gen_, _completion_).
    // 10. Else,
    //    a. Assert: _state_ is either ~executing~ or ~draining-queue~.
    // 11. Return _promiseCapability_.[[Promise]].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("AsyncGeneratorPrototype.return"))
}
