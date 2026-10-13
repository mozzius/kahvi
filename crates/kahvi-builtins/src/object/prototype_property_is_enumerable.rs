//! Object.prototype.propertyIsEnumerable ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.prototype.propertyisenumerable

use kahvi_core::prelude::*;

/// Object.prototype.propertyIsEnumerable
pub(crate) fn prototype_property_is_enumerable(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _propertyKey_ be ? ToPropertyKey(_value_).
    // 2. Let _obj_ be ? ToObject(*this* value).
    // 3. Let _propertyDesc_ be ? _obj_.[[GetOwnProperty]](_propertyKey_).
    // 4. If _propertyDesc_ is *undefined*, return *false*.
    // 5. Return _propertyDesc_.[[Enumerable]].
    //
    // NOTE:
    // This method does not consider objects in the prototype chain.
    // NOTE:
    // The ordering of steps [step-propertyisenumerable-topropertykey] and [step-propertyisenumerable-toobject] is chosen to ensure that any exception that would have been thrown by step [step-propertyisenumerable-topropertykey] in previous editions of this specification will continue to be thrown even if the *this* value is *undefined* or *null*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.prototype.propertyIsEnumerable"))
}
