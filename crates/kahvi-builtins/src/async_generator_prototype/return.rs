//! %AsyncGeneratorPrototype%.return ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-asyncgenerator-prototype-return

use kahvi_core::prelude::*;
use kahvi_core::vm;

/// AsyncGeneratorPrototype.return
pub(crate) fn r#return(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1-10. AsyncGeneratorValidate, AsyncGeneratorEnqueue, then AsyncGeneratorAwaitReturn or AsyncGeneratorResume.
    vm::async_generator_return(agent, this, args.get_or_undefined(0))
}
