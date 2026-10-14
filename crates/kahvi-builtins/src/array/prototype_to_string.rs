//! Array.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.tostring

use kahvi_core::prelude::*;

/// Array.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _array_ be ? ToObject(*this* value).
    // 2. Let _func_ be ? Get(_array_, *"join"*).
    // 3. If IsCallable(_func_) is *false*, set _func_ to the intrinsic function %Object.prototype.toString%.
    // 4. Return ? Call(_func_, _array_).
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    // 1. Let _array_ be ? ToObject(*this* value).
    let array = ops::to_object(agent, this)?;
    // 2. Let _func_ be ? Get(_array_, *"join"*).
    let join = agent.key("join");
    let mut func = ops::get(agent, array, join)?;
    // 3. If IsCallable(_func_) is *false*, set _func_ to the intrinsic function %Object.prototype.toString%.
    if !ops::is_callable(agent, func) {
        func = Value::Object(agent.intrinsic("%Object.prototype.toString%"));
    }
    // 4. Return ? Call(_func_, _array_).
    ops::call(agent, func, Value::Object(array), &[])
}
