//! Object.prototype.hasOwnProperty ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.prototype.hasownproperty

use kahvi_core::prelude::*;

/// Object.prototype.hasOwnProperty
pub(crate) fn prototype_has_own_property(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let P be ? ToPropertyKey(V).
    let p = ops::to_property_key(agent, args.get_or_undefined(0))?;
    // 2. Let O be ? ToObject(this value).
    let o = ops::to_object(agent, this)?;
    // 3. Return ? HasOwnProperty(O, P).
    Ok(Value::Boolean(ops::has_own_property(agent, o, p)?))
}
