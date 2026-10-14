//! Proxy [[OwnPropertyKeys]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-ownpropertykeys

#![allow(unused_imports)]

use std::collections::HashSet;

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::object::{PropertyDescriptor, PropertyKey};
use crate::ops;
use crate::value::{Object, Value};

/// The [[OwnPropertyKeys]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn own_property_keys(
    agent: &mut Agent,
    _o: Object,
    p: &ProxyData,
) -> JsResult<Vec<PropertyKey>> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    let (target, handler) = p.validate_non_revoked(agent)?;
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"ownKeys"*).
    let trap_key = agent.key("ownKeys");
    let trap = ops::get_method(agent, Value::Object(handler), trap_key)?;
    // 6. If _trap_ is *undefined*, then
    if matches!(trap, Value::Undefined) {
        // a. Return ? _target_.[[OwnPropertyKeys]]().
        return target.internal_own_property_keys(agent);
    }
    // 7. Let _trapResultArray_ be ? Call(_trap_, _handler_, « _target_ »).
    let trap_result_array = ops::call(
        agent,
        trap,
        Value::Object(handler),
        &[Value::Object(target)],
    )?;
    // 8. Let _trapResult_ be ? CreateListFromArrayLike(_trapResultArray_, ~property-key~).
    let trap_result_values = ops::create_list_from_array_like(agent, trap_result_array, true)?;
    let mut trap_result = Vec::with_capacity(trap_result_values.len());
    for value in trap_result_values {
        let key = match value {
            Value::String(s) => agent.key_from_string(s),
            Value::Symbol(s) => PropertyKey::from(s),
            _ => return Err(agent.throw_type_error("ownKeys trap result is not a property key")),
        };
        trap_result.push(key);
    }
    // 9. If _trapResult_ contains any duplicate entries, throw a *TypeError* exception.
    let mut seen = HashSet::with_capacity(trap_result.len());
    for key in &trap_result {
        if !seen.insert(*key) {
            return Err(agent.throw_type_error("ownKeys trap result contains duplicate entries"));
        }
    }
    // 10. Let _extensibleTarget_ be ? IsExtensible(_target_).
    let extensible_target = ops::is_extensible(agent, target)?;
    // 11. Let _targetKeys_ be ? _target_.[[OwnPropertyKeys]]().
    let target_keys = target.internal_own_property_keys(agent)?;
    // 12. Assert: _targetKeys_ is a List of property keys.
    // 13. Assert: _targetKeys_ contains no duplicate entries.
    // 14. Let _targetConfigurableKeys_ be a new empty List.
    let mut target_configurable_keys = Vec::new();
    // 15. Let _targetNonconfigurableKeys_ be a new empty List.
    let mut target_nonconfigurable_keys = Vec::new();
    // 16. For each element _key_ of _targetKeys_, do
    for key in target_keys {
        // a. Let _propertyDesc_ be ? _target_.[[GetOwnProperty]](_key_).
        let property_desc = target.internal_get_own_property(agent, key)?;
        match property_desc {
            // b. If _propertyDesc_ is not *undefined* and _propertyDesc_.[[Configurable]] is *false*, then
            Some(desc) if desc.configurable == Some(false) => {
                // i. Append _key_ to _targetNonconfigurableKeys_.
                target_nonconfigurable_keys.push(key);
            }
            // c. Else,
            _ => {
                // i. Append _key_ to _targetConfigurableKeys_.
                target_configurable_keys.push(key);
            }
        }
    }
    // 17. If _extensibleTarget_ is *true* and _targetNonconfigurableKeys_ is empty, then
    if extensible_target && target_nonconfigurable_keys.is_empty() {
        // a. Return _trapResult_.
        return Ok(trap_result);
    }
    // 18. Let _uncheckedResultKeys_ be a List whose elements are the elements of _trapResult_.
    let mut unchecked_result_keys: HashSet<PropertyKey> = trap_result.iter().copied().collect();
    // 19. For each element _key_ of _targetNonconfigurableKeys_, do
    for key in target_nonconfigurable_keys {
        // a. If _uncheckedResultKeys_ does not contain _key_, throw a *TypeError* exception.
        // b. Remove _key_ from _uncheckedResultKeys_.
        if !unchecked_result_keys.remove(&key) {
            return Err(agent.throw_type_error(
                "ownKeys trap result omits a non-configurable property of the target",
            ));
        }
    }
    // 20. If _extensibleTarget_ is *true*, return _trapResult_.
    if extensible_target {
        return Ok(trap_result);
    }
    // 21. For each element _key_ of _targetConfigurableKeys_, do
    for key in target_configurable_keys {
        // a. If _uncheckedResultKeys_ does not contain _key_, throw a *TypeError* exception.
        // b. Remove _key_ from _uncheckedResultKeys_.
        if !unchecked_result_keys.remove(&key) {
            return Err(agent.throw_type_error(
                "ownKeys trap result omits a property of the non-extensible target",
            ));
        }
    }
    // 22. If _uncheckedResultKeys_ is not empty, throw a *TypeError* exception.
    if !unchecked_result_keys.is_empty() {
        return Err(agent.throw_type_error(
            "ownKeys trap result has keys not present on the non-extensible target",
        ));
    }
    // 23. Return _trapResult_.
    Ok(trap_result)
}
