//! Promise ( _executor_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise-executor

use kahvi_core::ops::promise;
use kahvi_core::prelude::*;

/// Promise
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If NewTarget is undefined, throw a TypeError exception.
    let Some(new_target) = new_target else {
        return Err(agent.throw_type_error("Promise constructor cannot be invoked without 'new'"));
    };
    // 2. If IsCallable(executor) is false, throw a TypeError exception.
    let executor = args.get_or_undefined(0);
    if !ops::is_callable(agent, executor) {
        return Err(agent.throw_type_error("Promise resolver is not a function"));
    }
    // 3. Let promise be ? OrdinaryCreateFromConstructor(NewTarget, "%Promise.prototype%", « [[PromiseState]], ... »).
    // 4-7. Set [[PromiseState]] to pending, the reaction lists to empty, [[PromiseIsHandled]] to false.
    let proto = ops::get_prototype_from_constructor(agent, new_target, "%Promise.prototype%")?;
    let p = promise::promise_alloc(agent, proto);
    // 8. Let resolvingFunctions be CreateResolvingFunctions(promise).
    let (resolve, reject) = promise::create_resolving_functions(agent, p);
    // 9. Let completion be Completion(Call(executor, undefined, « resolvingFunctions.[[Resolve]], resolvingFunctions.[[Reject]] »)).
    match ops::call(
        agent,
        executor,
        Value::Undefined,
        &[Value::Object(resolve), Value::Object(reject)],
    ) {
        Ok(_) => {}
        // 10. If completion is an abrupt completion, then perform ? Call(resolvingFunctions.[[Reject]], undefined, « completion.[[Value]] »).
        Err(JsError::Throw(e)) => {
            ops::call(agent, Value::Object(reject), Value::Undefined, &[e])?;
        }
        Err(e) => return Err(e),
    }
    // 11. Return promise.
    Ok(Value::Object(p))
}
