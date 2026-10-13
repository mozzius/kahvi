//! %GeneratorPrototype%.throw ( _exception_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-generator.prototype.throw

use kahvi_core::prelude::*;
use kahvi_core::vm;

/// GeneratorPrototype.throw
pub(crate) fn throw(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _gen_ be the *this* value.
    // 2. Let _completion_ be ThrowCompletion(_exception_).
    // 3. Return ? GeneratorResumeAbrupt(_gen_, _completion_, ~empty~).
    vm::generator_resume_abrupt(agent, this, args.get_or_undefined(0), false)
}
