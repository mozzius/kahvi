//! Proxy [[GetOwnProperty]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-getownproperty-p

#![allow(unused_imports)]

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::object::internal::validate_and_apply_property_descriptor;
use crate::object::{PropertyDescriptor, PropertyKey};
use crate::ops;
use crate::value::{Object, Value};

/// The [[GetOwnProperty]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn get_own_property(
    agent: &mut Agent,
    o: Object,
    p: &ProxyData,
    key: PropertyKey,
) -> JsResult<Option<PropertyDescriptor>> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"getOwnPropertyDescriptor"*).
    // 6. If _trap_ is *undefined*, then
    //    a. Return ? _target_.[[GetOwnProperty]](_propertyKey_).
    // 7. Let _trapResultObj_ be ? Call(_trap_, _handler_, « _target_, _propertyKey_ »).
    // 8. If _trapResultObj_ is not an Object and _trapResultObj_ is not *undefined*, throw a *TypeError* exception.
    // 9. Let _targetDesc_ be ? _target_.[[GetOwnProperty]](_propertyKey_).
    // 10. If _trapResultObj_ is *undefined*, then
    //    a. If _targetDesc_ is *undefined*, return *undefined*.
    //    b. If _targetDesc_.[[Configurable]] is *false*, throw a *TypeError* exception.
    //    c. Let _extensibleTarget_ be ? IsExtensible(_target_).
    //    d. If _extensibleTarget_ is *false*, throw a *TypeError* exception.
    //    e. Return *undefined*.
    // 11. Let _extensibleTarget_ be ? IsExtensible(_target_).
    // 12. Let _resultDesc_ be ? ToPropertyDescriptor(_trapResultObj_).
    // 13. Perform CompletePropertyDescriptor(_resultDesc_).
    // 14. Let _valid_ be IsCompatiblePropertyDescriptor(_extensibleTarget_, _resultDesc_, _targetDesc_).
    // 15. If _valid_ is *false*, throw a *TypeError* exception.
    // 16. If _resultDesc_.[[Configurable]] is *false*, then
    //    a. If _targetDesc_ is *undefined* or _targetDesc_.[[Configurable]] is *true*, then
    //       i. Throw a *TypeError* exception.
    //    b. If _resultDesc_ has a [[Writable]] field and _resultDesc_.[[Writable]] is *false*, then
    //       i. Assert: _targetDesc_ has a [[Writable]] field.
    //       ii. If _targetDesc_.[[Writable]] is *true*, throw a *TypeError* exception.
    // 17. Return _resultDesc_.
    //
    // NOTE:
    // [[GetOwnProperty]] for Proxy objects enforces the following invariants:
    // -
    // The result of [[GetOwnProperty]] must be either a Property Descriptor or *undefined*.
    // -
    // A property cannot be reported as non-existent, if it exists as a non-configurable own property of the target object.
    // -
    // A property cannot be reported as non-existent, if it exists as an own property of a non-extensible target object.
    // -
    // A property cannot be reported as existent, if it does not exist as an own property of the target object and the target object is not extensible.
    // -
    // A property cannot be reported as non-configurable, unless it exists as a non-configurable own property of the target object.
    // -
    // A property cannot be reported as both non-configurable and non-writable, unless it exists as a non-configurable, non-writable own property of the target object.
    let _ = o;
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    let (target, handler) = p.validate_non_revoked(agent)?;
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"getOwnPropertyDescriptor"*).
    let trap_key = agent.key("getOwnPropertyDescriptor");
    let trap = ops::get_method(agent, Value::Object(handler), trap_key)?;
    // 6. If _trap_ is *undefined*, then
    if matches!(trap, Value::Undefined) {
        // a. Return ? _target_.[[GetOwnProperty]](_propertyKey_).
        return target.internal_get_own_property(agent, key);
    }
    // 7. Let _trapResultObj_ be ? Call(_trap_, _handler_, « _target_, _propertyKey_ »).
    let key_value = agent.key_to_value(key);
    let trap_result_obj = ops::call(
        agent,
        trap,
        Value::Object(handler),
        &[Value::Object(target), key_value],
    )?;
    // 8. If _trapResultObj_ is not an Object and _trapResultObj_ is not *undefined*, throw a *TypeError* exception.
    if !matches!(trap_result_obj, Value::Object(_) | Value::Undefined) {
        return Err(agent.throw_type_error(
            "Proxy getOwnPropertyDescriptor trap returned neither object nor undefined",
        ));
    }
    // 9. Let _targetDesc_ be ? _target_.[[GetOwnProperty]](_propertyKey_).
    let target_desc = target.internal_get_own_property(agent, key)?;
    // 10. If _trapResultObj_ is *undefined*, then
    if matches!(trap_result_obj, Value::Undefined) {
        // a. If _targetDesc_ is *undefined*, return *undefined*.
        let Some(target_desc) = target_desc else {
            return Ok(None);
        };
        // b. If _targetDesc_.[[Configurable]] is *false*, throw a *TypeError* exception.
        if target_desc.configurable == Some(false) {
            return Err(agent.throw_type_error(
                "Proxy getOwnPropertyDescriptor trap returned undefined for non-configurable property",
            ));
        }
        // c. Let _extensibleTarget_ be ? IsExtensible(_target_).
        let extensible_target = ops::is_extensible(agent, target)?;
        // d. If _extensibleTarget_ is *false*, throw a *TypeError* exception.
        if !extensible_target {
            return Err(agent.throw_type_error(
                "Proxy getOwnPropertyDescriptor trap returned undefined for non-extensible target",
            ));
        }
        // e. Return *undefined*.
        return Ok(None);
    }
    // 11. Let _extensibleTarget_ be ? IsExtensible(_target_).
    let extensible_target = ops::is_extensible(agent, target)?;
    // 12. Let _resultDesc_ be ? ToPropertyDescriptor(_trapResultObj_).
    let mut result_desc = ops::to_property_descriptor(agent, trap_result_obj)?;
    // 13. Perform CompletePropertyDescriptor(_resultDesc_).
    ops::complete_property_descriptor(&mut result_desc);
    // 14. Let _valid_ be IsCompatiblePropertyDescriptor(_extensibleTarget_, _resultDesc_, _targetDesc_).
    //     IsCompatiblePropertyDescriptor(E, Desc, current) is ValidateAndApplyPropertyDescriptor(undefined, "", E, Desc, current).
    let valid = validate_and_apply_property_descriptor(
        agent,
        None,
        key,
        extensible_target,
        result_desc,
        target_desc,
    );
    // 15. If _valid_ is *false*, throw a *TypeError* exception.
    if !valid {
        return Err(agent.throw_type_error(
            "Proxy getOwnPropertyDescriptor trap returned an incompatible descriptor",
        ));
    }
    // 16. If _resultDesc_.[[Configurable]] is *false*, then
    if result_desc.configurable == Some(false) {
        // a. If _targetDesc_ is *undefined* or _targetDesc_.[[Configurable]] is *true*, then
        if target_desc.is_none_or(|d| d.configurable == Some(true)) {
            // i. Throw a *TypeError* exception.
            return Err(agent.throw_type_error(
                "Proxy getOwnPropertyDescriptor trap reported non-configurability for a configurable property",
            ));
        }
        // b. If _resultDesc_ has a [[Writable]] field and _resultDesc_.[[Writable]] is *false*, then
        if result_desc.writable == Some(false) {
            // i. Assert: _targetDesc_ has a [[Writable]] field.
            // ii. If _targetDesc_.[[Writable]] is *true*, throw a *TypeError* exception.
            if target_desc.is_some_and(|d| d.writable == Some(true)) {
                return Err(agent.throw_type_error(
                    "Proxy getOwnPropertyDescriptor trap reported non-writability for a writable property",
                ));
            }
        }
    }
    // 17. Return _resultDesc_.
    Ok(Some(result_desc))
}
