//! ArrayBuffer.prototype.transferToFixedLength ( [ _newLength_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-arraybuffer.prototype.transfertofixedlength

use kahvi_core::prelude::*;

/// ArrayBuffer.prototype.transferToFixedLength
pub(crate) fn prototype_transfer_to_fixed_length(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Return ? ArrayBufferCopyAndDetach(_obj_, _newLength_, ~fixed-length~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("ArrayBuffer.prototype.transferToFixedLength"))
}
