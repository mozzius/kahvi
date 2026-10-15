//! set Error.prototype.stack
//!
//! Spec: https://tc39.es/proposal-error-stack-accessor/#sec-set-error.prototype.stack

use kahvi_core::prelude::*;

/// set Error.prototype.stack
pub(crate) fn prototype_set_stack(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The value of the [[Set]] attribute is a built-in function that takes an argument _v_. It performs the following steps when called:
    //
    // 1. Let _E_ be the *this* value.
    // 2. If _E_ is not an Object, throw a *TypeError* exception.
    // 3. If _v_ is not a String, throw a *TypeError* exception.
    // 4. Perform ? SetterThatIgnoresPrototypeProperties(*this* value, %Error.prototype%, *"stack"*, _v_).
    // 5. Return *undefined*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("set Error.prototype.stack"))
}
