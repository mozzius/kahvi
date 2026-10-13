//! %TypedArray%.prototype.entries ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.entries

use kahvi_core::prelude::*;

/// TypedArray.prototype.entries
pub(crate) fn prototype_entries(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 3. Return CreateArrayIterator(_obj_, ~key+value~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.entries"))
}
