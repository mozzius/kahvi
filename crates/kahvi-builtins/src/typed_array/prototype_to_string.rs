//! %TypedArray%.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.tostring

use kahvi_core::prelude::*;

/// TypedArray.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The initial value of the *"toString"* property is %Array.prototype.toString%, defined in [sec-array.prototype.tostring].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.toString"))
}
