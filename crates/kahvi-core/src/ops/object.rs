//! Operations on objects (ECMA-262 §7.3).

use crate::agent::Agent;
use crate::error::JsResult;
use crate::object::array::array_create;
use crate::object::{ObjectKind, PropertyDescriptor, PropertyKey};
use crate::ops::{is_callable, is_constructor, to_boolean, to_length, to_object, to_property_key};
use crate::value::{Object, Value};

/// Get(O, P) (§7.3.2)
pub fn get(agent: &mut Agent, o: Object, key: PropertyKey) -> JsResult<Value> {
    // 1. Return ? O.[[Get]](P, O).
    o.internal_get(agent, key, Value::Object(o))
}

/// GetV(V, P) (§7.3.3)
pub fn get_v(agent: &mut Agent, v: Value, key: PropertyKey) -> JsResult<Value> {
    // Fast path: primitives look properties up on their prototype without allocating.
    let o = match v {
        Value::Object(o) => o,
        Value::String(s) => {
            if key == PropertyKey::String(agent.names.length) {
                return Ok(Value::Number(agent.str_len(s) as f64));
            }
            if let PropertyKey::Index(i) = key
                && let Some(&u) = agent.str(s).get(i as usize)
            {
                return Ok(Value::String(agent.intern(&[u])));
            }
            agent.intrinsic("%String.prototype%")
        }
        Value::Number(_) => agent.intrinsic("%Number.prototype%"),
        Value::Boolean(_) => agent.intrinsic("%Boolean.prototype%"),
        Value::Symbol(_) => agent.intrinsic("%Symbol.prototype%"),
        Value::BigInt(_) => agent.intrinsic("%BigInt.prototype%"),
        // 1. Let O be ? ToObject(V).
        Value::Undefined | Value::Null => to_object(agent, v)?,
    };
    // 2. Return ? O.[[Get]](P, V).
    o.internal_get(agent, key, v)
}

/// Set(O, P, V, Throw) (§7.3.4)
pub fn set(agent: &mut Agent, o: Object, key: PropertyKey, v: Value, throw: bool) -> JsResult<()> {
    // 1. Let success be ? O.[[Set]](P, V, O).
    let success = o.internal_set(agent, key, v, Value::Object(o))?;
    // 2. If success is false and Throw is true, throw a TypeError exception.
    if !success && throw {
        let msg = format!(
            "Cannot assign to read only property '{}'",
            agent.key_display(key)
        );
        return Err(agent.throw_type_error(&msg));
    }
    // 3. Return unused.
    Ok(())
}

/// CreateDataProperty(O, P, V) (§7.3.5)
pub fn create_data_property(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    v: Value,
) -> JsResult<bool> {
    // 1. Let newDesc be the PropertyDescriptor { [[Value]]: V, [[Writable]]: true,
    //    [[Enumerable]]: true, [[Configurable]]: true }.
    let desc = PropertyDescriptor::data(v, true, true, true);
    // 2. Return ? O.[[DefineOwnProperty]](P, newDesc).
    o.internal_define_own_property(agent, key, desc)
}

/// CreateDataPropertyOrThrow(O, P, V) (§7.3.7)
pub fn create_data_property_or_throw(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    v: Value,
) -> JsResult<()> {
    // 1. Let success be ? CreateDataProperty(O, P, V).
    // 2. If success is false, throw a TypeError exception.
    if !create_data_property(agent, o, key, v)? {
        let msg = format!("Cannot define property '{}'", agent.key_display(key));
        return Err(agent.throw_type_error(&msg));
    }
    Ok(())
}

/// CreateNonEnumerableDataPropertyOrThrow(O, P, V) (§7.3.8)
pub fn create_non_enumerable_data_property_or_throw(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    v: Value,
) -> JsResult<()> {
    // 2. Let newDesc be the PropertyDescriptor { [[Value]]: V, [[Writable]]: true,
    //    [[Enumerable]]: false, [[Configurable]]: true }.
    let desc = PropertyDescriptor::data(v, true, false, true);
    // 3. Perform ! DefinePropertyOrThrow(O, P, newDesc).
    define_property_or_throw(agent, o, key, desc)
}

/// DefinePropertyOrThrow(O, P, desc) (§7.3.9)
pub fn define_property_or_throw(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    desc: PropertyDescriptor,
) -> JsResult<()> {
    // 1. Let success be ? O.[[DefineOwnProperty]](P, desc).
    // 2. If success is false, throw a TypeError exception.
    if !o.internal_define_own_property(agent, key, desc)? {
        let msg = format!("Cannot redefine property '{}'", agent.key_display(key));
        return Err(agent.throw_type_error(&msg));
    }
    Ok(())
}

/// DeletePropertyOrThrow(O, P) (§7.3.10)
pub fn delete_property_or_throw(agent: &mut Agent, o: Object, key: PropertyKey) -> JsResult<()> {
    // 1. Let success be ? O.[[Delete]](P).
    // 2. If success is false, throw a TypeError exception.
    if !o.internal_delete(agent, key)? {
        let msg = format!("Cannot delete property '{}'", agent.key_display(key));
        return Err(agent.throw_type_error(&msg));
    }
    Ok(())
}

/// GetMethod(V, P) (§7.3.11)
pub fn get_method(agent: &mut Agent, v: Value, key: PropertyKey) -> JsResult<Value> {
    // 1. Let func be ? GetV(V, P).
    let func = get_v(agent, v, key)?;
    // 2. If func is either undefined or null, return undefined.
    if func.is_nullish() {
        return Ok(Value::Undefined);
    }
    // 3. If IsCallable(func) is false, throw a TypeError exception.
    if !is_callable(agent, func) {
        let msg = format!("'{}' is not a function", agent.key_display(key));
        return Err(agent.throw_type_error(&msg));
    }
    // 4. Return func.
    Ok(func)
}

/// HasProperty(O, P) (§7.3.12)
pub fn has_property(agent: &mut Agent, o: Object, key: PropertyKey) -> JsResult<bool> {
    o.internal_has_property(agent, key)
}

/// HasOwnProperty(O, P) (§7.3.13)
pub fn has_own_property(agent: &mut Agent, o: Object, key: PropertyKey) -> JsResult<bool> {
    // 1. Let desc be ? O.[[GetOwnProperty]](P).
    // 2. If desc is undefined, return false. 3. Return true.
    Ok(o.internal_get_own_property(agent, key)?.is_some())
}

/// Call(F, V [, argumentsList]) (§7.3.14)
pub fn call(agent: &mut Agent, f: Value, this: Value, args: &[Value]) -> JsResult<Value> {
    // 2. If IsCallable(F) is false, throw a TypeError exception.
    let Value::Object(fo) = f else {
        return Err(agent.throw_type_error("not a function"));
    };
    if !fo.is_callable(agent) {
        return Err(agent.throw_type_error("not a function"));
    }
    // 3. Return ? F.[[Call]](V, argumentsList).
    fo.internal_call(agent, this, args)
}

/// Construct(F [, argumentsList [, newTarget]]) (§7.3.15)
pub fn construct(
    agent: &mut Agent,
    f: Object,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Object> {
    // 1. If newTarget is not present, set newTarget to F.
    let new_target = new_target.unwrap_or(f);
    // 3. Return ? F.[[Construct]](argumentsList, newTarget).
    f.internal_construct(agent, args, new_target)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityLevel {
    Sealed,
    Frozen,
}

/// SetIntegrityLevel(O, level) (§7.3.16)
pub fn set_integrity_level(agent: &mut Agent, o: Object, level: IntegrityLevel) -> JsResult<bool> {
    // 3. Let status be ? O.[[PreventExtensions]]().
    // 4. If status is false, return false.
    if !o.internal_prevent_extensions(agent)? {
        return Ok(false);
    }
    // 5. Let keys be ? O.[[OwnPropertyKeys]]().
    let keys = o.internal_own_property_keys(agent)?;
    match level {
        // 6. If level is sealed, then for each element k of keys,
        //    perform ? DefinePropertyOrThrow(O, k, PropertyDescriptor { [[Configurable]]: false }).
        IntegrityLevel::Sealed => {
            for k in keys {
                let d = PropertyDescriptor {
                    configurable: Some(false),
                    ..Default::default()
                };
                define_property_or_throw(agent, o, k, d)?;
            }
        }
        // 7. Else, for each element k of keys,
        IntegrityLevel::Frozen => {
            for k in keys {
                // i. Let currentDesc be ? O.[[GetOwnProperty]](k).
                // ii. If currentDesc is not undefined, then
                if let Some(current) = o.internal_get_own_property(agent, k)? {
                    // 1-2. Accessor: { [[Configurable]]: false }; data: also [[Writable]]: false.
                    let d = if current.is_accessor_descriptor() {
                        PropertyDescriptor {
                            configurable: Some(false),
                            ..Default::default()
                        }
                    } else {
                        PropertyDescriptor {
                            configurable: Some(false),
                            writable: Some(false),
                            ..Default::default()
                        }
                    };
                    // 3. Perform ? DefinePropertyOrThrow(O, k, desc).
                    define_property_or_throw(agent, o, k, d)?;
                }
            }
        }
    }
    // 8. Return true.
    Ok(true)
}

/// TestIntegrityLevel(O, level) (§7.3.17)
pub fn test_integrity_level(agent: &mut Agent, o: Object, level: IntegrityLevel) -> JsResult<bool> {
    // 1. Let extensible be ? IsExtensible(O).
    // 2. If extensible is true, return false.
    if o.internal_is_extensible(agent)? {
        return Ok(false);
    }
    // 4. Let keys be ? O.[[OwnPropertyKeys]]().
    for k in o.internal_own_property_keys(agent)? {
        // a. Let currentDesc be ? O.[[GetOwnProperty]](k).
        if let Some(current) = o.internal_get_own_property(agent, k)? {
            // i. If currentDesc.[[Configurable]] is true, return false.
            if current.configurable == Some(true) {
                return Ok(false);
            }
            // ii. If level is frozen and IsDataDescriptor(currentDesc) is true, then
            //     if currentDesc.[[Writable]] is true, return false.
            if level == IntegrityLevel::Frozen
                && current.is_data_descriptor()
                && current.writable == Some(true)
            {
                return Ok(false);
            }
        }
    }
    // 5. Return true.
    Ok(true)
}

/// CreateArrayFromList(elements) (§7.3.18)
pub fn create_array_from_list(agent: &mut Agent, elements: &[Value]) -> Object {
    // 1. Let array be ! ArrayCreate(0).
    let array = array_create(agent, 0.0, None).expect("length 0 is valid");
    // 2. Let n be 0. 3. For each element e of elements, do
    for (n, e) in elements.iter().enumerate() {
        // a. Perform ! CreateDataPropertyOrThrow(array, ! ToString(𝔽(n)), e).
        let key = agent.key_from_index(n as f64);
        let _ = create_data_property(agent, array, key, *e);
    }
    // 4. Return array.
    array
}

/// LengthOfArrayLike(obj) (§7.3.19)
pub fn length_of_array_like(agent: &mut Agent, o: Object) -> JsResult<f64> {
    // 1. Return ℝ(? ToLength(? Get(obj, "length"))).
    let len = get(agent, o, PropertyKey::String(agent.names.length))?;
    to_length(agent, len)
}

/// CreateListFromArrayLike(obj [, validElementTypes]) (§7.3.20)
pub fn create_list_from_array_like(
    agent: &mut Agent,
    v: Value,
    property_keys_only: bool,
) -> JsResult<Vec<Value>> {
    // 2. If obj is not an Object, throw a TypeError exception.
    let Value::Object(o) = v else {
        return Err(agent.throw_type_error("CreateListFromArrayLike called on non-object"));
    };
    // 3. Let len be ? LengthOfArrayLike(obj).
    let len = length_of_array_like(agent, o)?;
    // 4. Let list be a new empty List.
    let mut list = Vec::new();
    // 5-6. For each index < len:
    let mut index = 0.0;
    while index < len {
        // a. Let indexName be ! ToString(𝔽(index)).
        let key = agent.key_from_index(index);
        // b. Let next be ? Get(obj, indexName).
        let next = get(agent, o, key)?;
        // c. If validElementTypes is property-key and next is not a property key, throw a TypeError exception.
        if property_keys_only && !matches!(next, Value::String(_) | Value::Symbol(_)) {
            return Err(agent.throw_type_error("element is not a property key"));
        }
        // d. Append next to list.
        list.push(next);
        // e. Set index to index + 1.
        index += 1.0;
    }
    // 7. Return list.
    Ok(list)
}

/// Invoke(V, P [, argumentsList]) (§7.3.21)
pub fn invoke(agent: &mut Agent, v: Value, key: PropertyKey, args: &[Value]) -> JsResult<Value> {
    // 2. Let func be ? GetV(V, P).
    let func = get_v(agent, v, key)?;
    // 3. Return ? Call(func, V, argumentsList).
    call(agent, func, v, args)
}

/// OrdinaryHasInstance(C, O) (§7.3.22)
pub fn ordinary_has_instance(agent: &mut Agent, c: Value, o: Value) -> JsResult<bool> {
    // 1. If IsCallable(C) is false, return false.
    let Value::Object(co) = c else {
        return Ok(false);
    };
    if !co.is_callable(agent) {
        return Ok(false);
    }
    // 2. If C has a [[BoundTargetFunction]] internal slot, then
    if let ObjectKind::BoundFunction(b) = &agent.obj(co).kind {
        // a. Let BC be C.[[BoundTargetFunction]]. b. Return ? InstanceofOperator(O, BC).
        let target = b.target;
        return instance_of_operator(agent, o, Value::Object(target));
    }
    // 3. If O is not an Object, return false.
    let Value::Object(mut obj) = o else {
        return Ok(false);
    };
    // 4. Let P be ? Get(C, "prototype").
    let p = get(agent, co, PropertyKey::String(agent.names.prototype))?;
    // 5. If P is not an Object, throw a TypeError exception.
    let Value::Object(p) = p else {
        return Err(agent.throw_type_error("Function has non-object prototype in instanceof check"));
    };
    // 6. Repeat,
    loop {
        // a. Set O to ? O.[[GetPrototypeOf]]().
        // b. If O is null, return false.
        let Some(next) = obj.internal_get_prototype_of(agent)? else {
            return Ok(false);
        };
        // c. If SameValue(P, O) is true, return true.
        if next == p {
            return Ok(true);
        }
        obj = next;
    }
}

/// InstanceofOperator(V, target) (§13.10.2)
pub fn instance_of_operator(agent: &mut Agent, v: Value, target: Value) -> JsResult<bool> {
    // 1. If target is not an Object, throw a TypeError exception.
    if !target.is_object() {
        return Err(agent.throw_type_error("Right-hand side of 'instanceof' is not an object"));
    }
    // 2. Let instOfHandler be ? GetMethod(target, %Symbol.hasInstance%).
    let handler = get_method(
        agent,
        target,
        PropertyKey::Symbol(agent.symbols_wk.has_instance),
    )?;
    // 3. If instOfHandler is not undefined, then
    if !handler.is_undefined() {
        // a. Return ToBoolean(? Call(instOfHandler, target, « V »)).
        let r = call(agent, handler, target, &[v])?;
        return Ok(to_boolean(agent, r));
    }
    // 4. If IsCallable(target) is false, throw a TypeError exception.
    if !is_callable(agent, target) {
        return Err(agent.throw_type_error("Right-hand side of 'instanceof' is not callable"));
    }
    // 5. Return ? OrdinaryHasInstance(target, V).
    ordinary_has_instance(agent, target, v)
}

/// SpeciesConstructor(O, defaultConstructor) (§7.3.23)
pub fn species_constructor(agent: &mut Agent, o: Object, default: Object) -> JsResult<Object> {
    // 1. Let C be ? Get(O, "constructor").
    let c = get(agent, o, PropertyKey::String(agent.names.constructor))?;
    // 2. If C is undefined, return defaultConstructor.
    if c.is_undefined() {
        return Ok(default);
    }
    // 3. If C is not an Object, throw a TypeError exception.
    let Value::Object(c) = c else {
        return Err(agent.throw_type_error("object.constructor is not an object"));
    };
    // 4. Let S be ? Get(C, %Symbol.species%).
    let s = get(agent, c, PropertyKey::Symbol(agent.symbols_wk.species))?;
    // 5. If S is either undefined or null, return defaultConstructor.
    if s.is_nullish() {
        return Ok(default);
    }
    // 6. If IsConstructor(S) is true, return S.
    if let Value::Object(so) = s
        && is_constructor(agent, s)
    {
        return Ok(so);
    }
    // 7. Throw a TypeError exception.
    Err(agent.throw_type_error("object.constructor[Symbol.species] is not a constructor"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumerableKind {
    Key,
    Value,
    KeyValue,
}

/// EnumerableOwnProperties(O, kind) (§7.3.26)
pub fn enumerable_own_properties(
    agent: &mut Agent,
    o: Object,
    kind: EnumerableKind,
) -> JsResult<Vec<Value>> {
    // 1. Let ownKeys be ? O.[[OwnPropertyKeys]]().
    let own_keys = o.internal_own_property_keys(agent)?;
    // 2. Let results be a new empty List.
    let mut results = Vec::new();
    // 3. For each element key of ownKeys, do
    for key in own_keys {
        // a. If key is a String, then
        if key.is_symbol() {
            continue;
        }
        // i. Let desc be ? O.[[GetOwnProperty]](key).
        // ii. If desc is not undefined and desc.[[Enumerable]] is true, then
        let Some(desc) = o.internal_get_own_property(agent, key)? else {
            continue;
        };
        if desc.enumerable != Some(true) {
            continue;
        }
        let key_value = agent.key_to_value(key);
        match kind {
            // 1. If kind is key, append key to results.
            EnumerableKind::Key => results.push(key_value),
            _ => {
                // 2. Else, a. Let value be ? Get(O, key).
                let value = get(agent, o, key)?;
                if kind == EnumerableKind::Value {
                    // b. If kind is value, append value to results.
                    results.push(value);
                } else {
                    // c. Else, append CreateArrayFromList(« key, value ») to results.
                    let entry = create_array_from_list(agent, &[key_value, value]);
                    results.push(Value::Object(entry));
                }
            }
        }
    }
    // 4. Return results.
    Ok(results)
}

/// GetPrototypeFromConstructor(constructor, intrinsicDefaultProto) (§10.1.14)
pub fn get_prototype_from_constructor(
    agent: &mut Agent,
    constructor: Object,
    default: &str,
) -> JsResult<Object> {
    // 2. Let proto be ? Get(constructor, "prototype").
    let proto = get(
        agent,
        constructor,
        PropertyKey::String(agent.names.prototype),
    )?;
    // 3. If proto is not an Object, then
    if let Value::Object(p) = proto {
        return Ok(p);
    }
    // a. Let realm be ? GetFunctionRealm(constructor).
    let realm = crate::function::get_function_realm(agent, constructor)?;
    // b. Set proto to realm's intrinsic object named intrinsicDefaultProto.
    Ok(agent.intrinsic_in(realm, default))
}

/// OrdinaryCreateFromConstructor(constructor, intrinsicDefaultProto, kind) (§10.1.13)
pub fn ordinary_create_from_constructor(
    agent: &mut Agent,
    constructor: Object,
    default: &str,
    kind: ObjectKind,
) -> JsResult<Object> {
    // 2. Let proto be ? GetPrototypeFromConstructor(constructor, intrinsicDefaultProto).
    let proto = get_prototype_from_constructor(agent, constructor, default)?;
    // 3-4. Return OrdinaryObjectCreate(proto, internalSlotsList).
    Ok(agent.object_with_kind(Some(proto), kind))
}

/// FromPropertyDescriptor(Desc) (§6.2.6.4)
pub fn from_property_descriptor(agent: &mut Agent, desc: Option<PropertyDescriptor>) -> Value {
    // 1. If Desc is undefined, return undefined.
    let Some(desc) = desc else {
        return Value::Undefined;
    };
    // 2. Let obj be OrdinaryObjectCreate(%Object.prototype%).
    let proto = agent.intrinsic("%Object.prototype%");
    let obj = agent.ordinary_object_create(Some(proto));
    let names = agent.names;
    let put = |agent: &mut Agent, name, v: Value| {
        let _ = create_data_property(agent, obj, PropertyKey::String(name), v);
    };
    // 4-9. Copy each present field.
    if let Some(v) = desc.value {
        put(agent, names.value, v);
    }
    if let Some(w) = desc.writable {
        put(agent, names.writable, Value::Boolean(w));
    }
    if let Some(g) = desc.get {
        put(agent, names.get, g);
    }
    if let Some(s) = desc.set {
        put(agent, names.set, s);
    }
    if let Some(e) = desc.enumerable {
        put(agent, names.enumerable, Value::Boolean(e));
    }
    if let Some(c) = desc.configurable {
        put(agent, names.configurable, Value::Boolean(c));
    }
    // 10. Return obj.
    Value::Object(obj)
}

/// ToPropertyDescriptor(Obj) (§6.2.6.5)
pub fn to_property_descriptor(agent: &mut Agent, v: Value) -> JsResult<PropertyDescriptor> {
    // 1. If Obj is not an Object, throw a TypeError exception.
    let Value::Object(o) = v else {
        return Err(agent.throw_type_error("Property description must be an object"));
    };
    let names = agent.names;
    let mut desc = PropertyDescriptor::default();
    // 3-4. enumerable
    if has_property(agent, o, PropertyKey::String(names.enumerable))? {
        let e = get(agent, o, PropertyKey::String(names.enumerable))?;
        desc.enumerable = Some(to_boolean(agent, e));
    }
    // 5-6. configurable
    if has_property(agent, o, PropertyKey::String(names.configurable))? {
        let c = get(agent, o, PropertyKey::String(names.configurable))?;
        desc.configurable = Some(to_boolean(agent, c));
    }
    // 7-8. value
    if has_property(agent, o, PropertyKey::String(names.value))? {
        desc.value = Some(get(agent, o, PropertyKey::String(names.value))?);
    }
    // 9-10. writable
    if has_property(agent, o, PropertyKey::String(names.writable))? {
        let w = get(agent, o, PropertyKey::String(names.writable))?;
        desc.writable = Some(to_boolean(agent, w));
    }
    // 11-12. get: must be callable or undefined.
    if has_property(agent, o, PropertyKey::String(names.get))? {
        let g = get(agent, o, PropertyKey::String(names.get))?;
        if !is_callable(agent, g) && !g.is_undefined() {
            return Err(agent.throw_type_error("Getter must be a function"));
        }
        desc.get = Some(g);
    }
    // 13-14. set: must be callable or undefined.
    if has_property(agent, o, PropertyKey::String(names.set))? {
        let s = get(agent, o, PropertyKey::String(names.set))?;
        if !is_callable(agent, s) && !s.is_undefined() {
            return Err(agent.throw_type_error("Setter must be a function"));
        }
        desc.set = Some(s);
    }
    // 15. If desc has a [[Get]] or [[Set]] field, it may not have [[Value]] or [[Writable]].
    if (desc.get.is_some() || desc.set.is_some())
        && (desc.value.is_some() || desc.writable.is_some())
    {
        return Err(agent.throw_type_error("Invalid property descriptor: cannot both specify accessors and a value or writable attribute"));
    }
    // 16. Return desc.
    Ok(desc)
}

/// CompletePropertyDescriptor(Desc) (§6.2.6.6)
pub fn complete_property_descriptor(desc: &mut PropertyDescriptor) {
    if desc.is_generic_descriptor() || desc.is_data_descriptor() {
        desc.value.get_or_insert(Value::Undefined);
        desc.writable.get_or_insert(false);
    } else {
        desc.get.get_or_insert(Value::Undefined);
        desc.set.get_or_insert(Value::Undefined);
    }
    desc.enumerable.get_or_insert(false);
    desc.configurable.get_or_insert(false);
}

/// CopyDataProperties(target, source, excludedItems) (§7.3.25)
pub fn copy_data_properties(
    agent: &mut Agent,
    target: Object,
    source: Value,
    excluded: &[PropertyKey],
) -> JsResult<()> {
    // 1. If source is either undefined or null, return unused.
    if source.is_nullish() {
        return Ok(());
    }
    // 2. Let from be ! ToObject(source).
    let from = to_object(agent, source)?;
    // 3. Let keys be ? from.[[OwnPropertyKeys]]().
    let keys = from.internal_own_property_keys(agent)?;
    // 4. For each element nextKey of keys, do
    for next_key in keys {
        // a. If nextKey is not an element of excludedItems, then
        if excluded.contains(&next_key) {
            continue;
        }
        // i. Let desc be ? from.[[GetOwnProperty]](nextKey).
        // ii. If desc is not undefined and desc.[[Enumerable]] is true, then
        if let Some(desc) = from.internal_get_own_property(agent, next_key)?
            && desc.enumerable == Some(true)
        {
            // 1. Let propValue be ? Get(from, nextKey).
            let value = get(agent, from, next_key)?;
            // 2. Perform ! CreateDataPropertyOrThrow(target, nextKey, propValue).
            create_data_property_or_throw(agent, target, next_key, value)?;
        }
    }
    Ok(())
}

/// ArraySpeciesCreate(originalArray, length) (§10.4.2.3)
pub fn array_species_create(agent: &mut Agent, original: Object, length: f64) -> JsResult<Object> {
    // 1. Let isArray be ? IsArray(originalArray).
    // 2. If isArray is false, return ? ArrayCreate(length).
    if !crate::ops::is_array(agent, Value::Object(original))? {
        return array_create(agent, length, None);
    }
    // 3. Let C be ? Get(originalArray, "constructor").
    let mut c = get(
        agent,
        original,
        PropertyKey::String(agent.names.constructor),
    )?;
    // 4. If IsConstructor(C) is true, then
    if let Value::Object(co) = c
        && is_constructor(agent, c)
    {
        // a. Let thisRealm be the current Realm Record.
        let this_realm = agent.current_realm();
        // b. Let realmC be ? GetFunctionRealm(C).
        let realm_c = crate::function::get_function_realm(agent, co)?;
        // c. If thisRealm and realmC are not the same Realm Record, then
        //    i. If SameValue(C, realmC.[[Intrinsics]].[[%Array%]]) is true, set C to undefined.
        if this_realm != realm_c && agent.intrinsic_in(realm_c, "%Array%") == co {
            c = Value::Undefined;
        }
    }
    // 5. If C is an Object, then
    if let Value::Object(co) = c {
        // a. Set C to ? Get(C, %Symbol.species%).
        c = get(agent, co, PropertyKey::Symbol(agent.symbols_wk.species))?;
        // b. If C is null, set C to undefined.
        if c.is_null() {
            c = Value::Undefined;
        }
    }
    // 6. If C is undefined, return ? ArrayCreate(length).
    if c.is_undefined() {
        return array_create(agent, length, None);
    }
    // 7. If IsConstructor(C) is false, throw a TypeError exception.
    let (Value::Object(co), true) = (c, is_constructor(agent, c)) else {
        return Err(agent.throw_type_error("species is not a constructor"));
    };
    // 8. Return ? Construct(C, « 𝔽(length) »).
    construct(agent, co, &[Value::Number(length)], None)
}

/// ToPropertyKey wrapper used by the VM.
pub fn property_key_of(agent: &mut Agent, v: Value) -> JsResult<PropertyKey> {
    to_property_key(agent, v)
}
