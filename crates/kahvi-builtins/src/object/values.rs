//! Object.values ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.values

use kahvi_core::prelude::*;

/// Object.values
pub(crate) fn values(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _coerced_ be ? ToObject(_obj_).
    let coerced = ops::to_object(agent, args.get_or_undefined(0))?;
    // 2. Let _valueList_ be ? EnumerableOwnProperties(_coerced_, ~value~).
    let value_list = ops::enumerable_own_properties(agent, coerced, ops::EnumerableKind::Value)?;
    // 3. Return CreateArrayFromList(_valueList_).
    Ok(Value::Object(ops::create_array_from_list(
        agent,
        &value_list,
    )))
}
