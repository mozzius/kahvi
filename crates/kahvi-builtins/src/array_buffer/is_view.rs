//! ArrayBuffer.isView ( _arg_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-arraybuffer.isview

use kahvi_core::prelude::*;

/// ArrayBuffer.isView
pub(crate) fn is_view(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _arg_ is not an Object, return *false*.
    // 2. If _arg_ has a [[ViewedArrayBuffer]] internal slot, return *true*.
    // 3. Return *false*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("ArrayBuffer.isView"))
}
