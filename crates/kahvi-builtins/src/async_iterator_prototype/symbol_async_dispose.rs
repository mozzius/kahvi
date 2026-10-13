//! %AsyncIteratorPrototype% [ %Symbol.asyncDispose% ] ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%asynciteratorprototype%-%symbol.asyncdispose%

use kahvi_core::prelude::*;

/// AsyncIteratorPrototype[Symbol.asyncDispose]
pub(crate) fn symbol_async_dispose(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Let _promiseCapability_ be ! NewPromiseCapability(%Promise%).
    // 3. Let _return_ be Completion(GetMethod(_obj_, *"return"*)).
    // 4. IfAbruptRejectPromise(_return_, _promiseCapability_).
    // 5. If _return_ is *undefined*, then
    //    a. Perform ! Call(_promiseCapability_.[[Resolve]], *undefined*, « *undefined* »).
    // 6. Else,
    //    a. Let _result_ be Completion(Call(_return_, _obj_, « »)).
    //    b. IfAbruptRejectPromise(_result_, _promiseCapability_).
    //    c. Let _resultWrapper_ be Completion(PromiseResolve(%Promise%, _result_)).
    //    d. IfAbruptRejectPromise(_resultWrapper_, _promiseCapability_).
    //    e. Let _unwrap_ be a new Abstract Closure with no parameters that captures nothing and performs the following steps when called:
    //       i. Return *undefined*.
    //    f. Let _onFulfilled_ be CreateBuiltinFunction(_unwrap_, 1, *""*, « »).
    //    g. Perform PerformPromiseThen(_resultWrapper_, _onFulfilled_, *undefined*, _promiseCapability_).
    // 7. Return _promiseCapability_.[[Promise]].
    //
    // The value of the *"name"* property of this method is *"[Symbol.asyncDispose]"*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("AsyncIteratorPrototype[Symbol.asyncDispose]"))
}
