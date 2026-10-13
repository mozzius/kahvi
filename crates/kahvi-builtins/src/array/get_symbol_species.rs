//! get Array [ %Symbol.species% ]
//!
//! Spec: https://tc39.es/ecma262/#sec-get-array-%symbol.species%

use kahvi_core::prelude::*;

/// get Array[Symbol.species]
pub(crate) fn get_symbol_species(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `Array[%Symbol.species%]` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Return the *this* value.
    //
    // The value of the *"name"* property of this function is *"get [Symbol.species]"*.
    // NOTE:
    // Array prototype methods normally use their *this* value's constructor to create a derived object. However, a subclass constructor may over-ride that default behaviour by redefining its %Symbol.species% property.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get Array[Symbol.species]"))
}
