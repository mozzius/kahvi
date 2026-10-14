//! Proxy [[GetPrototypeOf]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-getprototypeof

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::ops;
use crate::value::{Object, Value};

/// The [[GetPrototypeOf]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn get_prototype_of(
    agent: &mut Agent,
    _o: Object,
    p: &ProxyData,
) -> JsResult<Option<Object>> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    let (target, handler) = p.validate_non_revoked(agent)?;
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"getPrototypeOf"*).
    let trap_key = agent.key("getPrototypeOf");
    let trap = ops::get_method(agent, Value::Object(handler), trap_key)?;
    // 6. If _trap_ is *undefined*, then
    if matches!(trap, Value::Undefined) {
        // a. Return ? _target_.[[GetPrototypeOf]]().
        return target.internal_get_prototype_of(agent);
    }
    // 7. Let _handlerProto_ be ? Call(_trap_, _handler_, « _target_ »).
    let handler_proto = ops::call(
        agent,
        trap,
        Value::Object(handler),
        &[Value::Object(target)],
    )?;
    // 8. If _handlerProto_ is not an Object and _handlerProto_ is not *null*, throw a *TypeError* exception.
    let handler_proto = match handler_proto {
        Value::Object(o) => Some(o),
        Value::Null => None,
        _ => {
            return Err(
                agent.throw_type_error("Proxy getPrototypeOf trap must return an object or null")
            );
        }
    };
    // 9. Let _extensibleTarget_ be ? IsExtensible(_target_).
    let extensible_target = ops::is_extensible(agent, target)?;
    // 10. If _extensibleTarget_ is *true*, return _handlerProto_.
    if extensible_target {
        return Ok(handler_proto);
    }
    // 11. Let _targetProto_ be ? _target_.[[GetPrototypeOf]]().
    let target_proto = target.internal_get_prototype_of(agent)?;
    // 12. If SameValue(_handlerProto_, _targetProto_) is *false*, throw a *TypeError* exception.
    if handler_proto != target_proto {
        return Err(agent.throw_type_error(
            "Proxy getPrototypeOf trap result differs from non-extensible target's prototype",
        ));
    }
    // 13. Return _handlerProto_.
    Ok(handler_proto)
}
