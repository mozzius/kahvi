//! Proxy [[GetPrototypeOf]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-getprototypeof

#![allow(unused_imports)]

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::value::{Object, Value};

/// The [[GetPrototypeOf]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn get_prototype_of(
    agent: &mut Agent,
    o: Object,
    p: &ProxyData,
) -> JsResult<Option<Object>> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"getPrototypeOf"*).
    // 6. If _trap_ is *undefined*, then
    //    a. Return ? _target_.[[GetPrototypeOf]]().
    // 7. Let _handlerProto_ be ? Call(_trap_, _handler_, « _target_ »).
    // 8. If _handlerProto_ is not an Object and _handlerProto_ is not *null*, throw a *TypeError* exception.
    // 9. Let _extensibleTarget_ be ? IsExtensible(_target_).
    // 10. If _extensibleTarget_ is *true*, return _handlerProto_.
    // 11. Let _targetProto_ be ? _target_.[[GetPrototypeOf]]().
    // 12. If SameValue(_handlerProto_, _targetProto_) is *false*, throw a *TypeError* exception.
    // 13. Return _handlerProto_.
    //
    // NOTE:
    // [[GetPrototypeOf]] for Proxy objects enforces the following invariants:
    // -
    // The result of [[GetPrototypeOf]] must be either an Object or *null*.
    // -
    // If the target object is not extensible, [[GetPrototypeOf]] applied to the Proxy object must return the same value as [[GetPrototypeOf]] applied to the Proxy object's target object.
    let _ = (o, p);
    Err(agent.not_implemented("Proxy [[GetPrototypeOf]]"))
}
