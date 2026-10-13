//! Proxy [[SetPrototypeOf]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-setprototypeof-v

#![allow(unused_imports)]

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::value::{Object, Value};

/// The [[SetPrototypeOf]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn set_prototype_of(
    agent: &mut Agent,
    o: Object,
    p: &ProxyData,
    proto: Option<Object>,
) -> JsResult<bool> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"setPrototypeOf"*).
    // 6. If _trap_ is *undefined*, then
    //    a. Return ? _target_.[[SetPrototypeOf]](_proto_).
    // 7. Let _boolTrapResult_ be ToBoolean(? Call(_trap_, _handler_, « _target_, _proto_ »)).
    // 8. If _boolTrapResult_ is *false*, return *false*.
    // 9. Let _extensibleTarget_ be ? IsExtensible(_target_).
    // 10. If _extensibleTarget_ is *true*, return *true*.
    // 11. Let _targetProto_ be ? _target_.[[GetPrototypeOf]]().
    // 12. If SameValue(_proto_, _targetProto_) is *false*, throw a *TypeError* exception.
    // 13. Return *true*.
    //
    // NOTE:
    // [[SetPrototypeOf]] for Proxy objects enforces the following invariants:
    // -
    // The result of [[SetPrototypeOf]] is a Boolean value.
    // -
    // If the target object is not extensible, the argument value must be the same as the result of [[GetPrototypeOf]] applied to target object.
    let _ = (o, p);
    let _ = proto;
    Err(agent.not_implemented("Proxy [[SetPrototypeOf]]"))
}
