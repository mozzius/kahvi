//! %GeneratorPrototype%.next ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-generator.prototype.next

use kahvi_core::prelude::*;
use kahvi_core::vm;

/// GeneratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Return ? GeneratorResume(*this* value, _value_, ~empty~).
    vm::generator_resume(agent, this, args.get_or_undefined(0))
}
