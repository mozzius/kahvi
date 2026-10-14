//! DataView ( _buffer_ [ , _byteOffset_ [ , _byteLength_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-dataview-buffer-byteoffset-bytelength

use kahvi_core::prelude::*;

use crate::support::typed_array::data_view_constructor;

/// DataView
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The steps of DataView ( buffer [ , byteOffset [ , byteLength ] ] ) (§25.3.2.1).
    data_view_constructor(agent, args, new_target)
}
