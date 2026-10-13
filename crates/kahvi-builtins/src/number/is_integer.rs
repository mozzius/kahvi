//! Number.isInteger ( _number_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-number.isinteger

use kahvi_core::prelude::*;

/// Number.isInteger
pub(crate) fn is_integer(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _number_ is an integral Number, return *true*.
    // 2. Return *false*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Number.isInteger"))
}
