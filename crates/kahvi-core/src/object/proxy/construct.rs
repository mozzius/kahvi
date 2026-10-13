//! Proxy [[Construct]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-construct-argumentslist-newtarget

#![allow(unused_imports)]

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::value::{Object, Value};

/// The [[Construct]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn construct(
    agent: &mut Agent,
    o: Object,
    p: &ProxyData,
    args: &[Value],
    new_target: Object,
) -> JsResult<Object> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Assert: IsConstructor(_target_) is *true*.
    // 4. Let _handler_ be _obj_.[[ProxyHandler]].
    // 5. Assert: _handler_ is an Object.
    // 6. Let _trap_ be ? GetMethod(_handler_, *"construct"*).
    // 7. If _trap_ is *undefined*, then
    //    a. Return ? Construct(_target_, _argList_, _newTarget_).
    // 8. Let _argArray_ be CreateArrayFromList(_argList_).
    // 9. Let _newObj_ be ? Call(_trap_, _handler_, « _target_, _argArray_, _newTarget_ »).
    // 10. If _newObj_ is not an Object, throw a *TypeError* exception.
    // 11. Return _newObj_.
    //
    // NOTE:
    // A Proxy exotic object only has a [[Construct]] internal method if the initial value of its [[ProxyTarget]] internal slot is an object that has a [[Construct]] internal method.
    // NOTE:
    // [[Construct]] for Proxy objects enforces the following invariants:
    // -
    // The result of [[Construct]] must be an Object.
    let _ = (o, p);
    let _ = (args, new_target);
    Err(agent.not_implemented("Proxy [[Construct]]"))
}
