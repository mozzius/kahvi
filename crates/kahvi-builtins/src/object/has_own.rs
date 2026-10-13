//! Object.hasOwn ( _obj_, _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.hasown

use kahvi_core::prelude::*;

/// Object.hasOwn
pub(crate) fn has_own(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _coerced_ be ? ToObject(_obj_).
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    // 3. Return ? HasOwnProperty(_coerced_, _propertyKey_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.hasOwn"))
}
