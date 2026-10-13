//! Proxy [[Set]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-set-p-v-receiver

#![allow(unused_imports)]

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::object::{PropertyDescriptor, PropertyKey};
use crate::value::{Object, Value};

/// The [[Set]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn set(
    agent: &mut Agent,
    o: Object,
    p: &ProxyData,
    key: PropertyKey,
    value: Value,
    receiver: Value,
) -> JsResult<bool> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"set"*).
    // 6. If _trap_ is *undefined*, then
    //    a. Return ? _target_.[[Set]](_propertyKey_, _value_, _receiver_).
    // 7. Let _boolTrapResult_ be ToBoolean(? Call(_trap_, _handler_, « _target_, _propertyKey_, _value_, _receiver_ »)).
    // 8. If _boolTrapResult_ is *false*, return *false*.
    // 9. Let _targetDesc_ be ? _target_.[[GetOwnProperty]](_propertyKey_).
    // 10. If _targetDesc_ is not *undefined* and _targetDesc_.[[Configurable]] is *false*, then
    //    a. If IsDataDescriptor(_targetDesc_) is *true* and _targetDesc_.[[Writable]] is *false*, then
    //       i. If SameValue(_value_, _targetDesc_.[[Value]]) is *false*, throw a *TypeError* exception.
    //    b. If IsAccessorDescriptor(_targetDesc_) is *true*, then
    //       i. If _targetDesc_.[[Setter]] is *undefined*, throw a *TypeError* exception.
    // 11. Return *true*.
    //
    // NOTE:
    // [[Set]] for Proxy objects enforces the following invariants:
    // -
    // The result of [[Set]] is a Boolean value.
    // -
    // Cannot change the value of a property to be different from the value of the corresponding target object property if the corresponding target object property is a non-writable, non-configurable own data property.
    // -
    // Cannot set the value of a property if the corresponding target object property is a non-configurable own accessor property that has *undefined* as its [[Setter]] attribute.
    let _ = (o, p);
    let _ = (key, value, receiver);
    Err(agent.not_implemented("Proxy [[Set]]"))
}
