//! Object.prototype.__lookupGetter__ ( _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.prototype.__lookupGetter__

use kahvi_core::prelude::*;

/// Object.prototype.__lookupGetter__
pub(crate) fn prototype_lookup_getter(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    // 3. Repeat,
    //    a. Let _propertyDesc_ be ? _obj_.[[GetOwnProperty]](_propertyKey_).
    //    b. If _propertyDesc_ is not *undefined*, then
    //       i. If IsAccessorDescriptor(_propertyDesc_) is *true*, return _propertyDesc_.[[Getter]].
    //       ii. Return *undefined*.
    //    c. Set _obj_ to ? _obj_.[[GetPrototypeOf]]().
    //    d. If _obj_ is *null*, return *undefined*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.prototype.__lookupGetter__"))
}
