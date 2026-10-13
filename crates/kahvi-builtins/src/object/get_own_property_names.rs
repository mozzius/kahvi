//! Object.getOwnPropertyNames ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.getownpropertynames

use kahvi_core::prelude::*;

/// Object.getOwnPropertyNames
pub(crate) fn get_own_property_names(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Return CreateArrayFromList(? GetOwnPropertyKeys(_obj_, ~string~)).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.getOwnPropertyNames"))
}
