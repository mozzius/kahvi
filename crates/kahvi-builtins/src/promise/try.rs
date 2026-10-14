//! Promise.try ( _callback_, ..._args_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise.try

use kahvi_core::prelude::*;

/// Promise.try
pub(crate) fn r#try(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _ctor_ be the *this* value.
    // 2. If _ctor_ is not an Object, throw a *TypeError* exception.
    // 3. Let _status_ be Completion(Call(_callback_, *undefined*, _args_)).
    // 4. If _status_ is an abrupt completion, then
    //    a. Let _promiseCapability_ be ? NewPromiseCapability(_ctor_).
    //    b. Perform ? Call(_promiseCapability_.[[Reject]], *undefined*, « _status_.[[Value]] »).
    //    c. Return _promiseCapability_.[[Promise]].
    // 5. Else,
    //    a. Return ? PromiseResolve(_ctor_, ! _status_).
    //
    // NOTE:
    // This function expects its *this* value to be a constructor function that supports the parameter conventions of the Promise constructor.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Promise.try"))
}
