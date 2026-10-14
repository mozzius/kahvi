//! %AsyncIteratorPrototype% [ %Symbol.asyncDispose% ] ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%asynciteratorprototype%-%symbol.asyncdispose%

use kahvi_core::function::create_builtin_function;
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
    let _ = (args, new_target);
    // 1. Let _obj_ be the *this* value.
    let obj = this;
    // 2. Let _promiseCapability_ be ! NewPromiseCapability(%Promise%).
    let promise_ctor = Value::Object(agent.intrinsic("%Promise%"));
    let capability = ops::promise::new_promise_capability(agent, promise_ctor)?;
    // 3. Let _return_ be Completion(GetMethod(_obj_, *"return"*)).
    let return_key = agent.key("return");
    let return_ = ops::get_method(agent, obj, return_key);
    // 4. IfAbruptRejectPromise(_return_, _promiseCapability_).
    let return_ = match reject_if_abrupt(agent, return_, capability)? {
        Ok(v) => v,
        Err(promise) => return Ok(promise),
    };
    // 5. If _return_ is *undefined*, then
    if matches!(return_, Value::Undefined) {
        // a. Perform ! Call(_promiseCapability_.[[Resolve]], *undefined*, « *undefined* »).
        ops::call(
            agent,
            capability.resolve,
            Value::Undefined,
            &[Value::Undefined],
        )?;
    } else {
        // 6. Else,
        // a. Let _result_ be Completion(Call(_return_, _obj_, « »)).
        let result = ops::call(agent, return_, obj, &[]);
        // b. IfAbruptRejectPromise(_result_, _promiseCapability_).
        let result = match reject_if_abrupt(agent, result, capability)? {
            Ok(v) => v,
            Err(promise) => return Ok(promise),
        };
        // c. Let _resultWrapper_ be Completion(PromiseResolve(%Promise%, _result_)).
        let promise_ctor = agent.intrinsic("%Promise%");
        let result_wrapper = ops::promise::promise_resolve(agent, promise_ctor, result);
        // d. IfAbruptRejectPromise(_resultWrapper_, _promiseCapability_).
        let result_wrapper = match reject_if_abrupt(agent, result_wrapper, capability)? {
            Ok(v) => v,
            Err(promise) => return Ok(promise),
        };
        // e. Let _unwrap_ be a new Abstract Closure with no parameters that captures nothing and performs the following steps when called:
        //    i. Return *undefined*.
        // f. Let _onFulfilled_ be CreateBuiltinFunction(_unwrap_, 1, *""*, « »).
        let empty = PropertyKey::String(agent.string(""));
        let on_fulfilled =
            create_builtin_function(agent, unwrap, 1, empty, Vec::new(), None, None, None, false);
        // g. Perform PerformPromiseThen(_resultWrapper_, _onFulfilled_, *undefined*, _promiseCapability_).
        ops::promise::perform_promise_then(
            agent,
            result_wrapper,
            Value::Object(on_fulfilled),
            Value::Undefined,
            Some(capability),
        );
    }
    // 7. Return _promiseCapability_.[[Promise]].
    Ok(capability.promise)
}

/// The closure of step 6.e.
fn unwrap(
    _agent: &mut Agent,
    _this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // i. Return *undefined*.
    Ok(Value::Undefined)
}

/// IfAbruptRejectPromise(value, capability): `Ok(Err(promise))` means return the promise.
fn reject_if_abrupt<T>(
    agent: &mut Agent,
    r: JsResult<T>,
    capability: ops::promise::PromiseCapability,
) -> JsResult<Result<T, Value>> {
    match r {
        Ok(v) => Ok(Ok(v)),
        Err(JsError::Throw(e)) => {
            ops::call(agent, capability.reject, Value::Undefined, &[e])?;
            Ok(Err(capability.promise))
        }
        Err(e) => Err(e),
    }
}
