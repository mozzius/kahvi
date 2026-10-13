//! %TypedArray% ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%

use kahvi_core::prelude::*;

/// Intrinsic.TypedArray
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Throw a *TypeError* exception.
    //
    // The *"length"* property of this function is *+0*_𝔽.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intrinsic.TypedArray"))
}
