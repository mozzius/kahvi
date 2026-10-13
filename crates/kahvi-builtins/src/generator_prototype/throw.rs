//! %GeneratorPrototype%.throw ( _exception_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-generator.prototype.throw

use kahvi_core::prelude::*;

/// GeneratorPrototype.throw
pub(crate) fn throw(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _gen_ be the *this* value.
    // 2. Let _completion_ be ThrowCompletion(_exception_).
    // 3. Return ? GeneratorResumeAbrupt(_gen_, _completion_, ~empty~).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("GeneratorPrototype.throw"))
}
