//! Object.prototype.__lookupGetter__ ( _key_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.prototype.__lookupGetter__

use kahvi_core::prelude::*;

/// Object.prototype.__lookupGetter__
pub(crate) fn prototype_lookup_getter(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    let obj = ops::to_object(agent, this)?;
    // 2. Let _propertyKey_ be ? ToPropertyKey(_key_).
    let property_key = ops::to_property_key(agent, args.get_or_undefined(0))?;
    // The lookup may run user code (proxy traps), so the object and key stay rooted.
    let key_value = agent.key_to_value(property_key);
    agent.with_roots(&[Value::Object(obj), key_value], |agent| {
        // 3. Repeat,
        let mut obj = obj;
        loop {
            // a. Let _propertyDesc_ be ? _obj_.[[GetOwnProperty]](_propertyKey_).
            let property_desc = obj.internal_get_own_property(agent, property_key)?;
            // b. If _propertyDesc_ is not *undefined*, then
            if let Some(property_desc) = property_desc {
                // i. If IsAccessorDescriptor(_propertyDesc_) is *true*, return _propertyDesc_.[[Getter]].
                if property_desc.is_accessor_descriptor() {
                    return Ok(property_desc.get.unwrap_or(Value::Undefined));
                }
                // ii. Return *undefined*.
                return Ok(Value::Undefined);
            }
            // c. Set _obj_ to ? _obj_.[[GetPrototypeOf]]().
            match obj.internal_get_prototype_of(agent)? {
                // d. If _obj_ is *null*, return *undefined*.
                None => return Ok(Value::Undefined),
                Some(proto) => obj = proto,
            }
        }
    })
}
