//! Object.getPrototypeOf ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.getprototypeof

use kahvi_core::prelude::*;

/// Object.getPrototypeOf
pub(crate) fn get_prototype_of(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let obj be ? ToObject(O).
    let obj = ops::to_object(agent, args.get_or_undefined(0))?;
    // 2. Return ? obj.[[GetPrototypeOf]]().
    Ok(obj
        .internal_get_prototype_of(agent)?
        .map(Value::Object)
        .unwrap_or(Value::Null))
}
