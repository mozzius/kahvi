//! Proxy [[DefineOwnProperty]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-defineownproperty-p-desc

#![allow(unused_imports)]

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::object::{PropertyDescriptor, PropertyKey};
use crate::value::{Object, Value};

/// The [[DefineOwnProperty]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn define_own_property(
    agent: &mut Agent,
    o: Object,
    p: &ProxyData,
    key: PropertyKey,
    desc: PropertyDescriptor,
) -> JsResult<bool> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"defineProperty"*).
    // 6. If _trap_ is *undefined*, then
    //    a. Return ? _target_.[[DefineOwnProperty]](_propertyKey_, _propertyDesc_).
    // 7. Let _propertyDescObj_ be FromPropertyDescriptor(_propertyDesc_).
    // 8. Let _boolTrapResult_ be ToBoolean(? Call(_trap_, _handler_, « _target_, _propertyKey_, _propertyDescObj_ »)).
    // 9. If _boolTrapResult_ is *false*, return *false*.
    // 10. Let _targetDesc_ be ? _target_.[[GetOwnProperty]](_propertyKey_).
    // 11. Let _extensibleTarget_ be ? IsExtensible(_target_).
    // 12. If _propertyDesc_ has a [[Configurable]] field and _propertyDesc_.[[Configurable]] is *false*, then
    //    a. Let _settingConfigurableFalse_ be *true*.
    // 13. Else,
    //    a. Let _settingConfigurableFalse_ be *false*.
    // 14. If _targetDesc_ is *undefined*, then
    //    a. If _extensibleTarget_ is *false*, throw a *TypeError* exception.
    //    b. If _settingConfigurableFalse_ is *true*, throw a *TypeError* exception.
    // 15. Else,
    //    a. If IsCompatiblePropertyDescriptor(_extensibleTarget_, _propertyDesc_, _targetDesc_) is *false*, throw a *TypeError* exception.
    //    b. If _settingConfigurableFalse_ is *true* and _targetDesc_.[[Configurable]] is *true*, throw a *TypeError* exception.
    //    c. If IsDataDescriptor(_targetDesc_) is *true*, _targetDesc_.[[Configurable]] is *false*, and _targetDesc_.[[Writable]] is *true*, then
    //       i. If _propertyDesc_ has a [[Writable]] field and _propertyDesc_.[[Writable]] is *false*, throw a *TypeError* exception.
    // 16. Return *true*.
    //
    // NOTE:
    // [[DefineOwnProperty]] for Proxy objects enforces the following invariants:
    // -
    // The result of [[DefineOwnProperty]] is a Boolean value.
    // -
    // A property cannot be added, if the target object is not extensible.
    // -
    // A property cannot be non-configurable, unless there exists a corresponding non-configurable own property of the target object.
    // -
    // A non-configurable property cannot be non-writable, unless there exists a corresponding non-configurable, non-writable own property of the target object.
    // -
    // If a property has a corresponding target object property then applying the Property Descriptor of the property to the target object using [[DefineOwnProperty]] will not throw an exception.
    let _ = (o, p);
    let _ = (key, desc);
    Err(agent.not_implemented("Proxy [[DefineOwnProperty]]"))
}
