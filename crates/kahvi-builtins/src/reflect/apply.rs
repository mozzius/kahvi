//! Reflect.apply ( _target_, _thisArg_, _args_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.apply

use kahvi_core::prelude::*;

/// Reflect.apply
pub(crate) fn apply(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let target = args.get_or_undefined(0);
    // 1. If IsCallable(target) is false, throw a TypeError exception.
    if !ops::is_callable(agent, target) {
        return Err(agent.throw_type_error("Reflect.apply target is not callable"));
    }
    // 2. Let args be ? CreateListFromArrayLike(argumentsList).
    let list = ops::create_list_from_array_like(agent, args.get_or_undefined(2), false)?;
    // 4. Return ? Call(target, thisArgument, args).
    ops::call(agent, target, args.get_or_undefined(1), &list)
}
