//! Promise.reject ( _reason_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise.reject

use kahvi_core::prelude::*;

/// Promise.reject
pub(crate) fn reject(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns a new promise rejected with the passed argument.
    //
    // 1. Let _ctor_ be the *this* value.
    // 2. Let _promiseCapability_ be ? NewPromiseCapability(_ctor_).
    // 3. Perform ? Call(_promiseCapability_.[[Reject]], *undefined*, « _reason_ »).
    // 4. Return _promiseCapability_.[[Promise]].
    //
    // NOTE:
    // This function expects its *this* value to be a constructor function that supports the parameter conventions of the Promise constructor.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Promise.reject"))
}
