//! Array.prototype [ %Symbol.iterator% ] ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype-%symbol.iterator%

use kahvi_core::prelude::*;

/// Array.prototype[Symbol.iterator]
pub(crate) fn prototype_symbol_iterator(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The initial value of the %Symbol.iterator% property is %Array.prototype.values%, defined in [sec-array.prototype.values].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype[Symbol.iterator]"))
}
