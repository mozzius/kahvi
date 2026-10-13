//! Symbol.keyFor ( _symbol_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-symbol.keyfor

use kahvi_core::prelude::*;

/// Symbol.keyFor
pub(crate) fn key_for(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _symbol_ is not a Symbol, throw a *TypeError* exception.
    // 2. Return KeyForSymbol(_symbol_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Symbol.keyFor"))
}
