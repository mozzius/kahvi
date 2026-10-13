//! Date.prototype.toGMTString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.togmtstring

use kahvi_core::prelude::*;

/// Date.prototype.toGMTString
pub(crate) fn prototype_to_gmt_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // The `toUTCString` method is preferred. This method is provided principally for compatibility with old code.
    // The initial value of the *"toGMTString"* property is %Date.prototype.toUTCString%, defined in [sec-date.prototype.toutcstring].
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.toGMTString"))
}
