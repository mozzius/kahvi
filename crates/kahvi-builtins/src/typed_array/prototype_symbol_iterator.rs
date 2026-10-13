//! %TypedArray%.prototype [ %Symbol.iterator% ] ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype-%symbol.iterator%

use kahvi_core::prelude::*;

/// TypedArray.prototype[Symbol.iterator]
pub(crate) fn prototype_symbol_iterator(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The initial value of the %Symbol.iterator% property is %TypedArray.prototype.values%, defined in [sec-%typedarray%.prototype.values].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype[Symbol.iterator]"))
}
