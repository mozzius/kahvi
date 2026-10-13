//! Object.prototype.isPrototypeOf ( _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.prototype.isprototypeof

use kahvi_core::prelude::*;

/// Object.prototype.isPrototypeOf
pub(crate) fn prototype_is_prototype_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. If _value_ is not an Object, return *false*.
    // 2. Let _obj_ be ? ToObject(*this* value).
    // 3. Repeat,
    //    a. Set _value_ to ? _value_.[[GetPrototypeOf]]().
    //    b. If _value_ is *null*, return *false*.
    //    c. If SameValue(_obj_, _value_) is *true*, return *true*.
    //
    // NOTE:
    // The ordering of steps [step-isprototypeof-check-object] and [step-isprototypeof-toobject] preserves the behaviour specified by previous editions of this specification for the case where _value_ is not an object and the *this* value is *undefined* or *null*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.prototype.isPrototypeOf"))
}
