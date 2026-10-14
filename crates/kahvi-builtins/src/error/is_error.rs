//! Error.isError ( _arg_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-error.iserror

use kahvi_core::prelude::*;

/// Error.isError
pub(crate) fn is_error(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _arg_ is not an Object, return *false*.
    // 2. If _arg_ does not have an [[ErrorData]] internal slot, return *false*.
    // 3. Return *true*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Error.isError"))
}
