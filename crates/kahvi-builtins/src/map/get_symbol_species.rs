//! get Map [ %Symbol.species% ]
//!
//! Spec: https://tc39.es/ecma262/#sec-get-map-%symbol.species%

use kahvi_core::prelude::*;

/// get Map[Symbol.species]
pub(crate) fn get_symbol_species(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `Map[%Symbol.species%]` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Return the *this* value.
    //
    // The value of the *"name"* property of this function is *"get [Symbol.species]"*.
    // NOTE:
    // Methods that create derived collection objects should call %Symbol.species% to determine the constructor to use to create the derived objects. Subclass constructor may over-ride %Symbol.species% to change the default constructor assignment.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get Map[Symbol.species]"))
}
