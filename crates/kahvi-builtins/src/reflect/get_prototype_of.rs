//! Reflect.getPrototypeOf ( _target_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.getprototypeof

use kahvi_core::prelude::*;

/// Reflect.getPrototypeOf
pub(crate) fn get_prototype_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    // 2. Return ? _target_.[[GetPrototypeOf]]().
    let _ = (this, new_target);
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    let Value::Object(target) = args.get_or_undefined(0) else {
        return Err(agent.throw_type_error("Reflect.getPrototypeOf called on non-object"));
    };
    // 2. Return ? _target_.[[GetPrototypeOf]]().
    let proto = target.internal_get_prototype_of(agent)?;
    Ok(match proto {
        Some(p) => Value::Object(p),
        None => Value::Null,
    })
}
