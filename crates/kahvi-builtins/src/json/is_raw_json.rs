//! JSON.isRawJSON ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-json.israwjson

use kahvi_core::prelude::*;

/// JSON.isRawJSON
pub(crate) fn is_raw_json(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _obj_ is an Object and _obj_ has an [[IsRawJSON]] internal slot, return *true*.
    // 2. Return *false*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("JSON.isRawJSON"))
}
