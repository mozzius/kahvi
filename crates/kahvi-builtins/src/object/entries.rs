//! Object.entries ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.entries

use kahvi_core::prelude::*;

/// Object.entries
pub(crate) fn entries(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _coerced_ be ? ToObject(_obj_).
    let coerced = ops::to_object(agent, args.get_or_undefined(0))?;
    // 2. Let _entryList_ be ? EnumerableOwnProperties(_coerced_, ~key+value~).
    let entry_list = ops::enumerable_own_properties(agent, coerced, ops::EnumerableKind::KeyValue)?;
    // 3. Return CreateArrayFromList(_entryList_).
    Ok(Value::Object(ops::create_array_from_list(
        agent,
        &entry_list,
    )))
}
