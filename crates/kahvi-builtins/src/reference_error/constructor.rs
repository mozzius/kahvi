//! ReferenceError ( _message_ [ , _options_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-nativeerror

use crate::support::error::construct_error;
use kahvi_core::prelude::*;

/// ReferenceError
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    construct_error(agent, args, new_target, "%ReferenceError.prototype%")
}
