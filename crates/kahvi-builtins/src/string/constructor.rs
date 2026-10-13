//! String ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string-constructor-string-value

use kahvi_core::prelude::*;

/// String
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If _value_ is not present, then
    //    a. Let _string_ be the empty String.
    // 2. Else,
    //    a. If NewTarget is *undefined* and _value_ is a Symbol, return SymbolDescriptiveString(_value_).
    //    b. Let _string_ be ? ToString(_value_).
    // 3. If NewTarget is *undefined*, return _string_.
    // 4. Return StringCreate(_string_, ? GetPrototypeFromConstructor(NewTarget, *"%String.prototype%"*)).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String"))
}
