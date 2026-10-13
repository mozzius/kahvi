//! get ArrayBuffer [ %Symbol.species% ]
//!
//! Spec: https://tc39.es/ecma262/#sec-get-arraybuffer-%symbol.species%

use kahvi_core::prelude::*;

/// get ArrayBuffer[Symbol.species]
pub(crate) fn get_symbol_species(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `ArrayBuffer[%Symbol.species%]` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Return the *this* value.
    //
    // The value of the *"name"* property of this function is *"get [Symbol.species]"*.
    // NOTE:
    // [sec-arraybuffer.prototype.slice] normally uses its *this* value's constructor to create a derived object. However, a subclass constructor may over-ride that default behaviour for the [sec-arraybuffer.prototype.slice] method by redefining its %Symbol.species% property.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get ArrayBuffer[Symbol.species]"))
}
