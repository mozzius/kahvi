//! Proxy [[Call]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-call-thisargument-argumentslist

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::ops;
use crate::value::{Object, Value};

/// The [[Call]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn call(
    agent: &mut Agent,
    _o: Object,
    p: &ProxyData,
    this: Value,
    args: &[Value],
) -> JsResult<Value> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    let (target, handler) = p.validate_non_revoked(agent)?;
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"apply"*).
    let apply_units: Vec<u16> = "apply".encode_utf16().collect();
    let apply_name = agent.intern(&apply_units);
    let apply_key = agent.key_from_string(apply_name);
    let trap = ops::get_method(agent, Value::Object(handler), apply_key)?;
    // 6. If _trap_ is *undefined*, then
    if matches!(trap, Value::Undefined) {
        // a. Return ? Call(_target_, _thisArg_, _argList_).
        return ops::call(agent, Value::Object(target), this, args);
    }
    // 7. Let _argArray_ be CreateArrayFromList(_argList_).
    let arg_array = ops::create_array_from_list(agent, args);
    // 8. Return ? Call(_trap_, _handler_, « _target_, _thisArg_, _argArray_ »).
    //
    // NOTE:
    // A Proxy exotic object only has a [[Call]] internal method if the initial value of its [[ProxyTarget]] internal slot is an object that has a [[Call]] internal method.
    ops::call(
        agent,
        trap,
        Value::Object(handler),
        &[Value::Object(target), this, Value::Object(arg_array)],
    )
}
