//! Proxy exotic objects (ECMA-262 §10.5).
//!
//! Each internal method returns `None` for non-proxies (so callers fall
//! through to ordinary behaviour) and `Some(result)` for proxies. The traps
//! are not implemented yet: each is a separate ticket.

use super::{ObjectKind, PropertyDescriptor, PropertyKey};
use crate::agent::Agent;
use crate::error::JsResult;
use crate::value::{Object, Value};

/// [[ProxyTarget]] and [[ProxyHandler]]; both null once revoked.
#[derive(Debug, Clone)]
pub struct ProxyData {
    pub target: Option<Object>,
    pub handler: Option<Object>,
    /// Whether the proxy has [[Call]] / [[Construct]] (fixed at creation).
    pub callable: bool,
    pub constructor: bool,
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

fn unimplemented<T>(agent: &mut Agent, o: Object, what: &str) -> Option<JsResult<T>> {
    if !matches!(agent.obj(o).kind, ObjectKind::Native(_)) || !is_proxy(agent, o) {
        return None;
    }
    Some(Err(agent.not_implemented(&format!("Proxy {what}"))))
}

/// [[GetPrototypeOf]] (§10.5.1)
pub fn get_prototype_of(agent: &mut Agent, o: Object) -> Option<JsResult<Option<Object>>> {
    unimplemented(agent, o, "[[GetPrototypeOf]]")
}

/// [[SetPrototypeOf]] (§10.5.2)
pub fn set_prototype_of(
    agent: &mut Agent,
    o: Object,
    _v: Option<Object>,
) -> Option<JsResult<bool>> {
    unimplemented(agent, o, "[[SetPrototypeOf]]")
}

/// [[IsExtensible]] (§10.5.3)
pub fn is_extensible(agent: &mut Agent, o: Object) -> Option<JsResult<bool>> {
    unimplemented(agent, o, "[[IsExtensible]]")
}

/// [[PreventExtensions]] (§10.5.4)
pub fn prevent_extensions(agent: &mut Agent, o: Object) -> Option<JsResult<bool>> {
    unimplemented(agent, o, "[[PreventExtensions]]")
}

/// [[GetOwnProperty]] (§10.5.5)
pub fn get_own_property(
    agent: &mut Agent,
    o: Object,
    _key: PropertyKey,
) -> Option<JsResult<Option<PropertyDescriptor>>> {
    unimplemented(agent, o, "[[GetOwnProperty]]")
}

/// [[DefineOwnProperty]] (§10.5.6)
pub fn define_own_property(
    agent: &mut Agent,
    o: Object,
    _key: PropertyKey,
    _desc: PropertyDescriptor,
) -> Option<JsResult<bool>> {
    unimplemented(agent, o, "[[DefineOwnProperty]]")
}

/// [[HasProperty]] (§10.5.7)
pub fn has_property(agent: &mut Agent, o: Object, _key: PropertyKey) -> Option<JsResult<bool>> {
    unimplemented(agent, o, "[[HasProperty]]")
}

/// [[Get]] (§10.5.8)
pub fn get(
    agent: &mut Agent,
    o: Object,
    _key: PropertyKey,
    _receiver: Value,
) -> Option<JsResult<Value>> {
    unimplemented(agent, o, "[[Get]]")
}

/// [[Set]] (§10.5.9)
pub fn set(
    agent: &mut Agent,
    o: Object,
    _key: PropertyKey,
    _v: Value,
    _receiver: Value,
) -> Option<JsResult<bool>> {
    unimplemented(agent, o, "[[Set]]")
}

/// [[Delete]] (§10.5.10)
pub fn delete(agent: &mut Agent, o: Object, _key: PropertyKey) -> Option<JsResult<bool>> {
    unimplemented(agent, o, "[[Delete]]")
}

/// [[OwnPropertyKeys]] (§10.5.11)
pub fn own_property_keys(agent: &mut Agent, o: Object) -> Option<JsResult<Vec<PropertyKey>>> {
    unimplemented(agent, o, "[[OwnPropertyKeys]]")
}

/// [[Call]] (§10.5.12)
pub fn call(
    agent: &mut Agent,
    o: Object,
    _this: Value,
    _args: &[Value],
) -> Option<JsResult<Value>> {
    unimplemented(agent, o, "[[Call]]")
}

/// [[Construct]] (§10.5.13)
pub fn construct(
    agent: &mut Agent,
    o: Object,
    _args: &[Value],
    _new_target: Object,
) -> Option<JsResult<Object>> {
    unimplemented(agent, o, "[[Construct]]")
}
