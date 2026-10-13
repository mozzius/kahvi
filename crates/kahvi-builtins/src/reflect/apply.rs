//! Reflect.apply ( _target_, _thisArg_, _args_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.apply

use kahvi_core::prelude::*;

/// Reflect.apply
pub(crate) fn apply(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If IsCallable(_target_) is *false*, throw a *TypeError* exception.
    // 2. Let _argList_ be ? CreateListFromArrayLike(_args_).
    // 3. Perform PrepareForTailCall().
    // 4. Return ? Call(_target_, _thisArg_, _argList_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Reflect.apply"))
}
