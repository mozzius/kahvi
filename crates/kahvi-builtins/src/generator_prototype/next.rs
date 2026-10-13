//! %GeneratorPrototype%.next ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-generator.prototype.next

use kahvi_core::prelude::*;

/// GeneratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? GeneratorResume(*this* value, _value_, ~empty~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("GeneratorPrototype.next"))
}
