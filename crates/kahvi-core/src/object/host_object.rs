//! Host objects: exotic objects whose properties the host implements (what
//! React Native's JSI calls a HostObject). Not part of ECMAScript; their
//! internal methods follow JSI's model:
//!
//! - [[Get]] always asks the host (a host returns undefined for names it
//!   does not know; the prototype chain is not consulted).
//! - [[Set]] and [[DefineOwnProperty]] with a value hand it to the host.
//! - [[OwnPropertyKeys]] are the host's property names; [[GetOwnProperty]]
//!   of one of them is a writable, enumerable, configurable data property
//!   whose value the host gives.
//! - [[HasProperty]] is true for the host's names, else follows the
//!   prototype chain. [[Delete]] does nothing and succeeds.

use std::sync::Arc;

use crate::agent::Agent;
use crate::error::JsResult;
use crate::gc::{Trace, Tracer};
use crate::object::{ObjectKind, PropertyDescriptor, PropertyKey};
use crate::value::{Object, Value};

/// What the host implements. The object is passed as `this`.
pub trait HostObject: std::any::Any + Send + Sync {
    fn get(&self, agent: &mut Agent, this: Object, key: PropertyKey) -> JsResult<Value>;
    fn set(&self, agent: &mut Agent, this: Object, key: PropertyKey, value: Value) -> JsResult<()>;
    fn property_names(&self, agent: &mut Agent, this: Object) -> JsResult<Vec<PropertyKey>>;
}

/// The internal slot of a host object.
#[derive(Clone)]
pub struct HostObjectData(pub Arc<dyn HostObject>);

impl std::fmt::Debug for HostObjectData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("HostObjectData")
    }
}

/// The host holds its own references; the slot holds no engine handles.
impl Trace for HostObjectData {
    fn trace(&self, _: &mut Tracer) {}
}

/// A new host object with %Object.prototype% as its prototype.
pub fn create_host_object(agent: &mut Agent, host: Arc<dyn HostObject>) -> Object {
    let proto = agent.intrinsic("%Object.prototype%");
    let o = agent.ordinary_object_create(Some(proto));
    agent.obj_mut(o).kind = ObjectKind::Native(Box::new(HostObjectData(host)));
    o
}

/// The host behind a host object.
pub fn host_of(agent: &Agent, o: Object) -> Option<Arc<dyn HostObject>> {
    if !matches!(agent.obj(o).kind, ObjectKind::Native(_)) {
        return None;
    }
    agent.native::<HostObjectData>(o).map(|d| d.0.clone())
}

pub(crate) fn get_own_property(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
) -> Option<JsResult<Option<PropertyDescriptor>>> {
    let host = host_of(agent, o)?;
    Some((|| {
        if !host.property_names(agent, o)?.contains(&key) {
            return Ok(None);
        }
        let value = host.get(agent, o, key)?;
        Ok(Some(PropertyDescriptor::data(value, true, true, true)))
    })())
}

pub(crate) fn define_own_property(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    desc: PropertyDescriptor,
) -> Option<JsResult<bool>> {
    let host = host_of(agent, o)?;
    Some(match desc.value {
        Some(v) => host.set(agent, o, key, v).map(|_| true),
        None => Ok(false),
    })
}

pub(crate) fn has_property(agent: &mut Agent, o: Object, key: PropertyKey) -> Option<JsResult<bool>> {
    let host = host_of(agent, o)?;
    Some((|| {
        if host.property_names(agent, o)?.contains(&key) {
            return Ok(true);
        }
        match o.internal_get_prototype_of(agent)? {
            Some(p) => p.internal_has_property(agent, key),
            None => Ok(false),
        }
    })())
}

pub(crate) fn get(agent: &mut Agent, o: Object, key: PropertyKey) -> Option<JsResult<Value>> {
    let host = host_of(agent, o)?;
    Some(host.get(agent, o, key))
}

pub(crate) fn set(agent: &mut Agent, o: Object, key: PropertyKey, value: Value) -> Option<JsResult<bool>> {
    let host = host_of(agent, o)?;
    Some(host.set(agent, o, key, value).map(|_| true))
}

pub(crate) fn delete(agent: &Agent, o: Object) -> Option<JsResult<bool>> {
    host_of(agent, o)?;
    Some(Ok(true))
}

pub(crate) fn own_property_keys(agent: &mut Agent, o: Object) -> Option<JsResult<Vec<PropertyKey>>> {
    let host = host_of(agent, o)?;
    Some(host.property_names(agent, o))
}
