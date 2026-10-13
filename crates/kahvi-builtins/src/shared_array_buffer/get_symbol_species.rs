//! get SharedArrayBuffer [ %Symbol.species% ]
//!
//! Spec: https://tc39.es/ecma262/#sec-sharedarraybuffer-%symbol.species%

use kahvi_core::prelude::*;

/// get SharedArrayBuffer[Symbol.species]
pub(crate) fn get_symbol_species(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `SharedArrayBuffer[%Symbol.species%]` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Return the *this* value.
    //
    // The value of the *"name"* property of this function is *"get [Symbol.species]"*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get SharedArrayBuffer[Symbol.species]"))
}
