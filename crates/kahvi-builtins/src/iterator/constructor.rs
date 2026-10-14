//! Iterator ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator

use kahvi_core::prelude::*;

/// Iterator
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    _args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. If NewTarget is either undefined or the active function object, throw a TypeError exception.
    let active = agent.active_function();
    let Some(new_target) = new_target.filter(|nt| Some(*nt) != active) else {
        return Err(agent.throw_type_error("Iterator is an abstract class"));
    };
    // 2. Return ? OrdinaryCreateFromConstructor(NewTarget, "%Iterator.prototype%").
    let o = ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%Iterator.prototype%",
        ObjectKind::Ordinary,
    )?;
    Ok(Value::Object(o))
}
