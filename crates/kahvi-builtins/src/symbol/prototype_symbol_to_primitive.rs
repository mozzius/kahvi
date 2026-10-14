//! Symbol.prototype [ %Symbol.toPrimitive% ] ( _hint_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-symbol.prototype-%symbol.toprimitive%

use kahvi_core::prelude::*;

use crate::support::this_value::this_symbol_value;

/// Symbol.prototype[Symbol.toPrimitive]
pub(crate) fn prototype_symbol_to_primitive(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method is called by ECMAScript language operators to convert a Symbol object to a primitive value.
    // It performs the following steps when called:
    //
    // 1. Return ? ThisSymbolValue(*this* value).
    //
    // NOTE:
    // The argument is ignored.
    // This property has the attributes { [[Writable]]: *false*, [[Enumerable]]: *false*, [[Configurable]]: *true* }.
    // The value of the *"name"* property of this method is *"[Symbol.toPrimitive]"*.
    Ok(Value::Symbol(this_symbol_value(agent, this)?))
}
