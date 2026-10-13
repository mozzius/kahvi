//! Symbol.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-symbol.prototype.tostring

use kahvi_core::prelude::*;

/// Symbol.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _symbol_ be ? ThisSymbolValue(*this* value).
    // 2. Return SymbolDescriptiveString(_symbol_).
    //
    // [subclause sec-symboldescriptivestring: SymbolDescriptiveString (
    // _symbol_: a Symbol,
    // ): a String]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Symbol.prototype.toString"))
}
