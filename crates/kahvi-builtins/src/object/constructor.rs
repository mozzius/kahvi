//! Object ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object-value

use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;

/// Object
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let value = args.get_or_undefined(0);
    // 1. If NewTarget is neither undefined nor the active function object, then
    let active = agent.active_function();
    if let Some(nt) = new_target
        && Some(nt) != active
    {
        // a. Return ? OrdinaryCreateFromConstructor(NewTarget, "%Object.prototype%").
        let o = ops::ordinary_create_from_constructor(
            agent,
            nt,
            "%Object.prototype%",
            ObjectKind::Ordinary,
        )?;
        return Ok(Value::Object(o));
    }
    // 2. If value is either undefined or null, return OrdinaryObjectCreate(%Object.prototype%).
    if value.is_nullish() {
        let proto = agent.intrinsic("%Object.prototype%");
        return Ok(Value::Object(agent.ordinary_object_create(Some(proto))));
    }
    // 3. Return ! ToObject(value).
    Ok(Value::Object(ops::to_object(agent, value)?))
}
