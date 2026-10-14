//! Object.prototype.propertyIsEnumerable ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.prototype.propertyisenumerable

use kahvi_core::prelude::*;

/// Object.prototype.propertyIsEnumerable
pub(crate) fn prototype_property_is_enumerable(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let P be ? ToPropertyKey(V).
    let p = ops::to_property_key(agent, args.get_or_undefined(0))?;
    // 2. Let O be ? ToObject(this value).
    let o = ops::to_object(agent, this)?;
    // 3. Let desc be ? O.[[GetOwnProperty]](P).
    // 4. If desc is undefined, return false.
    // 5. Return desc.[[Enumerable]].
    let desc = o.internal_get_own_property(agent, p)?;
    Ok(Value::Boolean(
        desc.is_some_and(|d| d.enumerable == Some(true)),
    ))
}
