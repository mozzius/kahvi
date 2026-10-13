//! get %TypedArray%.prototype [ %Symbol.toStringTag% ]
//!
//! Spec: https://tc39.es/ecma262/#sec-get-%typedarray%.prototype-%symbol.tostringtag%

use kahvi_core::prelude::*;

/// get TypedArray.prototype[Symbol.toStringTag]
pub(crate) fn prototype_get_symbol_to_string_tag(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // %TypedArray%`.prototype[%Symbol.toStringTag%]` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. If _obj_ is not an Object, return *undefined*.
    // 3. If _obj_ does not have a [[TypedArrayName]] internal slot, return *undefined*.
    // 4. Let _name_ be _obj_.[[TypedArrayName]].
    // 5. Assert: _name_ is a String.
    // 6. Return _name_.
    //
    // This property has the attributes { [[Enumerable]]: *false*, [[Configurable]]: *true* }.
    // The initial value of the *"name"* property of this function is *"get [Symbol.toStringTag]"*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("get TypedArray.prototype[Symbol.toStringTag]"))
}
