//! Object.values ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.values

use kahvi_core::prelude::*;

/// Object.values
pub(crate) fn values(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _coerced_ be ? ToObject(_obj_).
    // 2. Let _valueList_ be ? EnumerableOwnProperties(_coerced_, ~value~).
    // 3. Return CreateArrayFromList(_valueList_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.values"))
}
