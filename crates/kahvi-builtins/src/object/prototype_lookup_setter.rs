//! Object.prototype.__lookupSetter__ ( _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.prototype.__lookupSetter__

use kahvi_core::prelude::*;

/// Object.prototype.__lookupSetter__
pub(crate) fn prototype_lookup_setter(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    // 3. Repeat,
    //    a. Let _propertyDesc_ be ? _obj_.[[GetOwnProperty]](_propertyKey_).
    //    b. If _propertyDesc_ is not *undefined*, then
    //       i. If IsAccessorDescriptor(_propertyDesc_) is *true*, return _propertyDesc_.[[Setter]].
    //       ii. Return *undefined*.
    //    c. Set _obj_ to ? _obj_.[[GetPrototypeOf]]().
    //    d. If _obj_ is *null*, return *undefined*.
    // 1. Let _obj_ be ? ToObject(*this* value).
    let mut obj = ops::to_object(agent, this)?;
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    let property_key = ops::to_property_key(agent, args.get_or_undefined(0))?;
    // 3. Repeat,
    loop {
        // a. Let _propertyDesc_ be ? _obj_.[[GetOwnProperty]](_propertyKey_).
        let property_desc = obj.internal_get_own_property(agent, property_key)?;
        // b. If _propertyDesc_ is not *undefined*, then
        if let Some(property_desc) = property_desc {
            // i. If IsAccessorDescriptor(_propertyDesc_) is *true*, return _propertyDesc_.[[Setter]].
            if property_desc.is_accessor_descriptor() {
                return Ok(property_desc.set.unwrap_or(Value::Undefined));
            }
            // ii. Return *undefined*.
            return Ok(Value::Undefined);
        }
        // c. Set _obj_ to ? _obj_.[[GetPrototypeOf]]().
        let Some(proto) = obj.internal_get_prototype_of(agent)? else {
            // d. If _obj_ is *null*, return *undefined*.
            return Ok(Value::Undefined);
        };
        obj = proto;
    }
}
