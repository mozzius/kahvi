//! Proxy [[Get]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-get-p-receiver

#![allow(unused_imports)]

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::object::{PropertyDescriptor, PropertyKey};
use crate::value::{Object, Value};

/// The [[Get]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn get(
    agent: &mut Agent,
    o: Object,
    p: &ProxyData,
    key: PropertyKey,
    receiver: Value,
) -> JsResult<Value> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"get"*).
    // 6. If _trap_ is *undefined*, then
    //    a. Return ? _target_.[[Get]](_propertyKey_, _receiver_).
    // 7. Let _trapResult_ be ? Call(_trap_, _handler_, « _target_, _propertyKey_, _receiver_ »).
    // 8. Let _targetDesc_ be ? _target_.[[GetOwnProperty]](_propertyKey_).
    // 9. If _targetDesc_ is not *undefined* and _targetDesc_.[[Configurable]] is *false*, then
    //    a. If IsDataDescriptor(_targetDesc_) is *true* and _targetDesc_.[[Writable]] is *false*, then
    //       i. If SameValue(_trapResult_, _targetDesc_.[[Value]]) is *false*, throw a *TypeError* exception.
    //    b. If IsAccessorDescriptor(_targetDesc_) is *true* and _targetDesc_.[[Getter]] is *undefined*, then
    //       i. If _trapResult_ is not *undefined*, throw a *TypeError* exception.
    // 10. Return _trapResult_.
    //
    // NOTE:
    // [[Get]] for Proxy objects enforces the following invariants:
    // -
    // The value reported for a property must be the same as the value of the corresponding target object property if the target object property is a non-writable, non-configurable own data property.
    // -
    // The value reported for a property must be *undefined* if the corresponding target object property is a non-configurable own accessor property that has *undefined* as its [[Getter]] attribute.
    let _ = (o, p);
    let _ = (key, receiver);
    Err(agent.not_implemented("Proxy [[Get]]"))
}
