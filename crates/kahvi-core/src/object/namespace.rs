//! Module namespace exotic objects (§10.4.6). Each internal method returns
//! `None` for other objects, like the Proxy dispatchers.

use super::{ObjectKind, PropertyDescriptor, PropertyKey};
use crate::agent::Agent;
use crate::error::JsResult;
use crate::value::{JsString, Object, Value};

/// [[Module]] and [[Exports]].
#[derive(Debug)]
pub struct NamespaceData {
    pub module: u32,
    /// The exported names, ordered as if by sorting them by code unit.
    pub exports: Vec<JsString>,
}

fn data(agent: &Agent, o: Object) -> Option<&NamespaceData> {
    match &agent.obj(o).kind {
        ObjectKind::ModuleNamespace(d) => Some(d),
        _ => None,
    }
}

/// The export name a (non-Symbol) key stands for, if it is one of [[Exports]].
fn export_named(agent: &mut Agent, o: Object, key: PropertyKey) -> Option<JsString> {
    let name = match key {
        PropertyKey::Symbol(_) => return None,
        PropertyKey::String(s) => s,
        PropertyKey::Index(i) => {
            let s = i.to_string();
            agent.string(&s)
        }
    };
    data(agent, o)
        .expect("a module namespace")
        .exports
        .contains(&name)
        .then_some(name)
}

pub fn is_namespace(agent: &Agent, o: Object) -> bool {
    data(agent, o).is_some()
}

/// [[GetPrototypeOf]] (§10.4.6.1): null.
pub fn get_prototype_of(agent: &Agent, o: Object) -> Option<JsResult<Option<Object>>> {
    data(agent, o).map(|_| Ok(None))
}

/// [[SetPrototypeOf]](V) (§10.4.6.2): SetImmutablePrototype(O, V).
pub fn set_prototype_of(agent: &Agent, o: Object, v: Option<Object>) -> Option<JsResult<bool>> {
    data(agent, o).map(|_| Ok(v.is_none()))
}

/// [[IsExtensible]] (§10.4.6.3): false.
pub fn is_extensible(agent: &Agent, o: Object) -> Option<JsResult<bool>> {
    data(agent, o).map(|_| Ok(false))
}

/// [[PreventExtensions]] (§10.4.6.4): true.
pub fn prevent_extensions(agent: &Agent, o: Object) -> Option<JsResult<bool>> {
    data(agent, o).map(|_| Ok(true))
}

/// [[GetOwnProperty]](P) (§10.4.6.5)
pub fn get_own_property(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
) -> Option<JsResult<Option<PropertyDescriptor>>> {
    data(agent, o)?;
    // 1. If P is a Symbol, return OrdinaryGetOwnProperty(O, P).
    if let PropertyKey::Symbol(_) = key {
        return Some(Ok(super::internal::ordinary_get_own_property(
            agent, o, key,
        )));
    }
    // 2-3. If exports does not contain P, return undefined.
    if export_named(agent, o, key).is_none() {
        return Some(Ok(None));
    }
    // 4. Let value be ? O.[[Get]](P, O).
    let value = match o.internal_get(agent, key, Value::Object(o)) {
        Ok(v) => v,
        Err(e) => return Some(Err(e)),
    };
    // 5. Return PropertyDescriptor { [[Value]]: value, [[Writable]]: true, [[Enumerable]]: true, [[Configurable]]: false }.
    Some(Ok(Some(PropertyDescriptor {
        value: Some(value),
        writable: Some(true),
        get: None,
        set: None,
        enumerable: Some(true),
        configurable: Some(false),
    })))
}

/// [[DefineOwnProperty]](P, Desc) (§10.4.6.6)
pub fn define_own_property(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    desc: PropertyDescriptor,
) -> Option<JsResult<bool>> {
    data(agent, o)?;
    // 1. If P is a Symbol, return ! OrdinaryDefineOwnProperty(O, P, Desc).
    if let PropertyKey::Symbol(_) = key {
        return Some(super::internal::ordinary_define_own_property(
            agent, o, key, desc,
        ));
    }
    Some((|| {
        // 2. Let current be ? O.[[GetOwnProperty]](P).
        // 3. If current is undefined, return false.
        let Some(current) = o.internal_get_own_property(agent, key)? else {
            return Ok(false);
        };
        // 4. If Desc has a [[Configurable]] field and Desc.[[Configurable]] is true, return false.
        // 5. If Desc has an [[Enumerable]] field and Desc.[[Enumerable]] is false, return false.
        // 6. If IsAccessorDescriptor(Desc) is true, return false.
        // 7. If Desc has a [[Writable]] field and Desc.[[Writable]] is false, return false.
        if desc.configurable == Some(true)
            || desc.enumerable == Some(false)
            || desc.get.is_some()
            || desc.set.is_some()
            || desc.writable == Some(false)
        {
            return Ok(false);
        }
        // 8. If Desc has a [[Value]] field, return SameValue(Desc.[[Value]], current.[[Value]]).
        if let Some(v) = desc.value {
            let cur = current.value.unwrap_or(Value::Undefined);
            return Ok(crate::ops::same_value(agent, v, cur));
        }
        // 9. Return true.
        Ok(true)
    })())
}

/// [[HasProperty]](P) (§10.4.6.7)
pub fn has_property(agent: &mut Agent, o: Object, key: PropertyKey) -> Option<JsResult<bool>> {
    data(agent, o)?;
    // 1. If P is a Symbol, return ! OrdinaryHasProperty(O, P).
    if let PropertyKey::Symbol(_) = key {
        return Some(super::internal::ordinary_has_property(agent, o, key));
    }
    // 2-4. Return whether exports contains P.
    Some(Ok(export_named(agent, o, key).is_some()))
}

/// [[Get]](P, Receiver) (§10.4.6.8)
pub fn get(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    receiver: Value,
) -> Option<JsResult<Value>> {
    let module = data(agent, o)?.module;
    // 1. If P is a Symbol, return ! OrdinaryGet(O, P, Receiver).
    if let PropertyKey::Symbol(_) = key {
        return Some(super::internal::ordinary_get(agent, o, key, receiver));
    }
    // 2-3. If exports does not contain P, return undefined.
    let Some(name) = export_named(agent, o, key) else {
        return Some(Ok(Value::Undefined));
    };
    // 4-12. The binding the export resolves to.
    Some(crate::vm::module_namespace_get(agent, module, name))
}

/// [[Set]] (§10.4.6.9): false.
pub fn set(agent: &Agent, o: Object) -> Option<JsResult<bool>> {
    data(agent, o).map(|_| Ok(false))
}

/// [[Delete]](P) (§10.4.6.10)
pub fn delete(agent: &mut Agent, o: Object, key: PropertyKey) -> Option<JsResult<bool>> {
    data(agent, o)?;
    // 1. If P is a Symbol, return ! OrdinaryDelete(O, P).
    if let PropertyKey::Symbol(_) = key {
        return Some(super::internal::ordinary_delete(agent, o, key));
    }
    // 2-4. If exports contains P, return false; else true.
    Some(Ok(export_named(agent, o, key).is_none()))
}

/// [[OwnPropertyKeys]] (§10.4.6.11)
pub fn own_property_keys(agent: &mut Agent, o: Object) -> Option<JsResult<Vec<PropertyKey>>> {
    let exports = data(agent, o)?.exports.clone();
    // 1. Let exports be O.[[Exports]].
    // 2. Let symbolKeys be OrdinaryOwnPropertyKeys(O).
    // 3. Return the list-concatenation of exports and symbolKeys.
    let mut keys: Vec<PropertyKey> = exports
        .into_iter()
        .map(|s| agent.key_from_string(s))
        .collect();
    keys.extend(agent.obj(o).properties.keys_in_spec_order());
    Some(Ok(keys))
}
