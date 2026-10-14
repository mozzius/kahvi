//! get %TypedArray%.prototype [ %Symbol.toStringTag% ]
//!
//! Spec: https://tc39.es/ecma262/#sec-get-%typedarray%.prototype-%symbol.tostringtag%

use kahvi_core::prelude::*;

use kahvi_core::object::typed_array::typed_array_data;

/// get TypedArray.prototype[Symbol.toStringTag]
pub(crate) fn prototype_get_symbol_to_string_tag(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let O be the this value.
    // 2. If O is not an Object, return undefined.
    let Value::Object(o) = this else {
        return Ok(Value::Undefined);
    };
    // 3. If O does not have a [[TypedArrayName]] internal slot, return undefined.
    let Some(d) = typed_array_data(agent, o) else {
        return Ok(Value::Undefined);
    };
    // 4-6. Return O.[[TypedArrayName]].
    let name = agent.string(d.kind.name());
    Ok(Value::String(name))
}
