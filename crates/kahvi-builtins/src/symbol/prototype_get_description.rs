//! get Symbol.prototype.description
//!
//! Spec: https://tc39.es/ecma262/#sec-symbol.prototype.description

use crate::support::this_value::this_symbol_value;
use kahvi_core::prelude::*;

/// get Symbol.prototype.description
pub(crate) fn prototype_get_description(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let s be the this value.
    // 2. Let sym be ? ThisSymbolValue(s).
    let sym = this_symbol_value(agent, this)?;
    // 3. Return sym.[[Description]].
    Ok(agent
        .symbol_description(sym)
        .map(Value::String)
        .unwrap_or(Value::Undefined))
}
