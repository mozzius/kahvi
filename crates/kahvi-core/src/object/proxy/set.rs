//! Proxy [[Set]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-set-p-v-receiver

#![allow(unused_imports)]

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::object::{PropertyDescriptor, PropertyKey};
use crate::ops;
use crate::value::{Object, Value};

/// The [[Set]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn set(
    agent: &mut Agent,
    _o: Object,
    p: &ProxyData,
    key: PropertyKey,
    value: Value,
    receiver: Value,
) -> JsResult<bool> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    let (target, handler) = p.validate_non_revoked(agent)?;
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"set"*).
    let key_name = agent.names.set;
    let trap = ops::get_method(agent, Value::Object(handler), PropertyKey::String(key_name))?;
    // 6. If _trap_ is *undefined*, then
    if matches!(trap, Value::Undefined) {
        // a. Return ? _target_.[[Set]](_propertyKey_, _value_, _receiver_).
        return target.internal_set(agent, key, value, receiver);
    }
    // 7. Let _boolTrapResult_ be ToBoolean(? Call(_trap_, _handler_, « _target_, _propertyKey_, _value_, _receiver_ »)).
    let key_value = agent.key_to_value(key);
    let trap_result = ops::call(
        agent,
        trap,
        Value::Object(handler),
        &[Value::Object(target), key_value, value, receiver],
    )?;
    let bool_trap_result = ops::to_boolean(agent, trap_result);
    // 8. If _boolTrapResult_ is *false*, return *false*.
    if !bool_trap_result {
        return Ok(false);
    }
    // 9. Let _targetDesc_ be ? _target_.[[GetOwnProperty]](_propertyKey_).
    let target_desc = target.internal_get_own_property(agent, key)?;
    // 10. If _targetDesc_ is not *undefined* and _targetDesc_.[[Configurable]] is *false*, then
    if let Some(target_desc) = target_desc.filter(|d| d.configurable == Some(false)) {
        // a. If IsDataDescriptor(_targetDesc_) is *true* and _targetDesc_.[[Writable]] is *false*, then
        if target_desc.is_data_descriptor() && target_desc.writable == Some(false) {
            // i. If SameValue(_value_, _targetDesc_.[[Value]]) is *false*, throw a *TypeError* exception.
            let target_value = target_desc.value.unwrap_or(Value::Undefined);
            if !ops::same_value(agent, value, target_value) {
                return Err(agent.throw_type_error(
                    "Proxy set trap changed the value of a non-writable, non-configurable target property",
                ));
            }
        }
        // b. If IsAccessorDescriptor(_targetDesc_) is *true*, then
        if target_desc.is_accessor_descriptor() {
            // i. If _targetDesc_.[[Setter]] is *undefined*, throw a *TypeError* exception.
            if matches!(
                target_desc.set.unwrap_or(Value::Undefined),
                Value::Undefined
            ) {
                return Err(agent.throw_type_error(
                    "Proxy set trap set a non-configurable accessor property without a setter",
                ));
            }
        }
    }
    // 11. Return *true*.
    Ok(true)
}
