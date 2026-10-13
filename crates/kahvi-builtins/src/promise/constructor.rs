//! Promise ( _executor_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise-executor

use kahvi_core::prelude::*;

/// Promise
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. If IsCallable(_executor_) is *false*, throw a *TypeError* exception.
    // 3. Let _promise_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Promise.prototype%"*, « [[PromiseState]], [[PromiseResult]], [[PromiseFulfillReactions]], [[PromiseRejectReactions]], [[PromiseIsHandled]] »).
    // 4. Set _promise_.[[PromiseState]] to ~pending~.
    // 5. Set _promise_.[[PromiseResult]] to ~empty~.
    // 6. Set _promise_.[[PromiseFulfillReactions]] to a new empty List.
    // 7. Set _promise_.[[PromiseRejectReactions]] to a new empty List.
    // 8. Set _promise_.[[PromiseIsHandled]] to *false*.
    // 9. Let _resolvingFuncs_ be CreateResolvingFunctions(_promise_).
    // 10. Let _completion_ be Completion(Call(_executor_, *undefined*, « _resolvingFuncs_.[[Resolve]], _resolvingFuncs_.[[Reject]] »)).
    // 11. If _completion_ is an abrupt completion, then
    //    a. Perform ? Call(_resolvingFuncs_.[[Reject]], *undefined*, « _completion_.[[Value]] »).
    // 12. Return _promise_.
    //
    // NOTE:
    // The _executor_ argument must be a function object. It is called for initiating and reporting completion of the possibly deferred action represented by this Promise. The executor is called with two arguments: _resolve_ and _reject_. These are functions that may be used by the _executor_ function to report eventual completion or failure of the deferred computation. Returning from the executor function does not mean that the deferred action has been completed but only that the request to eventually perform the deferred action has been accepted.
    // The _resolve_ function that is passed to an _executor_ function accepts a single argument. The _executor_ code may eventually call the _resolve_ function to indicate that it wishes to resolve the associated Promise. The argument passed to the _resolve_ function represents the eventual value of the deferred action and can be either the actual fulfillment value or another promise which will provide the value if it is fulfilled.
    // The _reject_ function that is passed to an _executor_ function accepts a single argument. The _executor_ code may eventually call the _reject_ function to indicate that the associated Promise is rejected and will never be fulfilled. The argument passed to the _reject_ function is used as the rejection value of the promise. Typically it will be an Error object.
    // The resolve and reject functions passed to an _executor_ function by the Promise constructor have the capability to actually resolve and reject the associated promise. Subclasses may have different constructor behaviour that passes in customized values for resolve and reject.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Promise"))
}
