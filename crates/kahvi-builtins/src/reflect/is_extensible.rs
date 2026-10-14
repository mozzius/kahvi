//! Reflect.isExtensible ( _target_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.isextensible

use kahvi_core::prelude::*;

/// Reflect.isExtensible
pub(crate) fn is_extensible(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (this, new_target);
    // This function performs the following steps when called:
    //
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    let Value::Object(target) = args.get_or_undefined(0) else {
        return Err(agent.throw_type_error("Reflect.isExtensible called on non-object"));
    };
    // 2. Return ? _target_.[[IsExtensible]]().
    let extensible = ops::is_extensible(agent, target)?;
    Ok(Value::Boolean(extensible))
}
