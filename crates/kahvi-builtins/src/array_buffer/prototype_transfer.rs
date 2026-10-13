//! ArrayBuffer.prototype.transfer ( [ _newLength_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-arraybuffer.prototype.transfer

use kahvi_core::prelude::*;

/// ArrayBuffer.prototype.transfer
pub(crate) fn prototype_transfer(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Return ? ArrayBufferCopyAndDetach(_obj_, _newLength_, ~preserve-resizability~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("ArrayBuffer.prototype.transfer"))
}
