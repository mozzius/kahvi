//! Object.hasOwn ( _obj_, _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.hasown

use kahvi_core::prelude::*;

/// Object.hasOwn
pub(crate) fn has_own(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _coerced_ be ? ToObject(_obj_).
    let coerced = ops::to_object(agent, args.get_or_undefined(0))?;
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    let property_key = ops::to_property_key(agent, args.get_or_undefined(1))?;
    // 3. Return ? HasOwnProperty(_coerced_, _propertyKey_).
    let has = ops::has_own_property(agent, coerced, property_key)?;
    Ok(Value::Boolean(has))
}
