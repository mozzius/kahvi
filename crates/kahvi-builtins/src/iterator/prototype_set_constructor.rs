//! set Iterator.prototype.constructor
//!
//! Spec: https://tc39.es/ecma262/#sec-set-iterator.prototype.constructor

use kahvi_core::prelude::*;

/// set Iterator.prototype.constructor
pub(crate) fn prototype_set_constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The value of the [[Setter]] attribute is a built-in function that takes an argument _v_. It performs the following steps when called:
    //
    // 1. Perform ? SetterThatIgnoresPrototypeProperties(*this* value, %Iterator.prototype%, *"constructor"*, _v_).
    // 2. Return *undefined*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("set Iterator.prototype.constructor"))
}
