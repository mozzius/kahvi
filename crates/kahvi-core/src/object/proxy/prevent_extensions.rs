//! Proxy [[PreventExtensions]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-preventextensions

#![allow(unused_imports)]

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::value::{Object, Value};

/// The [[PreventExtensions]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn prevent_extensions(agent: &mut Agent, o: Object, p: &ProxyData) -> JsResult<bool> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"preventExtensions"*).
    // 6. If _trap_ is *undefined*, then
    //    a. Return ? _target_.[[PreventExtensions]]().
    // 7. Let _boolTrapResult_ be ToBoolean(? Call(_trap_, _handler_, « _target_ »)).
    // 8. If _boolTrapResult_ is *true*, then
    //    a. Let _extensibleTarget_ be ? IsExtensible(_target_).
    //    b. If _extensibleTarget_ is *true*, throw a *TypeError* exception.
    // 9. Return _boolTrapResult_.
    //
    // NOTE:
    // [[PreventExtensions]] for Proxy objects enforces the following invariants:
    // -
    // The result of [[PreventExtensions]] is a Boolean value.
    // -
    // [[PreventExtensions]] applied to the Proxy object only returns *true* if [[IsExtensible]] applied to the Proxy object's target object is *false*.
    let _ = (o, p);
    Err(agent.not_implemented("Proxy [[PreventExtensions]]"))
}
