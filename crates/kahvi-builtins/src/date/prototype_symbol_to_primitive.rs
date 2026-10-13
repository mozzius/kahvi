//! Date.prototype [ %Symbol.toPrimitive% ] ( _hint_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype-%symbol.toprimitive%

use kahvi_core::prelude::*;

/// Date.prototype[Symbol.toPrimitive]
pub(crate) fn prototype_symbol_to_primitive(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method is called by ECMAScript language operators to convert a Date to a primitive value. The allowed values for _hint_ are *"default"*, *"number"*, and *"string"*. Dates are unique among built-in ECMAScript object in that they treat *"default"* as being equivalent to *"string"*, All other built-in ECMAScript objects treat *"default"* as being equivalent to *"number"*.
    // It performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. If _obj_ is not an Object, throw a *TypeError* exception.
    // 3. If _hint_ is either *"string"* or *"default"*, then
    //    a. Let _tryFirst_ be ~string~.
    // 4. Else if _hint_ is *"number"*, then
    //    a. Let _tryFirst_ be ~number~.
    // 5. Else,
    //    a. Throw a *TypeError* exception.
    // 6. Return ? OrdinaryToPrimitive(_obj_, _tryFirst_).
    //
    // This property has the attributes { [[Writable]]: *false*, [[Enumerable]]: *false*, [[Configurable]]: *true* }.
    // The value of the *"name"* property of this method is *"[Symbol.toPrimitive]"*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype[Symbol.toPrimitive]"))
}
