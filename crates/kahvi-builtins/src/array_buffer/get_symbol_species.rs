//! get ArrayBuffer [ %Symbol.species% ]
//!
//! Spec: https://tc39.es/ecma262/#sec-get-arraybuffer-%symbol.species%

use kahvi_core::prelude::*;

/// get ArrayBuffer[Symbol.species]
pub(crate) fn get_symbol_species(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return the this value.
    let _ = agent;
    Ok(this)
}
