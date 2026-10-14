//! Object.isExtensible ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.isextensible

use kahvi_core::prelude::*;

/// Object.isExtensible
pub(crate) fn is_extensible(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _obj_ is not an Object, return *false*.
    // 2. Return ? IsExtensible(_obj_).
    let _ = (this, new_target);
    // 1. If _obj_ is not an Object, return *false*.
    let Value::Object(obj) = args.get_or_undefined(0) else {
        return Ok(Value::Boolean(false));
    };
    // 2. Return ? IsExtensible(_obj_).
    let extensible = ops::is_extensible(agent, obj)?;
    Ok(Value::Boolean(extensible))
}
