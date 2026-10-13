//! Object.prototype.hasOwnProperty ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.prototype.hasownproperty

use kahvi_core::prelude::*;

/// Object.prototype.hasOwnProperty
pub(crate) fn prototype_has_own_property(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _propertyKey_ be ? ToPropertyKey(_value_).
    // 2. Let _obj_ be ? ToObject(*this* value).
    // 3. Return ? HasOwnProperty(_obj_, _propertyKey_).
    //
    // NOTE:
    // The ordering of steps [step-hasownproperty-topropertykey] and [step-hasownproperty-toobject] is chosen to ensure that any exception that would have been thrown by step [step-hasownproperty-topropertykey] in previous editions of this specification will continue to be thrown even if the *this* value is *undefined* or *null*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.prototype.hasOwnProperty"))
}
