//! Object.prototype.toLocaleString ( [ _reserved1_ [ , _reserved2_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.prototype.tolocalestring

use kahvi_core::prelude::*;

/// Object.prototype.toLocaleString
pub(crate) fn prototype_to_locale_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Return ? Invoke(_thisValue_, *"toString"*).
    //
    // The optional parameters to this method are not used but are intended to correspond to the parameter pattern used by ECMA-402 `toLocaleString` methods. Implementations that do not include ECMA-402 support must not use those parameter positions for other purposes.
    // NOTE:
    // This method provides a generic `toLocaleString` implementation for objects that have no locale-sensitive `toString` behaviour. `Array`, `Number`, `Date`, and %TypedArray% provide their own locale-sensitive `toLocaleString` methods.
    // NOTE:
    // ECMA-402 intentionally does not provide an alternative to this default implementation.
    let _ = (args, new_target);
    // 1. Let _thisValue_ be the *this* value.
    let this_value = this;
    // 2. Return ? Invoke(_thisValue_, *"toString"*).
    let key = agent.key("toString");
    ops::invoke(agent, this_value, key, &[])
}
