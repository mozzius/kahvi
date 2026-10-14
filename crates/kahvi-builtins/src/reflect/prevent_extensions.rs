//! Reflect.preventExtensions ( _target_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.preventextensions

use kahvi_core::prelude::*;

/// Reflect.preventExtensions
pub(crate) fn prevent_extensions(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    let _ = (this, new_target);
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    let Value::Object(target) = args.get_or_undefined(0) else {
        return Err(agent.throw_type_error("Reflect.preventExtensions called on non-object"));
    };
    // 2. Return ? _target_.[[PreventExtensions]]().
    let result = target.internal_prevent_extensions(agent)?;
    Ok(Value::Boolean(result))
}
