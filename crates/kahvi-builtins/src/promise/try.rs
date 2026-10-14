//! Promise.try ( _callback_, ..._args_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise.try

use kahvi_core::error::JsError;
use kahvi_core::ops::promise;
use kahvi_core::prelude::*;

/// Promise.try
pub(crate) fn r#try(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _ctor_ be the *this* value.
    let ctor = this;
    // 2. If _ctor_ is not an Object, throw a *TypeError* exception.
    let ctor = match ctor {
        Value::Object(o) => o,
        _ => return Err(agent.throw_type_error("Promise.try called on a non-object")),
    };
    // 3. Let _status_ be Completion(Call(_callback_, *undefined*, _args_)).
    let callback = args.get_or_undefined(0);
    let rest = args.get(1..).unwrap_or_default();
    let status = ops::call(agent, callback, Value::Undefined, rest);
    match status {
        // 4. If _status_ is an abrupt completion, then
        Err(JsError::Throw(reason)) => {
            // a. Let _promiseCapability_ be ? NewPromiseCapability(_ctor_).
            let promise_capability = promise::new_promise_capability(agent, Value::Object(ctor))?;
            // b. Perform ? Call(_promiseCapability_.[[Reject]], *undefined*, « _status_.[[Value]] »).
            ops::call(
                agent,
                promise_capability.reject,
                Value::Undefined,
                &[reason],
            )?;
            // c. Return _promiseCapability_.[[Promise]].
            Ok(promise_capability.promise)
        }
        // Uncatchable engine errors are not abrupt completions of the callback.
        Err(e @ JsError::NotImplemented(_)) => Err(e),
        // 5. Else,
        Ok(value) => {
            // a. Return ? PromiseResolve(_ctor_, ! _status_).
            let p = promise::promise_resolve(agent, ctor, value)?;
            Ok(Value::Object(p))
        }
    }
}
