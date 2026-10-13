//! Symbol.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-symbol.prototype.tostring

use crate::support::this_value::{symbol_descriptive_string, this_symbol_value};
use kahvi_core::prelude::*;

/// Symbol.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let sym be ? ThisSymbolValue(this value).
    let sym = this_symbol_value(agent, this)?;
    // 2. Return SymbolDescriptiveString(sym).
    Ok(Value::String(symbol_descriptive_string(agent, sym)))
}
