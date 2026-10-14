//! %TypedArray% ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%

use kahvi_core::prelude::*;

/// Intrinsic.TypedArray
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Throw a TypeError exception.
    Err(agent.throw_type_error("%TypedArray% is not a constructor"))
}
