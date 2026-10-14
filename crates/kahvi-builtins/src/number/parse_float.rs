//! Number.parseFloat ( _string_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-number.parsefloat

use kahvi_core::prelude::*;

/// Number.parseFloat
pub(crate) fn parse_float(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The initial value of the *"parseFloat"* property is %parseFloat%.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Number.parseFloat"))
}
