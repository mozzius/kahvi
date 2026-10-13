//! Reflect.construct ( _target_, _args_ [ , _newTarget_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.construct

use kahvi_core::prelude::*;

/// Reflect.construct
pub(crate) fn construct(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If IsConstructor(_target_) is *false*, throw a *TypeError* exception.
    // 2. If _newTarget_ is not present, set _newTarget_ to _target_.
    // 3. Else if IsConstructor(_newTarget_) is *false*, throw a *TypeError* exception.
    // 4. Let _argList_ be ? CreateListFromArrayLike(_args_).
    // 5. Return ? Construct(_target_, _argList_, _newTarget_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Reflect.construct"))
}
