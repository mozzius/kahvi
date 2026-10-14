//! Object internal methods (ECMA-262 §10.1 ordinary objects, plus dispatch to
//! the exotic objects in §10.4). Named `internal_*` so they can't be confused
//! with the abstract operations of the same name in [`crate::ops`].

use super::{ObjectKind, Property, PropertyDescriptor, PropertyKey};
use crate::agent::Agent;
use crate::error::JsResult;
use crate::ops;
use crate::value::{Object, Value};

impl Object {
    // ---- [[GetPrototypeOf]] / [[SetPrototypeOf]] ---------------------------

    /// O.[[GetPrototypeOf]]()
    pub fn internal_get_prototype_of(self, agent: &mut Agent) -> JsResult<Option<Object>> {
        if let Some(r) = crate::object::proxy::get_prototype_of(agent, self) {
            return r;
        }
        if let Some(r) = super::namespace::get_prototype_of(agent, self) {
            return r;
        }
        Ok(agent.obj(self).prototype)
    }

    /// O.[[SetPrototypeOf]](V)
    pub fn internal_set_prototype_of(self, agent: &mut Agent, v: Option<Object>) -> JsResult<bool> {
        if let Some(r) = crate::object::proxy::set_prototype_of(agent, self, v) {
            return r;
        }
        if let Some(r) = super::namespace::set_prototype_of(agent, self, v) {
            return r;
        }
        if matches!(agent.obj(self).kind, ObjectKind::ImmutablePrototype) {
            // SetImmutablePrototype(O, V)
            return Ok(agent.obj(self).prototype == v);
        }
        Ok(ordinary_set_prototype_of(agent, self, v))
    }

    /// O.[[IsExtensible]]()
    pub fn internal_is_extensible(self, agent: &mut Agent) -> JsResult<bool> {
        if let Some(r) = crate::object::proxy::is_extensible(agent, self) {
            return r;
        }
        if let Some(r) = super::namespace::is_extensible(agent, self) {
            return r;
        }
        Ok(agent.obj(self).extensible)
    }

    /// O.[[PreventExtensions]]()
    pub fn internal_prevent_extensions(self, agent: &mut Agent) -> JsResult<bool> {
        if let Some(r) = crate::object::proxy::prevent_extensions(agent, self) {
            return r;
        }
        if let Some(r) = super::namespace::prevent_extensions(agent, self) {
            return r;
        }
        agent.obj_mut(self).extensible = false;
        Ok(true)
    }

    // ---- own properties -------------------------------------------------------

    /// O.[[GetOwnProperty]](P)
    pub fn internal_get_own_property(
        self,
        agent: &mut Agent,
        key: PropertyKey,
    ) -> JsResult<Option<PropertyDescriptor>> {
        if let Some(r) = crate::object::proxy::get_own_property(agent, self, key) {
            return r;
        }
        if let Some(r) = super::host_object::get_own_property(agent, self, key) {
            return r;
        }
        if let Some(r) = super::namespace::get_own_property(agent, self, key) {
            return r;
        }
        if let Some(r) = super::typed_array::get_own_property(agent, self, key) {
            return r;
        }
        match &agent.obj(self).kind {
            ObjectKind::StringObject(s) => {
                let s = *s;
                if let Some(d) = ordinary_get_own_property(agent, self, key) {
                    return Ok(Some(d));
                }
                Ok(string_get_own_property(agent, s, key))
            }
            ObjectKind::Arguments(Some(_)) => Ok(crate::function::arguments_get_own_property(
                agent, self, key,
            )),
            _ => Ok(ordinary_get_own_property(agent, self, key)),
        }
    }

    /// O.[[DefineOwnProperty]](P, Desc)
    pub fn internal_define_own_property(
        self,
        agent: &mut Agent,
        key: PropertyKey,
        desc: PropertyDescriptor,
    ) -> JsResult<bool> {
        if let Some(r) = crate::object::proxy::define_own_property(agent, self, key, desc) {
            return r;
        }
        if let Some(r) = super::host_object::define_own_property(agent, self, key, desc) {
            return r;
        }
        if let Some(r) = super::namespace::define_own_property(agent, self, key, desc) {
            return r;
        }
        if let Some(r) = super::typed_array::define_own_property(agent, self, key, desc) {
            return r;
        }
        match &agent.obj(self).kind {
            ObjectKind::Array => super::array::array_define_own_property(agent, self, key, desc),
            ObjectKind::StringObject(s) => {
                let s = *s;
                // 1. Let stringDesc be StringGetOwnProperty(S, P).
                if let Some(string_desc) = string_get_own_property(agent, s, key) {
                    // 2. If stringDesc is not undefined, then
                    //    a. Let extensible be S.[[Extensible]].
                    let extensible = agent.obj(self).extensible;
                    //    b. Return IsCompatiblePropertyDescriptor(extensible, Desc, stringDesc).
                    return Ok(validate_and_apply_property_descriptor(
                        agent,
                        None,
                        key,
                        extensible,
                        desc,
                        Some(string_desc),
                    ));
                }
                // 3. Return ! OrdinaryDefineOwnProperty(S, P, Desc).
                ordinary_define_own_property(agent, self, key, desc)
            }
            ObjectKind::Arguments(Some(_)) => {
                crate::function::arguments_define_own_property(agent, self, key, desc)
            }
            _ => ordinary_define_own_property(agent, self, key, desc),
        }
    }

    /// O.[[HasProperty]](P)
    pub fn internal_has_property(self, agent: &mut Agent, key: PropertyKey) -> JsResult<bool> {
        if let Some(r) = crate::object::proxy::has_property(agent, self, key) {
            return r;
        }
        if let Some(r) = super::host_object::has_property(agent, self, key) {
            return r;
        }
        if let Some(r) = super::namespace::has_property(agent, self, key) {
            return r;
        }
        if let Some(r) = super::typed_array::has_property(agent, self, key) {
            return r;
        }
        ordinary_has_property(agent, self, key)
    }

    /// O.[[Get]](P, Receiver)
    pub fn internal_get(
        self,
        agent: &mut Agent,
        key: PropertyKey,
        receiver: Value,
    ) -> JsResult<Value> {
        if let Some(r) = crate::object::proxy::get(agent, self, key, receiver) {
            return r;
        }
        if let Some(r) = super::host_object::get(agent, self, key) {
            return r;
        }
        if let Some(r) = super::namespace::get(agent, self, key, receiver) {
            return r;
        }
        if let Some(r) = super::typed_array::get(agent, self, key) {
            return r;
        }
        if matches!(agent.obj(self).kind, ObjectKind::Arguments(Some(_)))
            && let Some(v) = crate::function::arguments_get(agent, self, key)?
        {
            return Ok(v);
        }
        ordinary_get(agent, self, key, receiver)
    }

    /// O.[[Set]](P, V, Receiver)
    pub fn internal_set(
        self,
        agent: &mut Agent,
        key: PropertyKey,
        value: Value,
        receiver: Value,
    ) -> JsResult<bool> {
        if let Some(r) = crate::object::proxy::set(agent, self, key, value, receiver) {
            return r;
        }
        if let Some(r) = super::host_object::set(agent, self, key, value) {
            return r;
        }
        if let Some(r) = super::namespace::set(agent, self) {
            return r;
        }
        if let Some(r) = super::typed_array::set(agent, self, key, value, receiver) {
            return r;
        }
        if matches!(agent.obj(self).kind, ObjectKind::Arguments(Some(_))) {
            return crate::function::arguments_set(agent, self, key, value, receiver);
        }
        ordinary_set(agent, self, key, value, receiver)
    }

    /// O.[[Delete]](P)
    pub fn internal_delete(self, agent: &mut Agent, key: PropertyKey) -> JsResult<bool> {
        if let Some(r) = crate::object::proxy::delete(agent, self, key) {
            return r;
        }
        if let Some(r) = super::host_object::delete(agent, self) {
            return r;
        }
        if let Some(r) = super::namespace::delete(agent, self, key) {
            return r;
        }
        if let Some(r) = super::typed_array::delete(agent, self, key) {
            return r;
        }
        if matches!(agent.obj(self).kind, ObjectKind::Arguments(Some(_))) {
            return crate::function::arguments_delete(agent, self, key);
        }
        ordinary_delete(agent, self, key)
    }

    /// O.[[OwnPropertyKeys]]()
    pub fn internal_own_property_keys(self, agent: &mut Agent) -> JsResult<Vec<PropertyKey>> {
        if let Some(r) = crate::object::proxy::own_property_keys(agent, self) {
            return r;
        }
        if let Some(r) = super::host_object::own_property_keys(agent, self) {
            return r;
        }
        if let Some(r) = super::namespace::own_property_keys(agent, self) {
            return r;
        }
        if let Some(r) = super::typed_array::own_property_keys(agent, self) {
            return r;
        }
        if let ObjectKind::StringObject(s) = &agent.obj(self).kind {
            let len = agent.str_len(*s) as u32;
            // 5. For each integer i such that 0 ≤ i < len, in ascending order, add ! ToString(𝔽(i)).
            let mut keys: Vec<PropertyKey> = (0..len).map(PropertyKey::Index).collect();
            // 6-8. Then the object's own array indices ≥ len, strings and symbols in order.
            keys.extend(
                agent
                    .obj(self)
                    .properties
                    .keys_in_spec_order()
                    .into_iter()
                    .filter(|k| !matches!(k, PropertyKey::Index(i) if *i < len)),
            );
            return Ok(keys);
        }
        Ok(agent.obj(self).properties.keys_in_spec_order())
    }

    // ---- [[Call]] / [[Construct]] presence -------------------------------------

    /// IsCallable: whether O has a [[Call]] internal method.
    pub fn is_callable(self, agent: &Agent) -> bool {
        match &agent.obj(self).kind {
            ObjectKind::Function(_) | ObjectKind::Builtin(_) | ObjectKind::BoundFunction(_) => true,
            ObjectKind::Native(_) => crate::object::proxy::is_callable(agent, self),
            _ => false,
        }
    }

    /// IsConstructor: whether O has a [[Construct]] internal method.
    pub fn is_constructor(self, agent: &Agent) -> bool {
        match &agent.obj(self).kind {
            ObjectKind::Function(f) => f.is_constructor(),
            ObjectKind::Builtin(b) => b.constructor,
            ObjectKind::BoundFunction(b) => b.target.is_constructor(agent),
            ObjectKind::Native(_) => crate::object::proxy::is_constructor(agent, self),
            _ => false,
        }
    }
}

/// OrdinarySetPrototypeOf(O, V) (§10.1.2.1)
pub fn ordinary_set_prototype_of(agent: &mut Agent, o: Object, v: Option<Object>) -> bool {
    // 1. Let current be O.[[Prototype]].
    let current = agent.obj(o).prototype;
    // 2. If SameValue(V, current) is true, return true.
    if v == current {
        return true;
    }
    // 3. Let extensible be O.[[Extensible]].
    // 4. If extensible is false, return false.
    if !agent.obj(o).extensible {
        return false;
    }
    // 5-8. Let p be V; loop while p is not null, checking for cycles.
    let mut p = v;
    while let Some(po) = p {
        // 8.b.i If SameValue(p, O) is true, return false.
        if po == o {
            return false;
        }
        // 8.c.i If p.[[GetPrototypeOf]] is not the ordinary definition, set done to true.
        if crate::object::proxy::is_proxy(agent, po) {
            break;
        }
        // 8.c.ii Else, set p to p.[[Prototype]].
        p = agent.obj(po).prototype;
    }
    // 9. Set O.[[Prototype]] to V.
    agent.obj_mut(o).prototype = v;
    // 10. Return true.
    true
}

/// OrdinaryGetOwnProperty(O, P) (§10.1.5.1)
pub fn ordinary_get_own_property(
    agent: &Agent,
    o: Object,
    key: PropertyKey,
) -> Option<PropertyDescriptor> {
    agent.obj(o).properties.get(&key).map(|p| p.to_descriptor())
}

/// StringGetOwnProperty(S, P) (§10.4.3.5)
fn string_get_own_property(
    agent: &mut Agent,
    s: crate::value::JsString,
    key: PropertyKey,
) -> Option<PropertyDescriptor> {
    // 1-7. Only canonical numeric index strings that are valid integer indices
    // less than the length name characters; those are always `Index` keys.
    let PropertyKey::Index(i) = key else {
        return None;
    };
    let units = agent.str(s);
    let unit = *units.get(i as usize)?;
    // 8. Let resultStr be the substring of str from index to index + 1.
    let result = agent.intern(&[unit]);
    // 9. Return the PropertyDescriptor { [[Value]]: resultStr, [[Writable]]: false,
    //    [[Enumerable]]: true, [[Configurable]]: false }.
    Some(PropertyDescriptor::data(
        Value::String(result),
        false,
        true,
        false,
    ))
}

/// OrdinaryDefineOwnProperty(O, P, Desc) (§10.1.6.1)
pub fn ordinary_define_own_property(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    desc: PropertyDescriptor,
) -> JsResult<bool> {
    // 1. Let current be ? O.[[GetOwnProperty]](P).
    let current = o.internal_get_own_property(agent, key)?;
    // 2. Let extensible be ? IsExtensible(O).
    let extensible = o.internal_is_extensible(agent)?;
    // 3. Return ValidateAndApplyPropertyDescriptor(O, P, extensible, Desc, current).
    Ok(validate_and_apply_property_descriptor(
        agent,
        Some(o),
        key,
        extensible,
        desc,
        current,
    ))
}

/// ValidateAndApplyPropertyDescriptor(O, P, extensible, Desc, current) (§10.1.6.3)
pub fn validate_and_apply_property_descriptor(
    agent: &mut Agent,
    o: Option<Object>,
    key: PropertyKey,
    extensible: bool,
    desc: PropertyDescriptor,
    current: Option<PropertyDescriptor>,
) -> bool {
    // 1. Assert: IsPropertyKey(P) is true.
    // 2. If current is undefined, then
    let Some(current) = current else {
        // a. If extensible is false, return false.
        if !extensible {
            return false;
        }
        // b. If O is undefined, return true.
        let Some(o) = o else { return true };
        // c. If IsAccessorDescriptor(Desc) is true, then create an accessor property
        //    with absent fields defaulting to undefined / false.
        let prop = if desc.is_accessor_descriptor() {
            Property::Accessor {
                get: desc.get.unwrap_or(Value::Undefined),
                set: desc.set.unwrap_or(Value::Undefined),
                enumerable: desc.enumerable.unwrap_or(false),
                configurable: desc.configurable.unwrap_or(false),
            }
        } else {
            // d. Else, create a data property with absent fields defaulting to undefined / false.
            Property::Data {
                value: desc.value.unwrap_or(Value::Undefined),
                writable: desc.writable.unwrap_or(false),
                enumerable: desc.enumerable.unwrap_or(false),
                configurable: desc.configurable.unwrap_or(false),
            }
        };
        agent.obj_mut(o).properties.insert(key, prop);
        // e. Return true.
        return true;
    };
    // 3. Assert: current is a fully populated Property Descriptor.
    // 4. If Desc does not have any fields, return true.
    if desc.value.is_none()
        && desc.writable.is_none()
        && desc.get.is_none()
        && desc.set.is_none()
        && desc.enumerable.is_none()
        && desc.configurable.is_none()
    {
        return true;
    }
    // 5. If current.[[Configurable]] is false, then
    if current.configurable == Some(false) {
        // a. If Desc has a [[Configurable]] field and Desc.[[Configurable]] is true, return false.
        if desc.configurable == Some(true) {
            return false;
        }
        // b. If Desc has an [[Enumerable]] field and SameValue(Desc.[[Enumerable]], current.[[Enumerable]]) is false, return false.
        if desc.enumerable.is_some() && desc.enumerable != current.enumerable {
            return false;
        }
        // c. If IsGenericDescriptor(Desc) is false and SameValue(IsAccessorDescriptor(Desc),
        //    IsAccessorDescriptor(current)) is false, return false.
        if !desc.is_generic_descriptor()
            && desc.is_accessor_descriptor() != current.is_accessor_descriptor()
        {
            return false;
        }
        // d. If IsAccessorDescriptor(current) is true, then
        if current.is_accessor_descriptor() {
            // i. If Desc has a [[Get]] field and SameValue(Desc.[[Get]], current.[[Get]]) is false, return false.
            if let Some(g) = desc.get
                && !ops::same_value(agent, g, current.get.unwrap_or_default())
            {
                return false;
            }
            // ii. If Desc has a [[Set]] field and SameValue(Desc.[[Set]], current.[[Set]]) is false, return false.
            if let Some(s) = desc.set
                && !ops::same_value(agent, s, current.set.unwrap_or_default())
            {
                return false;
            }
        } else if current.writable == Some(false) {
            // e. Else if current.[[Writable]] is false, then
            // i. If Desc has a [[Writable]] field and Desc.[[Writable]] is true, return false.
            if desc.writable == Some(true) {
                return false;
            }
            // ii. If Desc has a [[Value]] field and SameValue(Desc.[[Value]], current.[[Value]]) is false, return false.
            if let Some(v) = desc.value
                && !ops::same_value(agent, v, current.value.unwrap_or_default())
            {
                return false;
            }
        }
    }
    // 6. If O is not undefined, then
    if let Some(o) = o {
        let configurable = desc.configurable.or(current.configurable).unwrap_or(false);
        let enumerable = desc.enumerable.or(current.enumerable).unwrap_or(false);
        let prop = if current.is_data_descriptor() && desc.is_accessor_descriptor() {
            // a. If IsDataDescriptor(current) is true and IsAccessorDescriptor(Desc) is true, then
            //    replace with an accessor property, keeping configurable/enumerable unless given.
            Property::Accessor {
                get: desc.get.unwrap_or(Value::Undefined),
                set: desc.set.unwrap_or(Value::Undefined),
                enumerable,
                configurable,
            }
        } else if current.is_accessor_descriptor() && desc.is_data_descriptor() {
            // b. Else if IsAccessorDescriptor(current) is true and IsDataDescriptor(Desc) is true, then
            //    replace with a data property.
            Property::Data {
                value: desc.value.unwrap_or(Value::Undefined),
                writable: desc.writable.unwrap_or(false),
                enumerable,
                configurable,
            }
        } else if current.is_accessor_descriptor() {
            // c. Else, for each field of Desc, set the corresponding attribute.
            Property::Accessor {
                get: desc.get.or(current.get).unwrap_or(Value::Undefined),
                set: desc.set.or(current.set).unwrap_or(Value::Undefined),
                enumerable,
                configurable,
            }
        } else {
            Property::Data {
                value: desc.value.or(current.value).unwrap_or(Value::Undefined),
                writable: desc.writable.or(current.writable).unwrap_or(false),
                enumerable,
                configurable,
            }
        };
        agent.obj_mut(o).properties.insert(key, prop);
    }
    // 7. Return true.
    true
}

/// OrdinaryHasProperty(O, P) (§10.1.7.1)
pub fn ordinary_has_property(agent: &mut Agent, o: Object, key: PropertyKey) -> JsResult<bool> {
    // 1. Let hasOwn be ? O.[[GetOwnProperty]](P).
    // 2. If hasOwn is not undefined, return true.
    if o.internal_get_own_property(agent, key)?.is_some() {
        return Ok(true);
    }
    // 3. Let parent be ? O.[[GetPrototypeOf]]().
    // 4. If parent is not null, return ? parent.[[HasProperty]](P).
    match o.internal_get_prototype_of(agent)? {
        Some(parent) => parent.internal_has_property(agent, key),
        // 5. Return false.
        None => Ok(false),
    }
}

/// OrdinaryGet(O, P, Receiver) (§10.1.8.1)
pub fn ordinary_get(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    receiver: Value,
) -> JsResult<Value> {
    // 1. Let desc be ? O.[[GetOwnProperty]](P).
    let Some(desc) = o.internal_get_own_property(agent, key)? else {
        // 2. If desc is undefined, then
        //    a. Let parent be ? O.[[GetPrototypeOf]]().
        //    b. If parent is null, return undefined.
        //    c. Return ? parent.[[Get]](P, Receiver).
        return match o.internal_get_prototype_of(agent)? {
            Some(parent) => parent.internal_get(agent, key, receiver),
            None => Ok(Value::Undefined),
        };
    };
    // 3. If IsDataDescriptor(desc) is true, return desc.[[Value]].
    if desc.is_data_descriptor() {
        return Ok(desc.value.unwrap_or_default());
    }
    // 4. Assert: IsAccessorDescriptor(desc) is true.
    // 5. Let getter be desc.[[Get]].
    // 6. If getter is undefined, return undefined.
    match desc.get {
        Some(getter @ Value::Object(_)) => ops::call(agent, getter, receiver, &[]),
        _ => Ok(Value::Undefined),
    }
}

/// OrdinarySet(O, P, V, Receiver) (§10.1.9.1)
pub fn ordinary_set(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    value: Value,
    receiver: Value,
) -> JsResult<bool> {
    // 1. Let ownDesc be ? O.[[GetOwnProperty]](P).
    let own_desc = o.internal_get_own_property(agent, key)?;
    // 2. Return ? OrdinarySetWithOwnDescriptor(O, P, V, Receiver, ownDesc).
    ordinary_set_with_own_descriptor(agent, o, key, value, receiver, own_desc)
}

/// OrdinarySetWithOwnDescriptor(O, P, V, Receiver, ownDesc) (§10.1.9.2)
pub fn ordinary_set_with_own_descriptor(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    value: Value,
    receiver: Value,
    own_desc: Option<PropertyDescriptor>,
) -> JsResult<bool> {
    // 1. If ownDesc is undefined, then
    let own_desc = match own_desc {
        Some(d) => d,
        None => {
            // a. Let parent be ? O.[[GetPrototypeOf]]().
            // b. If parent is not null, return ? parent.[[Set]](P, V, Receiver).
            if let Some(parent) = o.internal_get_prototype_of(agent)? {
                return parent.internal_set(agent, key, value, receiver);
            }
            // c. Else, set ownDesc to the PropertyDescriptor { [[Value]]: undefined,
            //    [[Writable]]: true, [[Enumerable]]: true, [[Configurable]]: true }.
            PropertyDescriptor::data(Value::Undefined, true, true, true)
        }
    };
    // 2. If IsDataDescriptor(ownDesc) is true, then
    if own_desc.is_data_descriptor() {
        // a. If ownDesc.[[Writable]] is false, return false.
        if own_desc.writable == Some(false) {
            return Ok(false);
        }
        // b. If Receiver is not an Object, return false.
        let Value::Object(receiver) = receiver else {
            return Ok(false);
        };
        // c. Let existingDescriptor be ? Receiver.[[GetOwnProperty]](P).
        // d. If existingDescriptor is not undefined, then
        if let Some(existing) = receiver.internal_get_own_property(agent, key)? {
            // i. If IsAccessorDescriptor(existingDescriptor) is true, return false.
            if existing.is_accessor_descriptor() {
                return Ok(false);
            }
            // ii. If existingDescriptor.[[Writable]] is false, return false.
            if existing.writable == Some(false) {
                return Ok(false);
            }
            // iii. Let valueDesc be the PropertyDescriptor { [[Value]]: V }.
            let value_desc = PropertyDescriptor {
                value: Some(value),
                ..Default::default()
            };
            // iv. Return ? Receiver.[[DefineOwnProperty]](P, valueDesc).
            return receiver.internal_define_own_property(agent, key, value_desc);
        }
        // e. Else: Return ? CreateDataProperty(Receiver, P, V).
        return ops::create_data_property(agent, receiver, key, value);
    }
    // 3. Assert: IsAccessorDescriptor(ownDesc) is true.
    // 4. Let setter be ownDesc.[[Set]].
    // 5. If setter is undefined, return false.
    let Some(setter @ Value::Object(_)) = own_desc.set else {
        return Ok(false);
    };
    // 6. Perform ? Call(setter, Receiver, « V »).
    ops::call(agent, setter, receiver, &[value])?;
    // 7. Return true.
    Ok(true)
}

/// OrdinaryDelete(O, P) (§10.1.10.1)
pub fn ordinary_delete(agent: &mut Agent, o: Object, key: PropertyKey) -> JsResult<bool> {
    // 1. Let desc be ? O.[[GetOwnProperty]](P).
    // 2. If desc is undefined, return true.
    let Some(desc) = o.internal_get_own_property(agent, key)? else {
        return Ok(true);
    };
    // 3. If desc.[[Configurable]] is true, then
    if desc.configurable == Some(true) {
        // a. Remove the own property with name P from O.
        agent.obj_mut(o).properties.remove(&key);
        // b. Return true.
        return Ok(true);
    }
    // 4. Return false.
    Ok(false)
}
