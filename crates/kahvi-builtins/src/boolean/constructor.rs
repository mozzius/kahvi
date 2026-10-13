//! Boolean ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-boolean-constructor-boolean-value

use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;

/// Boolean
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let b be ToBoolean(value).
    let b = ops::to_boolean(agent, args.get_or_undefined(0));
    // 2. If NewTarget is undefined, return b.
    let Some(new_target) = new_target else {
        return Ok(Value::Boolean(b));
    };
    // 3. Let O be ? OrdinaryCreateFromConstructor(NewTarget, "%Boolean.prototype%", « [[BooleanData]] »).
    // 4. Set O.[[BooleanData]] to b.
    let o = ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%Boolean.prototype%",
        ObjectKind::BooleanObject(b),
    )?;
    // 5. Return O.
    Ok(Value::Object(o))
}
