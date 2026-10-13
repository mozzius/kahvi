//! Promise.withResolvers ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise.withResolvers

use kahvi_core::prelude::*;

/// Promise.withResolvers
pub(crate) fn with_resolvers(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns an object with three properties: a new promise together with the `resolve` and `reject` functions associated with it.
    //
    // 1. Let _ctor_ be the *this* value.
    // 2. Let _promiseCapability_ be ? NewPromiseCapability(_ctor_).
    // 3. Let _obj_ be OrdinaryObjectCreate(%Object.prototype%).
    // 4. Perform ! CreateDataPropertyOrThrow(_obj_, *"promise"*, _promiseCapability_.[[Promise]]).
    // 5. Perform ! CreateDataPropertyOrThrow(_obj_, *"resolve"*, _promiseCapability_.[[Resolve]]).
    // 6. Perform ! CreateDataPropertyOrThrow(_obj_, *"reject"*, _promiseCapability_.[[Reject]]).
    // 7. Return _obj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Promise.withResolvers"))
}
