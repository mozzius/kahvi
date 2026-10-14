//! Reflect.ownKeys ( _target_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-reflect.ownkeys

use kahvi_core::prelude::*;

/// Reflect.ownKeys
pub(crate) fn own_keys(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If _target_ is not an Object, throw a *TypeError* exception.
    let target = match args.get_or_undefined(0) {
        Value::Object(o) => o,
        _ => {
            return Err(agent.throw_type_error("Reflect.ownKeys target is not an object"));
        }
    };
    // 2. Let _keys_ be ? _target_.[[OwnPropertyKeys]]().
    let keys = target.internal_own_property_keys(agent)?;
    // 3. Return CreateArrayFromList(_keys_).
    let values: Vec<Value> = keys.into_iter().map(|k| agent.key_to_value(k)).collect();
    Ok(Value::Object(ops::create_array_from_list(agent, &values)))
}
