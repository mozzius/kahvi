//! Proxy [[GetOwnProperty]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-getownproperty-p

#![allow(unused_imports)]

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::object::{PropertyDescriptor, PropertyKey};
use crate::value::{Object, Value};

/// The [[GetOwnProperty]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn get_own_property(
    agent: &mut Agent,
    o: Object,
    p: &ProxyData,
    key: PropertyKey,
) -> JsResult<Option<PropertyDescriptor>> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"getOwnPropertyDescriptor"*).
    // 6. If _trap_ is *undefined*, then
    //    a. Return ? _target_.[[GetOwnProperty]](_propertyKey_).
    // 7. Let _trapResultObj_ be ? Call(_trap_, _handler_, « _target_, _propertyKey_ »).
    // 8. If _trapResultObj_ is not an Object and _trapResultObj_ is not *undefined*, throw a *TypeError* exception.
    // 9. Let _targetDesc_ be ? _target_.[[GetOwnProperty]](_propertyKey_).
    // 10. If _trapResultObj_ is *undefined*, then
    //    a. If _targetDesc_ is *undefined*, return *undefined*.
    //    b. If _targetDesc_.[[Configurable]] is *false*, throw a *TypeError* exception.
    //    c. Let _extensibleTarget_ be ? IsExtensible(_target_).
    //    d. If _extensibleTarget_ is *false*, throw a *TypeError* exception.
    //    e. Return *undefined*.
    // 11. Let _extensibleTarget_ be ? IsExtensible(_target_).
    // 12. Let _resultDesc_ be ? ToPropertyDescriptor(_trapResultObj_).
    // 13. Perform CompletePropertyDescriptor(_resultDesc_).
    // 14. Let _valid_ be IsCompatiblePropertyDescriptor(_extensibleTarget_, _resultDesc_, _targetDesc_).
    // 15. If _valid_ is *false*, throw a *TypeError* exception.
    // 16. If _resultDesc_.[[Configurable]] is *false*, then
    //    a. If _targetDesc_ is *undefined* or _targetDesc_.[[Configurable]] is *true*, then
    //       i. Throw a *TypeError* exception.
    //    b. If _resultDesc_ has a [[Writable]] field and _resultDesc_.[[Writable]] is *false*, then
    //       i. Assert: _targetDesc_ has a [[Writable]] field.
    //       ii. If _targetDesc_.[[Writable]] is *true*, throw a *TypeError* exception.
    // 17. Return _resultDesc_.
    //
    // NOTE:
    // [[GetOwnProperty]] for Proxy objects enforces the following invariants:
    // -
    // The result of [[GetOwnProperty]] must be either a Property Descriptor or *undefined*.
    // -
    // A property cannot be reported as non-existent, if it exists as a non-configurable own property of the target object.
    // -
    // A property cannot be reported as non-existent, if it exists as an own property of a non-extensible target object.
    // -
    // A property cannot be reported as existent, if it does not exist as an own property of the target object and the target object is not extensible.
    // -
    // A property cannot be reported as non-configurable, unless it exists as a non-configurable own property of the target object.
    // -
    // A property cannot be reported as both non-configurable and non-writable, unless it exists as a non-configurable, non-writable own property of the target object.
    let _ = (o, p);
    let _ = key;
    Err(agent.not_implemented("Proxy [[GetOwnProperty]]"))
}
