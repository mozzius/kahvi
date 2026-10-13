//! Proxy exotic objects (ECMA-262 §10.5).
//!
//! Each internal method has its own file (`get.rs` is [[Get]], …). The
//! functions here return `None` for non-proxies, so the generic internal
//! methods fall through to ordinary behaviour, and `Some(result)` for
//! proxies.

mod call;
mod construct;
mod define_own_property;
mod delete;
mod get;
mod get_own_property;
mod get_prototype_of;
mod has_property;
mod is_extensible;
mod own_property_keys;
mod prevent_extensions;
mod set;
mod set_prototype_of;

use super::{ObjectKind, PropertyDescriptor, PropertyKey};
use crate::agent::Agent;
use crate::error::JsResult;
use crate::value::{Object, Value};

/// [[ProxyTarget]] and [[ProxyHandler]]; both None once revoked.
#[derive(Debug, Clone)]
pub struct ProxyData {
    pub target: Option<Object>,
    pub handler: Option<Object>,
    /// Whether the proxy has [[Call]] / [[Construct]] (fixed at creation).
    pub callable: bool,
    pub constructor: bool,
}

impl ProxyData {
    /// ValidateNonRevokedProxy(proxy) (§10.5.14), returning (target, handler).
    pub fn validate_non_revoked(&self, agent: &mut Agent) -> JsResult<(Object, Object)> {
        // 1. If proxy.[[ProxyTarget]] is null, throw a TypeError exception.
        // 2. Assert: proxy.[[ProxyHandler]] is not null.
        match (self.target, self.handler) {
            (Some(t), Some(h)) => Ok((t, h)),
            _ => Err(agent.throw_type_error("Cannot perform operation on a revoked proxy")),
        }
    }
}

pub fn is_proxy(agent: &Agent, o: Object) -> bool {
    agent.native::<ProxyData>(o).is_some()
}

pub fn proxy_data(agent: &Agent, o: Object) -> Option<ProxyData> {
    agent.native::<ProxyData>(o).cloned()
}

pub fn is_callable(agent: &Agent, o: Object) -> bool {
    agent.native::<ProxyData>(o).is_some_and(|p| p.callable)
}

pub fn is_constructor(agent: &Agent, o: Object) -> bool {
    agent.native::<ProxyData>(o).is_some_and(|p| p.constructor)
}

/// ProxyCreate(target, handler) (§10.5.15)
pub fn proxy_create(agent: &mut Agent, target: Value, handler: Value) -> JsResult<Object> {
    // 1. If target is not an Object, throw a TypeError exception.
    let Value::Object(target) = target else {
        return Err(agent.throw_type_error("Cannot create proxy with a non-object as target"));
    };
    // 2. If handler is not an Object, throw a TypeError exception.
    let Value::Object(handler) = handler else {
        return Err(agent.throw_type_error("Cannot create proxy with a non-object as handler"));
    };
    // 3-6. P has the Proxy internal methods, plus [[Call]] / [[Construct]] when target has them.
    let data = ProxyData {
        target: Some(target),
        handler: Some(handler),
        callable: target.is_callable(agent),
        constructor: target.is_constructor(agent),
    };
    // 7-8. Set P.[[ProxyTarget]] to target and P.[[ProxyHandler]] to handler.
    // (A proxy has no [[Prototype]] of its own: [[GetPrototypeOf]] always goes to the handler or target.)
    Ok(agent.object_with_kind(None, ObjectKind::Native(Box::new(data))))
}

/// Revokes a proxy (the revoker closure of Proxy.revocable).
pub fn revoke(agent: &mut Agent, o: Object) {
    if let Some(p) = agent.native_mut::<ProxyData>(o) {
        p.target = None;
        p.handler = None;
    }
}

fn data_of(agent: &Agent, o: Object) -> Option<ProxyData> {
    if !matches!(agent.obj(o).kind, ObjectKind::Native(_)) {
        return None;
    }
    proxy_data(agent, o)
}

/// [[GetPrototypeOf]] (§10.5.1)
pub fn get_prototype_of(agent: &mut Agent, o: Object) -> Option<JsResult<Option<Object>>> {
    let p = data_of(agent, o)?;
    Some(get_prototype_of::get_prototype_of(agent, o, &p))
}

/// [[SetPrototypeOf]] (§10.5.2)
pub fn set_prototype_of(agent: &mut Agent, o: Object, v: Option<Object>) -> Option<JsResult<bool>> {
    let p = data_of(agent, o)?;
    Some(set_prototype_of::set_prototype_of(agent, o, &p, v))
}

/// [[IsExtensible]] (§10.5.3)
pub fn is_extensible(agent: &mut Agent, o: Object) -> Option<JsResult<bool>> {
    let p = data_of(agent, o)?;
    Some(is_extensible::is_extensible(agent, o, &p))
}

/// [[PreventExtensions]] (§10.5.4)
pub fn prevent_extensions(agent: &mut Agent, o: Object) -> Option<JsResult<bool>> {
    let p = data_of(agent, o)?;
    Some(prevent_extensions::prevent_extensions(agent, o, &p))
}

/// [[GetOwnProperty]] (§10.5.5)
pub fn get_own_property(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
) -> Option<JsResult<Option<PropertyDescriptor>>> {
    let p = data_of(agent, o)?;
    Some(get_own_property::get_own_property(agent, o, &p, key))
}

/// [[DefineOwnProperty]] (§10.5.6)
pub fn define_own_property(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    desc: PropertyDescriptor,
) -> Option<JsResult<bool>> {
    let p = data_of(agent, o)?;
    Some(define_own_property::define_own_property(
        agent, o, &p, key, desc,
    ))
}

/// [[HasProperty]] (§10.5.7)
pub fn has_property(agent: &mut Agent, o: Object, key: PropertyKey) -> Option<JsResult<bool>> {
    let p = data_of(agent, o)?;
    Some(has_property::has_property(agent, o, &p, key))
}

/// [[Get]] (§10.5.8)
pub fn get(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    receiver: Value,
) -> Option<JsResult<Value>> {
    let p = data_of(agent, o)?;
    Some(get::get(agent, o, &p, key, receiver))
}

/// [[Set]] (§10.5.9)
pub fn set(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    v: Value,
    receiver: Value,
) -> Option<JsResult<bool>> {
    let p = data_of(agent, o)?;
    Some(set::set(agent, o, &p, key, v, receiver))
}

/// [[Delete]] (§10.5.10)
pub fn delete(agent: &mut Agent, o: Object, key: PropertyKey) -> Option<JsResult<bool>> {
    let p = data_of(agent, o)?;
    Some(delete::delete(agent, o, &p, key))
}

/// [[OwnPropertyKeys]] (§10.5.11)
pub fn own_property_keys(agent: &mut Agent, o: Object) -> Option<JsResult<Vec<PropertyKey>>> {
    let p = data_of(agent, o)?;
    Some(own_property_keys::own_property_keys(agent, o, &p))
}

/// [[Call]] (§10.5.12)
pub fn call(agent: &mut Agent, o: Object, this: Value, args: &[Value]) -> Option<JsResult<Value>> {
    let p = data_of(agent, o)?;
    Some(call::call(agent, o, &p, this, args))
}

/// [[Construct]] (§10.5.13)
pub fn construct(
    agent: &mut Agent,
    o: Object,
    args: &[Value],
    new_target: Object,
) -> Option<JsResult<Object>> {
    let p = data_of(agent, o)?;
    Some(construct::construct(agent, o, &p, args, new_target))
}
