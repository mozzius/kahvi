//! Object.create ( _proto_, _properties_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.create

use kahvi_core::prelude::*;

/// Object.create
pub(crate) fn create(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function creates a new object with a specified prototype.
    // It performs the following steps when called:
    //
    // 1. If _proto_ is not an Object and _proto_ is not *null*, throw a *TypeError* exception.
    // 2. Let _obj_ be OrdinaryObjectCreate(_proto_).
    // 3. If _properties_ is not *undefined*, then
    //    a. Return ? ObjectDefineProperties(_obj_, _properties_).
    // 4. Return _obj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.create"))
}
