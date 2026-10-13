//! Object.getOwnPropertyNames ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.getownpropertynames

use kahvi_core::prelude::*;

/// Object.getOwnPropertyNames
pub(crate) fn get_own_property_names(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return CreateArrayFromList(? GetOwnPropertyKeys(O, string)).
    // GetOwnPropertyKeys: 1. Let obj be ? ToObject(O).
    let obj = ops::to_object(agent, args.get_or_undefined(0))?;
    // 2. Let keys be ? obj.[[OwnPropertyKeys]]().
    let keys = obj.internal_own_property_keys(agent)?;
    // 3-4. The keys that are Strings.
    let names: Vec<Value> = keys
        .into_iter()
        .filter(|k| !k.is_symbol())
        .map(|k| agent.key_to_value(k))
        .collect();
    Ok(Value::Object(ops::create_array_from_list(agent, &names)))
}
