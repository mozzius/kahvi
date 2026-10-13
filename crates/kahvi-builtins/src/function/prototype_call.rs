//! Function.prototype.call ( _thisArg_, ..._args_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-function.prototype.call

use kahvi_core::prelude::*;

/// Function.prototype.call
pub(crate) fn prototype_call(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _func_ be the *this* value.
    // 2. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
    // 3. Perform PrepareForTailCall().
    // 4. Return ? Call(_func_, _thisArg_, _args_).
    //
    // NOTE:
    // The _thisArg_ value is passed without modification as the *this* value. This is a change from Edition 3, where an *undefined* or *null* _thisArg_ is replaced with the global object and ToObject is applied to all other values and that result is passed as the *this* value. Even though the _thisArg_ is passed without modification, non-strict functions still perform these transformations upon entry to the function.
    // NOTE:
    // If _func_ is either an arrow function or a bound function exotic object, then the _thisArg_ will be ignored by the function [[Call]] in step [step-function-proto-call-call].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Function.prototype.call"))
}
