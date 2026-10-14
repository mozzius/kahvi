//! %AsyncGeneratorPrototype%.next ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncgenerator-prototype-next

use kahvi_core::prelude::*;
use kahvi_core::vm;

/// AsyncGeneratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1-11. AsyncGeneratorValidate, AsyncGeneratorEnqueue and AsyncGeneratorResume.
    vm::async_generator_next(agent, this, args.get_or_undefined(0))
}
