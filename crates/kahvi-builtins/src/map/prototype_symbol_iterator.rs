//! Map.prototype [ %Symbol.iterator% ] ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-map.prototype-%symbol.iterator%

use kahvi_core::prelude::*;

/// Map.prototype[Symbol.iterator]
pub(crate) fn prototype_symbol_iterator(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The initial value of the %Symbol.iterator% property is %Map.prototype.entries%, defined in [sec-map.prototype.entries].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Map.prototype[Symbol.iterator]"))
}
