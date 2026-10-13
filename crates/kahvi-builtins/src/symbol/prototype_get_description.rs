//! get Symbol.prototype.description
//!
//! Spec: https://tc39.es/ecma262/#sec-symbol.prototype.description

use kahvi_core::prelude::*;

/// get Symbol.prototype.description
pub(crate) fn prototype_get_description(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `Symbol.prototype.description` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _symbol_ be ? ThisSymbolValue(*this* value).
    // 2. Return _symbol_.[[Description]].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get Symbol.prototype.description"))
}
