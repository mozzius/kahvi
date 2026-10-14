//! Promise.prototype.finally ( _onFinally_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-promise.prototype.finally

use kahvi_core::prelude::*;

/// Promise.prototype.finally
pub(crate) fn prototype_finally(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _promise_ be the *this* value.
    // 2. If _promise_ is not an Object, throw a *TypeError* exception.
    // 3. Let _ctor_ be ? SpeciesConstructor(_promise_, %Promise%).
    // 4. Assert: IsConstructor(_ctor_) is *true*.
    // 5. If IsCallable(_onFinally_) is *false*, then
    //    a. Let _thenFinally_ be _onFinally_.
    //    b. Let _catchFinally_ be _onFinally_.
    // 6. Else,
    //    a. Let _thenFinallyClosure_ be a new Abstract Closure with parameters (_value_) that captures _onFinally_ and _ctor_ and performs the following steps when called:
    //       i. Let _result_ be ? Call(_onFinally_, *undefined*).
    //       ii. Let _p_ be ? PromiseResolve(_ctor_, _result_).
    //       iii. Let _returnValue_ be a new Abstract Closure with no parameters that captures _value_ and performs the following steps when called:
    //          1. Return NormalCompletion(_value_).
    //       iv. Let _valueThunk_ be CreateBuiltinFunction(_returnValue_, 0, *""*, « »).
    //       v. Return ? Invoke(_p_, *"then"*, « _valueThunk_ »).
    //    b. Let _thenFinally_ be CreateBuiltinFunction(_thenFinallyClosure_, 1, *""*, « »).
    //    c. Let _catchFinallyClosure_ be a new Abstract Closure with parameters (_reason_) that captures _onFinally_ and _ctor_ and performs the following steps when called:
    //       i. Let _result_ be ? Call(_onFinally_, *undefined*).
    //       ii. Let _p_ be ? PromiseResolve(_ctor_, _result_).
    //       iii. Let _throwReason_ be a new Abstract Closure with no parameters that captures _reason_ and performs the following steps when called:
    //          1. Throw _reason_.
    //       iv. Let _thrower_ be CreateBuiltinFunction(_throwReason_, 0, *""*, « »).
    //       v. Return ? Invoke(_p_, *"then"*, « _thrower_ »).
    //    d. Let _catchFinally_ be CreateBuiltinFunction(_catchFinallyClosure_, 1, *""*, « »).
    // 7. Return ? Invoke(_promise_, *"then"*, « _thenFinally_, _catchFinally_ »).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Promise.prototype.finally"))
}
