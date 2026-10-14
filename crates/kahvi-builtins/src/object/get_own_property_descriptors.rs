//! Object.getOwnPropertyDescriptors ( _obj_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.getownpropertydescriptors

use kahvi_core::prelude::*;

/// Object.getOwnPropertyDescriptors
pub(crate) fn get_own_property_descriptors(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _coerced_ be ? ToObject(_obj_).
    // 2. Let _ownKeys_ be ? _coerced_.[[OwnPropertyKeys]]().
    // 3. Let _descs_ be OrdinaryObjectCreate(%Object.prototype%).
    // 4. For each element _key_ of _ownKeys_, do
    //    a. Let _propertyDesc_ be ? _coerced_.[[GetOwnProperty]](_key_).
    //    b. Let _propertyDescObj_ be FromPropertyDescriptor(_propertyDesc_).
    //    c. If _propertyDescObj_ is not *undefined*, perform ! CreateDataPropertyOrThrow(_descs_, _key_, _propertyDescObj_).
    // 5. Return _descs_.
    let _ = (this, new_target);
    // 1. Let _coerced_ be ? ToObject(_obj_).
    let coerced = ops::to_object(agent, args.get_or_undefined(0))?;
    // 2. Let _ownKeys_ be ? _coerced_.[[OwnPropertyKeys]]().
    let own_keys = coerced.internal_own_property_keys(agent)?;
    // The keys, _coerced_ and _descs_ stay rooted while the descriptors are built.
    agent.with_roots(&[Value::Object(coerced)], |agent| {
        for key in &own_keys {
            agent.push_root_key(*key);
        }
        // 3. Let _descs_ be OrdinaryObjectCreate(%Object.prototype%).
        let proto = agent.intrinsic("%Object.prototype%");
        let descs = agent.ordinary_object_create(Some(proto));
        agent.push_root(Value::Object(descs));
        // 4. For each element _key_ of _ownKeys_, do
        for key in own_keys {
            // a. Let _propertyDesc_ be ? _coerced_.[[GetOwnProperty]](_key_).
            let property_desc = coerced.internal_get_own_property(agent, key)?;
            // b. Let _propertyDescObj_ be FromPropertyDescriptor(_propertyDesc_).
            let property_desc_obj = ops::from_property_descriptor(agent, property_desc);
            // c. If _propertyDescObj_ is not *undefined*, perform ! CreateDataPropertyOrThrow(_descs_, _key_, _propertyDescObj_).
            if let Value::Object(property_desc_obj) = property_desc_obj {
                ops::create_data_property_or_throw(
                    agent,
                    descs,
                    key,
                    Value::Object(property_desc_obj),
                )?;
            }
        }
        // 5. Return _descs_.
        Ok(Value::Object(descs))
    })
}
