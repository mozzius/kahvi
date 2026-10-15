//! get Error.prototype.stack
//!
//! Spec: https://tc39.es/proposal-error-stack-accessor/#sec-get-error.prototype.stack

use kahvi_core::prelude::*;

/// get Error.prototype.stack
pub(crate) fn prototype_get_stack(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The value of the [[Get]] attribute is a built-in function that requires no arguments. It performs the following steps when called:
    //
    // 1. Let _E_ be the *this* value.
    // 2. If _E_ is not an Object, throw a *TypeError* exception.
    // 3. If _E_ does not have an [[ErrorData]] internal slot, return *undefined*.
    // 4. Return an implementation-defined string that represents the stack trace of _E_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get Error.prototype.stack"))
}
