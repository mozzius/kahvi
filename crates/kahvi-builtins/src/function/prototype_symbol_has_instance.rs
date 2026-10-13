//! Function.prototype [ %Symbol.hasInstance% ] ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-function.prototype-%symbol.hasinstance%

use kahvi_core::prelude::*;

/// Function.prototype[Symbol.hasInstance]
pub(crate) fn prototype_symbol_has_instance(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Return ? OrdinaryHasInstance(_thisValue_, _value_).
    //
    // This property has the attributes { [[Writable]]: *false*, [[Enumerable]]: *false*, [[Configurable]]: *false* }.
    // NOTE:
    // This is the default implementation of `%Symbol.hasInstance%` that most functions inherit. `%Symbol.hasInstance%` is called by the `instanceof` operator to determine whether a value is an instance of a specific constructor. An expression such as
    // v instanceof F
    // evaluates as
    // F[%Symbol.hasInstance%](v)
    // A constructor function can control which objects are recognized as its instances by `instanceof` by exposing a different `%Symbol.hasInstance%` method on the function.
    // This property is non-writable and non-configurable to prevent tampering that could be used to globally expose the target function of a bound function.
    // The value of the *"name"* property of this method is *"[Symbol.hasInstance]"*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Function.prototype[Symbol.hasInstance]"))
}
