//! SharedArrayBuffer ( _length_ [ , _options_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-sharedarraybuffer-length

use kahvi_core::prelude::*;

/// SharedArrayBuffer
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. Let _byteLength_ be ? ToIndex(_length_).
    // 3. Let _requestedMaxByteLength_ be ? GetArrayBufferMaxByteLengthOption(_options_).
    // 4. Return ? AllocateSharedArrayBuffer(NewTarget, _byteLength_, _requestedMaxByteLength_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("SharedArrayBuffer"))
}
