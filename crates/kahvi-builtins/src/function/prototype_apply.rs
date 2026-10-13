//! Function.prototype.apply ( _thisArg_, _argArray_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-function.prototype.apply

use kahvi_core::prelude::*;

/// Function.prototype.apply
pub(crate) fn prototype_apply(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let func be the this value.
    let func = this;
    // 2. If IsCallable(func) is false, throw a TypeError exception.
    if !ops::is_callable(agent, func) {
        return Err(agent.throw_type_error("Function.prototype.apply called on a non-function"));
    }
    let this_arg = args.get_or_undefined(0);
    let arg_array = args.get_or_undefined(1);
    // 3. If argArray is either undefined or null, then
    if arg_array.is_nullish() {
        // b. Return ? Call(func, thisArg).
        return ops::call(agent, func, this_arg, &[]);
    }
    // 4. Let argList be ? CreateListFromArrayLike(argArray).
    let arg_list = ops::create_list_from_array_like(agent, arg_array, false)?;
    // 6. Return ? Call(func, thisArg, argList).
    ops::call(agent, func, this_arg, &arg_list)
}
