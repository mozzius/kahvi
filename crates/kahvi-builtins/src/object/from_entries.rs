//! Object.fromEntries ( _iterable_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.fromentries

use kahvi_core::prelude::*;

/// Object.fromEntries
pub(crate) fn from_entries(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Perform ? RequireObjectCoercible(_iterable_).
    // 2. Let _obj_ be OrdinaryObjectCreate(%Object.prototype%).
    // 3. Assert: _obj_ is an extensible ordinary object with no own properties.
    // 4. Let _closure_ be a new Abstract Closure with parameters (_key_, _value_) that captures _obj_ and performs the following steps when called:
    //    a. Let _propertyKey_ be ? ToPropertyKey(_key_).
    //    b. Perform ! CreateDataPropertyOrThrow(_obj_, _propertyKey_, _value_).
    //    c. Return NormalCompletion(*undefined*).
    // 5. Let _adder_ be CreateBuiltinFunction(_closure_, 2, *""*, « »).
    // 6. Return ? AddEntriesFromIterable(_obj_, _iterable_, _adder_).
    //
    // NOTE:
    // The function created for _adder_ is never directly accessible to ECMAScript code.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.fromEntries"))
}
