//! Proxy [[HasProperty]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-hasproperty-p

#![allow(unused_imports)]

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::object::{PropertyDescriptor, PropertyKey};
use crate::ops;
use crate::value::{Object, Value};

/// The [[HasProperty]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn has_property(
    agent: &mut Agent,
    o: Object,
    p: &ProxyData,
    key: PropertyKey,
) -> JsResult<bool> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    let _ = o;
    let (target, handler) = p.validate_non_revoked(agent)?;
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"has"*).
    let trap_key = agent.key("has");
    let trap = ops::get_method(agent, Value::Object(handler), trap_key)?;
    // 6. If _trap_ is *undefined*, then
    if matches!(trap, Value::Undefined) {
        // a. Return ? _target_.[[HasProperty]](_propertyKey_).
        return target.internal_has_property(agent, key);
    }
    // 7. Let _boolTrapResult_ be ToBoolean(? Call(_trap_, _handler_, « _target_, _propertyKey_ »)).
    let key_value = agent.key_to_value(key);
    let trap_result = ops::call(
        agent,
        trap,
        Value::Object(handler),
        &[Value::Object(target), key_value],
    )?;
    let bool_trap_result = ops::to_boolean(agent, trap_result);
    // 8. If _boolTrapResult_ is *false*, then
    if !bool_trap_result {
        // a. Let _targetDesc_ be ? _target_.[[GetOwnProperty]](_propertyKey_).
        let target_desc = target.internal_get_own_property(agent, key)?;
        // b. If _targetDesc_ is not *undefined*, then
        if let Some(target_desc) = target_desc {
            // i. If _targetDesc_.[[Configurable]] is *false*, throw a *TypeError* exception.
            if target_desc.configurable == Some(false) {
                return Err(agent.throw_type_error(
                    "Proxy has trap returned false for non-configurable property",
                ));
            }
            // ii. Let _extensibleTarget_ be ? IsExtensible(_target_).
            let extensible_target = ops::is_extensible(agent, target)?;
            // iii. If _extensibleTarget_ is *false*, throw a *TypeError* exception.
            if !extensible_target {
                return Err(agent.throw_type_error(
                    "Proxy has trap returned false for property of non-extensible target",
                ));
            }
        }
    }
    // 9. Return _boolTrapResult_.
    Ok(bool_trap_result)
}
