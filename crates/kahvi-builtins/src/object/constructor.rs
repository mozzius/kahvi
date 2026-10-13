//! Object ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object-value

use kahvi_core::prelude::*;

/// Object
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is neither *undefined* nor the active function object, then
    //    a. Return ? OrdinaryCreateFromConstructor(NewTarget, *"%Object.prototype%"*).
    // 2. If _value_ is either *undefined* or *null*, return OrdinaryObjectCreate(%Object.prototype%).
    // 3. Return ! ToObject(_value_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object"))
}
