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
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.getOwnPropertyDescriptors"))
}
