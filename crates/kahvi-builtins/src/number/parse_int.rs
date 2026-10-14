//! Number.parseInt ( _string_, _radix_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-number.parseint

use kahvi_core::prelude::*;

/// Number.parseInt
pub(crate) fn parse_int(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The initial value of the *"parseInt"* property is %parseInt%.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Number.parseInt"))
}
