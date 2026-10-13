//! Proxy [[IsExtensible]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-isextensible

#![allow(unused_imports)]

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::value::{Object, Value};

/// The [[IsExtensible]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn is_extensible(agent: &mut Agent, o: Object, p: &ProxyData) -> JsResult<bool> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"isExtensible"*).
    // 6. If _trap_ is *undefined*, then
    //    a. Return ? IsExtensible(_target_).
    // 7. Let _boolTrapResult_ be ToBoolean(? Call(_trap_, _handler_, « _target_ »)).
    // 8. Let _targetResult_ be ? IsExtensible(_target_).
    // 9. If _boolTrapResult_ is not _targetResult_, throw a *TypeError* exception.
    // 10. Return _boolTrapResult_.
    //
    // NOTE:
    // [[IsExtensible]] for Proxy objects enforces the following invariants:
    // -
    // The result of [[IsExtensible]] is a Boolean value.
    // -
    // [[IsExtensible]] applied to the Proxy object must return the same value as [[IsExtensible]] applied to the Proxy object's target object with the same argument.
    let _ = (o, p);
    Err(agent.not_implemented("Proxy [[IsExtensible]]"))
}
