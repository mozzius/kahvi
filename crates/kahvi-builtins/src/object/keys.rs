//! Object.keys ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.keys

use kahvi_core::prelude::*;

/// Object.keys
pub(crate) fn keys(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let obj be ? ToObject(O).
    let obj = ops::to_object(agent, args.get_or_undefined(0))?;
    // 2. Let keyList be ? EnumerableOwnProperties(obj, key).
    let key_list = ops::enumerable_own_properties(agent, obj, ops::EnumerableKind::Key)?;
    // 3. Return CreateArrayFromList(keyList).
    Ok(Value::Object(ops::create_array_from_list(agent, &key_list)))
}
