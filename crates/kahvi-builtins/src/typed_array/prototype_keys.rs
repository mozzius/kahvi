//! %TypedArray%.prototype.keys ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.keys

use kahvi_core::prelude::*;

/// TypedArray.prototype.keys
pub(crate) fn prototype_keys(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 3. Return CreateArrayIterator(_obj_, ~key~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.keys"))
}
