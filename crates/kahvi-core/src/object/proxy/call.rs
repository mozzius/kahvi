//! Proxy [[Call]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-call-thisargument-argumentslist

#![allow(unused_imports)]

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::value::{Object, Value};

/// The [[Call]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn call(
    agent: &mut Agent,
    o: Object,
    p: &ProxyData,
    this: Value,
    args: &[Value],
) -> JsResult<Value> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"apply"*).
    // 6. If _trap_ is *undefined*, then
    //    a. Return ? Call(_target_, _thisArg_, _argList_).
    // 7. Let _argArray_ be CreateArrayFromList(_argList_).
    // 8. Return ? Call(_trap_, _handler_, « _target_, _thisArg_, _argArray_ »).
    //
    // NOTE:
    // A Proxy exotic object only has a [[Call]] internal method if the initial value of its [[ProxyTarget]] internal slot is an object that has a [[Call]] internal method.
    let _ = (o, p);
    let _ = (this, args);
    Err(agent.not_implemented("Proxy [[Call]]"))
}
