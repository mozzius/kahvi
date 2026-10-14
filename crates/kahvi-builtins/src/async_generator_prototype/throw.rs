//! %AsyncGeneratorPrototype%.throw ( _exception_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncgenerator-prototype-throw

use kahvi_core::prelude::*;
use kahvi_core::vm;

/// AsyncGeneratorPrototype.throw
pub(crate) fn throw(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1-11. AsyncGeneratorValidate, then reject or AsyncGeneratorEnqueue and AsyncGeneratorResume.
    vm::async_generator_throw(agent, this, args.get_or_undefined(0))
}
