//! Error ( _message_ [ , _options_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-error-message

use crate::support::error::construct_error;
use kahvi_core::prelude::*;

/// Error
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    construct_error(agent, args, new_target, "%Error.prototype%")
}
