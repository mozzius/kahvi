//! set Iterator.prototype.constructor
//!
//! Spec: https://tc39.es/ecma262/#sec-set-iterator.prototype.constructor

use kahvi_core::prelude::*;

use crate::support::iterator_helper::setter_that_ignores_prototype_properties;

/// set Iterator.prototype.constructor
pub(crate) fn prototype_set_constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The value of the [[Setter]] attribute is a built-in function that takes an argument _v_. It performs the following steps when called:
    //
    let _ = new_target;
    let v = args.first().copied().unwrap_or(Value::Undefined);
    // 1. Perform ? SetterThatIgnoresPrototypeProperties(*this* value, %Iterator.prototype%, *"constructor"*, _v_).
    let home = agent.intrinsic("%Iterator.prototype%");
    let key = PropertyKey::String(agent.names.constructor);
    setter_that_ignores_prototype_properties(agent, this, home, key, v)?;
    // 2. Return *undefined*.
    Ok(Value::Undefined)
}
