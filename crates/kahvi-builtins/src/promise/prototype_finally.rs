//! Promise.prototype.finally ( _onFinally_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise.prototype.finally

use kahvi_core::function::builtin_slots;
use kahvi_core::ops::promise;
use kahvi_core::prelude::*;

/// Promise.prototype.finally
pub(crate) fn prototype_finally(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _promise_ be the *this* value.
    let promise_value = this;
    // 2. If _promise_ is not an Object, throw a *TypeError* exception.
    let Value::Object(promise_obj) = promise_value else {
        return Err(agent.throw_type_error("Promise.prototype.finally called on a non-object"));
    };
    // 3. Let _ctor_ be ? SpeciesConstructor(_promise_, %Promise%).
    let default = agent.intrinsic("%Promise%");
    let ctor = ops::species_constructor(agent, promise_obj, default)?;
    // 4. Assert: IsConstructor(_ctor_) is *true*.
    // (SpeciesConstructor only returns a constructor, so there is nothing to check here.)
    // 5. If IsCallable(_onFinally_) is *false*, then
    let on_finally = args.get_or_undefined(0);
    let (then_finally, catch_finally) = if !ops::is_callable(agent, on_finally) {
        // a. Let _thenFinally_ be _onFinally_.
        // b. Let _catchFinally_ be _onFinally_.
        (on_finally, on_finally)
    } else {
        // 6. Else,
        // a. Let _thenFinallyClosure_ be a new Abstract Closure with parameters (_value_) that captures _onFinally_ and _ctor_ and performs the following steps when called:
        //    (see `then_finally_closure`)
        // b. Let _thenFinally_ be CreateBuiltinFunction(_thenFinallyClosure_, 1, *""*, « »).
        let empty = PropertyKey::String(agent.names.empty);
        let then_finally = create_builtin_function(
            agent,
            then_finally_closure,
            1,
            empty,
            vec![on_finally, Value::Object(ctor)],
            None,
            None,
            None,
            false,
        );
        // c. Let _catchFinallyClosure_ be a new Abstract Closure with parameters (_reason_) that captures _onFinally_ and _ctor_ and performs the following steps when called:
        //    (see `catch_finally_closure`)
        // d. Let _catchFinally_ be CreateBuiltinFunction(_catchFinallyClosure_, 1, *""*, « »).
        let catch_finally = create_builtin_function(
            agent,
            catch_finally_closure,
            1,
            empty,
            vec![on_finally, Value::Object(ctor)],
            None,
            None,
            None,
            false,
        );
        (Value::Object(then_finally), Value::Object(catch_finally))
    };
    // 7. Return ? Invoke(_promise_, *"then"*, « _thenFinally_, _catchFinally_ »).
    let then_key = PropertyKey::String(agent.names.then);
    ops::invoke(
        agent,
        promise_value,
        then_key,
        &[then_finally, catch_finally],
    )
}

/// The slots of the active built-in function (the captured values of a closure).
fn active_slots(agent: &Agent) -> Vec<Value> {
    let Some(f) = agent.active_function() else {
        unreachable!("a built-in runs with an active function")
    };
    builtin_slots(agent, f).to_vec()
}

/// The Abstract Closure of thenFinally (§27.2.5.3 step 6.a), which captures
/// _onFinally_ and _ctor_.
fn then_finally_closure(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // Parameters: (_value_)
    let value = args.get_or_undefined(0);
    let slots = active_slots(agent);
    let (Some(&on_finally), Some(&Value::Object(ctor))) = (slots.first(), slots.get(1)) else {
        unreachable!("thenFinally closure slots")
    };
    // i. Let _result_ be ? Call(_onFinally_, *undefined*).
    let result = ops::call(agent, on_finally, Value::Undefined, &[])?;
    // ii. Let _p_ be ? PromiseResolve(_ctor_, _result_).
    let p = promise::promise_resolve(agent, ctor, result)?;
    // iii. Let _returnValue_ be a new Abstract Closure with no parameters that captures _value_ and performs the following steps when called:
    //      1. Return NormalCompletion(_value_).
    // iv. Let _valueThunk_ be CreateBuiltinFunction(_returnValue_, 0, *""*, « »).
    let empty = PropertyKey::String(agent.names.empty);
    let value_thunk = create_builtin_function(
        agent,
        return_value_closure,
        0,
        empty,
        vec![value],
        None,
        None,
        None,
        false,
    );
    // v. Return ? Invoke(_p_, *"then"*, « _valueThunk_ »).
    let then_key = PropertyKey::String(agent.names.then);
    ops::invoke(
        agent,
        Value::Object(p),
        then_key,
        &[Value::Object(value_thunk)],
    )
}

/// The Abstract Closure of catchFinally (§27.2.5.3 step 6.c), which captures
/// _onFinally_ and _ctor_.
fn catch_finally_closure(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // Parameters: (_reason_)
    let reason = args.get_or_undefined(0);
    let slots = active_slots(agent);
    let (Some(&on_finally), Some(&Value::Object(ctor))) = (slots.first(), slots.get(1)) else {
        unreachable!("catchFinally closure slots")
    };
    // i. Let _result_ be ? Call(_onFinally_, *undefined*).
    let result = ops::call(agent, on_finally, Value::Undefined, &[])?;
    // ii. Let _p_ be ? PromiseResolve(_ctor_, _result_).
    let p = promise::promise_resolve(agent, ctor, result)?;
    // iii. Let _throwReason_ be a new Abstract Closure with no parameters that captures _reason_ and performs the following steps when called:
    //      1. Throw _reason_.
    // iv. Let _thrower_ be CreateBuiltinFunction(_throwReason_, 0, *""*, « »).
    let empty = PropertyKey::String(agent.names.empty);
    let thrower = create_builtin_function(
        agent,
        throw_reason_closure,
        0,
        empty,
        vec![reason],
        None,
        None,
        None,
        false,
    );
    // v. Return ? Invoke(_p_, *"then"*, « _thrower_ »).
    let then_key = PropertyKey::String(agent.names.then);
    ops::invoke(agent, Value::Object(p), then_key, &[Value::Object(thrower)])
}

/// The Abstract Closure returnValue (§27.2.5.3 step 6.a.iii), which captures _value_.
fn return_value_closure(
    agent: &mut Agent,
    _this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return NormalCompletion(_value_).
    let slots = active_slots(agent);
    let Some(&value) = slots.first() else {
        unreachable!("returnValue closure slots")
    };
    Ok(value)
}

/// The Abstract Closure throwReason (§27.2.5.3 step 6.c.iii), which captures _reason_.
fn throw_reason_closure(
    agent: &mut Agent,
    _this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Throw _reason_.
    let slots = active_slots(agent);
    let Some(&reason) = slots.first() else {
        unreachable!("throwReason closure slots")
    };
    Err(JsError::Throw(reason))
}
