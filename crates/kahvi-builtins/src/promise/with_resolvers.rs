//! Promise.withResolvers ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise.withResolvers

use kahvi_core::ops::promise;
use kahvi_core::prelude::*;

/// Promise.withResolvers
pub(crate) fn with_resolvers(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns an object with three properties: a new promise together with the `resolve` and `reject` functions associated with it.
    //
    // 1. Let _ctor_ be the *this* value.
    let ctor = this;
    // 2. Let _promiseCapability_ be ? NewPromiseCapability(_ctor_).
    let promise_capability = promise::new_promise_capability(agent, ctor)?;
    // 3. Let _obj_ be OrdinaryObjectCreate(%Object.prototype%).
    let proto = agent.intrinsic("%Object.prototype%");
    let obj = agent.ordinary_object_create(Some(proto));
    // 4. Perform ! CreateDataPropertyOrThrow(_obj_, *"promise"*, _promiseCapability_.[[Promise]]).
    let key = agent.key("promise");
    ops::create_data_property_or_throw(agent, obj, key, promise_capability.promise)?;
    // 5. Perform ! CreateDataPropertyOrThrow(_obj_, *"resolve"*, _promiseCapability_.[[Resolve]]).
    let key = agent.key("resolve");
    ops::create_data_property_or_throw(agent, obj, key, promise_capability.resolve)?;
    // 6. Perform ! CreateDataPropertyOrThrow(_obj_, *"reject"*, _promiseCapability_.[[Reject]]).
    let key = agent.key("reject");
    ops::create_data_property_or_throw(agent, obj, key, promise_capability.reject)?;
    // 7. Return _obj_.
    Ok(Value::Object(obj))
}
