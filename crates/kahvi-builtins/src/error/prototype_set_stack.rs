//! set Error.prototype.stack
//!
//! Spec: https://tc39.es/proposal-error-stack-accessor/#sec-set-error.prototype.stack

use kahvi_core::prelude::*;

use crate::support::iterator_helper::setter_that_ignores_prototype_properties;

/// set Error.prototype.stack
pub(crate) fn prototype_set_stack(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let v = args.first().copied().unwrap_or(Value::Undefined);
    // 1. Let E be the this value.
    // 2. If E is not an Object, throw a TypeError exception.
    if !matches!(this, Value::Object(_)) {
        return Err(agent.throw_type_error("Error.prototype.stack setter called on a non-object"));
    }
    // 3. If v is not a String, throw a TypeError exception.
    if !matches!(v, Value::String(_)) {
        return Err(agent.throw_type_error("Error.prototype.stack must be set to a string"));
    }
    // 4. Perform ? SetterThatIgnoresPrototypeProperties(this value, %Error.prototype%, "stack", v).
    let home = agent.intrinsic("%Error.prototype%");
    let key = agent.key("stack");
    setter_that_ignores_prototype_properties(agent, this, home, key, v)?;
    // 5. Return undefined.
    Ok(Value::Undefined)
}
