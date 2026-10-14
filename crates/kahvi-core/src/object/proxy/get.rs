//! Proxy [[Get]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-get-p-receiver

#![allow(unused_imports)]

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::object::PropertyKey;
use crate::ops;
use crate::value::{Object, Value};

/// The [[Get]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn get(
    agent: &mut Agent,
    _o: Object,
    p: &ProxyData,
    key: PropertyKey,
    receiver: Value,
) -> JsResult<Value> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    let (target, handler) = p.validate_non_revoked(agent)?;
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"get"*).
    let key_name = agent.names.get;
    let trap = ops::get_method(agent, Value::Object(handler), PropertyKey::String(key_name))?;
    // 6. If _trap_ is *undefined*, then
    if matches!(trap, Value::Undefined) {
        // a. Return ? _target_.[[Get]](_propertyKey_, _receiver_).
        return target.internal_get(agent, key, receiver);
    }
    // 7. Let _trapResult_ be ? Call(_trap_, _handler_, « _target_, _propertyKey_, _receiver_ »).
    let key_value = agent.key_to_value(key);
    let trap_result = ops::call(
        agent,
        trap,
        Value::Object(handler),
        &[Value::Object(target), key_value, receiver],
    )?;
    // 8. Let _targetDesc_ be ? _target_.[[GetOwnProperty]](_propertyKey_).
    let target_desc = target.internal_get_own_property(agent, key)?;
    // 9. If _targetDesc_ is not *undefined* and _targetDesc_.[[Configurable]] is *false*, then
    if let Some(target_desc) = target_desc.filter(|d| d.configurable == Some(false)) {
        // a. If IsDataDescriptor(_targetDesc_) is *true* and _targetDesc_.[[Writable]] is *false*, then
        if target_desc.is_data_descriptor() && target_desc.writable == Some(false) {
            // i. If SameValue(_trapResult_, _targetDesc_.[[Value]]) is *false*, throw a *TypeError* exception.
            let target_value = target_desc.value.unwrap_or(Value::Undefined);
            if !ops::same_value(agent, trap_result, target_value) {
                return Err(agent.throw_type_error(
                    "Proxy get trap result differs from non-writable, non-configurable target property",
                ));
            }
        }
        // b. If IsAccessorDescriptor(_targetDesc_) is *true* and _targetDesc_.[[Getter]] is *undefined*, then
        if target_desc.is_accessor_descriptor()
            && matches!(
                target_desc.get.unwrap_or(Value::Undefined),
                Value::Undefined
            )
        {
            // i. If _trapResult_ is not *undefined*, throw a *TypeError* exception.
            if !matches!(trap_result, Value::Undefined) {
                return Err(agent.throw_type_error(
                    "Proxy get trap must return undefined for non-configurable accessor property without getter",
                ));
            }
        }
    }
    // 10. Return _trapResult_.
    Ok(trap_result)
}
