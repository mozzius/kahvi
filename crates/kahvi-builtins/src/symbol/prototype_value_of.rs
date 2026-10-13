//! Symbol.prototype.valueOf ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-symbol.prototype.valueof

use crate::support::this_value::this_symbol_value;
use kahvi_core::prelude::*;

/// Symbol.prototype.valueOf
pub(crate) fn prototype_value_of(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? ThisSymbolValue(this value).
    Ok(Value::Symbol(this_symbol_value(agent, this)?))
}
