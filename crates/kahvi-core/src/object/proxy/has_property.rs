//! Proxy [[HasProperty]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-hasproperty-p

#![allow(unused_imports)]

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::object::{PropertyDescriptor, PropertyKey};
use crate::value::{Object, Value};

/// The [[HasProperty]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn has_property(
    agent: &mut Agent,
    o: Object,
    p: &ProxyData,
    key: PropertyKey,
) -> JsResult<bool> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"has"*).
    // 6. If _trap_ is *undefined*, then
    //    a. Return ? _target_.[[HasProperty]](_propertyKey_).
    // 7. Let _boolTrapResult_ be ToBoolean(? Call(_trap_, _handler_, « _target_, _propertyKey_ »)).
    // 8. If _boolTrapResult_ is *false*, then
    //    a. Let _targetDesc_ be ? _target_.[[GetOwnProperty]](_propertyKey_).
    //    b. If _targetDesc_ is not *undefined*, then
    //       i. If _targetDesc_.[[Configurable]] is *false*, throw a *TypeError* exception.
    //       ii. Let _extensibleTarget_ be ? IsExtensible(_target_).
    //       iii. If _extensibleTarget_ is *false*, throw a *TypeError* exception.
    // 9. Return _boolTrapResult_.
    //
    // NOTE:
    // [[HasProperty]] for Proxy objects enforces the following invariants:
    // -
    // The result of [[HasProperty]] is a Boolean value.
    // -
    // A property cannot be reported as non-existent, if it exists as a non-configurable own property of the target object.
    // -
    // A property cannot be reported as non-existent, if it exists as an own property of the target object and the target object is not extensible.
    let _ = (o, p);
    let _ = key;
    Err(agent.not_implemented("Proxy [[HasProperty]]"))
}
