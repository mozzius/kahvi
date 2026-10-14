//! Number.isFinite ( _number_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-number.isfinite

use kahvi_core::prelude::*;

/// Number.isFinite
pub(crate) fn is_finite(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _number_ is a finite Number, return *true*.
    // 2. Return *false*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Number.isFinite"))
}
