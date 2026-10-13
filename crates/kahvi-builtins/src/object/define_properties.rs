//! Object.defineProperties ( _obj_, _properties_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.defineproperties

use kahvi_core::prelude::*;

/// Object.defineProperties
pub(crate) fn define_properties(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function adds own properties and/or updates the attributes of existing own properties of an object.
    // It performs the following steps when called:
    //
    // 1. If _obj_ is not an Object, throw a *TypeError* exception.
    // 2. Return ? ObjectDefineProperties(_obj_, _properties_).
    //
    // [subclause sec-objectdefineproperties: ObjectDefineProperties (
    // _obj_: an Object,
    // _properties_: an ECMAScript language value,
    // ): either a normal completion containing an Object or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.defineProperties"))
}
