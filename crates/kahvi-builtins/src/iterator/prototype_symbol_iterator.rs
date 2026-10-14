//! Iterator.prototype [ %Symbol.iterator% ] ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.prototype-%symbol.iterator%

use kahvi_core::prelude::*;

/// Iterator.prototype[Symbol.iterator]
pub(crate) fn prototype_symbol_iterator(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Return the *this* value.
    //
    // The value of the *"name"* property of this function is *"[Symbol.iterator]"*.
    let _ = (agent, args);
    Ok(this)
}
