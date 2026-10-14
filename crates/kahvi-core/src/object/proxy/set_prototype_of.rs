//! Proxy [[SetPrototypeOf]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-setprototypeof-v

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::ops;
use crate::value::{Object, Value};

/// The [[SetPrototypeOf]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn set_prototype_of(
    agent: &mut Agent,
    _o: Object,
    p: &ProxyData,
    proto: Option<Object>,
) -> JsResult<bool> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    let (target, handler) = p.validate_non_revoked(agent)?;
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"setPrototypeOf"*).
    let trap_key = agent.key("setPrototypeOf");
    let trap = ops::get_method(agent, Value::Object(handler), trap_key)?;
    // 6. If _trap_ is *undefined*, then
    if matches!(trap, Value::Undefined) {
        // a. Return ? _target_.[[SetPrototypeOf]](_proto_).
        return target.internal_set_prototype_of(agent, proto);
    }
    // 7. Let _boolTrapResult_ be ToBoolean(? Call(_trap_, _handler_, « _target_, _proto_ »)).
    let proto_value = match proto {
        Some(o) => Value::Object(o),
        None => Value::Null,
    };
    let trap_result = ops::call(
        agent,
        trap,
        Value::Object(handler),
        &[Value::Object(target), proto_value],
    )?;
    let bool_trap_result = ops::to_boolean(agent, trap_result);
    // 8. If _boolTrapResult_ is *false*, return *false*.
    if !bool_trap_result {
        return Ok(false);
    }
    // 9. Let _extensibleTarget_ be ? IsExtensible(_target_).
    let extensible_target = ops::is_extensible(agent, target)?;
    // 10. If _extensibleTarget_ is *true*, return *true*.
    if extensible_target {
        return Ok(true);
    }
    // 11. Let _targetProto_ be ? _target_.[[GetPrototypeOf]]().
    let target_proto = target.internal_get_prototype_of(agent)?;
    let target_proto_value = match target_proto {
        Some(o) => Value::Object(o),
        None => Value::Null,
    };
    // 12. If SameValue(_proto_, _targetProto_) is *false*, throw a *TypeError* exception.
    if !ops::same_value(agent, proto_value, target_proto_value) {
        return Err(agent.throw_type_error(
            "Proxy setPrototypeOf trap returned truish for a non-extensible target with a different prototype",
        ));
    }
    // 13. Return *true*.
    Ok(true)
}
