//! Function objects: ECMAScript functions (§10.2), built-in functions
//! (§10.3), bound functions (§10.4.1) and arguments objects (§10.4.4).

use std::sync::Arc;

use crate::agent::{Agent, ExecutionContext, MAX_CALL_DEPTH};
use crate::env::EnvId;
use crate::error::JsResult;
use crate::object::internal::{
    ordinary_define_own_property, ordinary_delete, ordinary_get, ordinary_get_own_property,
    ordinary_set,
};
use crate::object::{ObjectData, ObjectKind, Property, PropertyDescriptor, PropertyKey};
use crate::ops;
use crate::realm::RealmId;
use crate::value::{JsString, Object, Value};

/// The behaviour of a built-in function: (agent, this, arguments, NewTarget).
/// NewTarget is `None` for [[Call]] and the constructor/new.target for [[Construct]].
pub type NativeFn = fn(&mut Agent, Value, &[Value], Option<Object>) -> JsResult<Value>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThisMode {
    Lexical,
    Strict,
    Global,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstructorKind {
    Base,
    Derived,
}

/// A Private Name (§6.2.12): unique per class evaluation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PrivateName {
    pub id: u32,
    /// [[Description]] (the name without `#`).
    pub description: JsString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivateElementKind {
    Field,
    Method,
    Accessor,
}

/// PrivateElement (§6.2.10)
#[derive(Debug, Clone, Copy)]
pub struct PrivateElement {
    pub key: PrivateName,
    pub kind: PrivateElementKind,
    pub value: Value,
    pub get: Value,
    pub set: Value,
}

#[derive(Debug, Clone, Copy)]
pub enum FieldName {
    Key(PropertyKey),
    Private(PrivateName),
}

/// ClassFieldDefinition (§6.2.11)
#[derive(Debug, Clone, Copy)]
pub struct ClassFieldDefinition {
    pub name: FieldName,
    /// A function evaluating the initializer, or None.
    pub initializer: Option<Object>,
    /// Static blocks are stored as fields with no name.
    pub is_static_block: bool,
}

/// The internal slots of an ECMAScript function object (Table 30).
#[derive(Debug)]
pub struct EcmaFunction {
    pub code: Arc<crate::compiler::FunctionCode>,
    /// [[Environment]]
    pub environment: EnvId,
    /// [[PrivateEnvironment]]: private names visible to the code.
    pub private_environment: Option<Arc<crate::vm::PrivateEnv>>,
    /// [[HomeObject]]
    pub home_object: Option<Object>,
    /// [[ThisMode]]
    pub this_mode: ThisMode,
    /// [[ConstructorKind]]
    pub constructor_kind: ConstructorKind,
    /// [[IsClassConstructor]]
    pub is_class_constructor: bool,
    /// Whether MakeConstructor was performed (it has [[Construct]]).
    pub constructor: bool,
    /// [[Realm]]
    pub realm: RealmId,
    /// [[ScriptOrModule]]
    pub script_or_module: Option<crate::ScriptOrModule>,
    /// [[Fields]]
    pub fields: Vec<ClassFieldDefinition>,
    /// [[PrivateMethods]]
    pub private_methods: Vec<PrivateElement>,
    /// [[ClassFieldInitializerName]] (for anonymous function naming in fields).
    pub class_field_initializer_name: Option<FieldName>,
    /// The class source text, for class constructors' toString.
    pub source_override: Option<(Arc<str>, u32, u32)>,
    /// Static elements of a class still being defined (run by FinishClass).
    pub class_pending: Option<Box<ClassPending>>,
}

/// The static parts of ClassDefinitionEvaluation that run after all
/// elements are defined.
#[derive(Debug, Default)]
pub struct ClassPending {
    pub static_private_methods: Vec<PrivateElement>,
    pub static_elements: Vec<ClassFieldDefinition>,
}

impl EcmaFunction {
    pub fn is_constructor(&self) -> bool {
        self.constructor
    }
}

/// The internal slots of a built-in function object (§10.3).
#[derive(Debug, Clone)]
pub struct BuiltinFunction {
    pub behaviour: NativeFn,
    /// [[Realm]]
    pub realm: RealmId,
    /// [[InitialName]]
    pub initial_name: Option<JsString>,
    pub constructor: bool,
    /// Additional internal slots requested by CreateBuiltinFunction
    /// (e.g. a promise resolving function's [[Promise]]).
    pub slots: Vec<Value>,
}

/// Bound function exotic object slots (§10.4.1).
#[derive(Debug, Clone)]
pub struct BoundFunction {
    pub target: Object,
    pub bound_this: Value,
    pub bound_args: Vec<Value>,
}

/// The parameter map of a mapped arguments object (§10.4.4).
#[derive(Debug, Clone)]
pub struct MappedArguments {
    pub env: EnvId,
    /// For each argument index, the parameter name it is still mapped to.
    pub mapped: Vec<Option<JsString>>,
}

/// CreateBuiltinFunction(behaviour, length, name, additionalInternalSlotsList
/// [, realm [, prototype [, prefix]]]) (§10.3.4). `name` may be a symbol key.
#[allow(clippy::too_many_arguments)]
pub fn create_builtin_function(
    agent: &mut Agent,
    behaviour: NativeFn,
    length: u32,
    name: PropertyKey,
    slots: Vec<Value>,
    realm: Option<RealmId>,
    prototype: Option<Object>,
    prefix: Option<&str>,
    constructor: bool,
) -> Object {
    // 1. If realm is not present, set realm to the current Realm Record.
    let realm = realm.unwrap_or_else(|| agent.current_realm());
    // 2. If prototype is not present, set prototype to realm.[[Intrinsics]].[[%Function.prototype%]].
    let prototype = prototype.or_else(|| {
        agent
            .realm(realm)
            .intrinsics
            .get("%Function.prototype%")
            .copied()
    });
    // 3-9. Let func be a new built-in function object with the requested slots.
    let func = agent.alloc(ObjectData::new(
        prototype,
        ObjectKind::Builtin(Box::new(BuiltinFunction {
            behaviour,
            realm,
            initial_name: None,
            constructor,
            slots,
        })),
    ));
    // 10. Perform SetFunctionLength(func, length).
    set_function_length(agent, func, length as f64);
    // 11-12. Perform SetFunctionName(func, name, prefix).
    set_function_name(agent, func, name, prefix);
    // [[InitialName]] is the "name" value.
    if let Some(Property::Data {
        value: Value::String(s),
        ..
    }) = agent
        .obj(func)
        .properties
        .get(&PropertyKey::String(agent.names.name))
        .copied()
        && let ObjectKind::Builtin(b) = &mut agent.obj_mut(func).kind
    {
        b.initial_name = Some(s);
    }
    func
}

/// BoundFunctionCreate(targetFunction, boundThis, boundArgs) (§10.4.1.3)
pub fn bound_function_create(
    agent: &mut Agent,
    target: Object,
    bound_this: Value,
    bound_args: Vec<Value>,
) -> JsResult<Object> {
    // 1. Let proto be ? targetFunction.[[GetPrototypeOf]]().
    let proto = target.internal_get_prototype_of(agent)?;
    // 2-9. A new bound function exotic object with [[BoundTargetFunction]],
    // [[BoundThis]] and [[BoundArguments]]; it has [[Construct]] exactly
    // when the target does (Object::is_constructor checks the target).
    Ok(agent.alloc(ObjectData::new(
        proto,
        ObjectKind::BoundFunction(Box::new(BoundFunction {
            target,
            bound_this,
            bound_args,
        })),
    )))
}

/// Shorthand for the common case: a function in the current realm.
pub fn builtin(agent: &mut Agent, behaviour: NativeFn, length: u32, name: &str) -> Object {
    let key = agent.key(name);
    create_builtin_function(
        agent,
        behaviour,
        length,
        key,
        Vec::new(),
        None,
        None,
        None,
        false,
    )
}

/// The additional internal slots of the active built-in function.
pub fn builtin_slots(agent: &Agent, f: Object) -> &[Value] {
    match &agent.obj(f).kind {
        ObjectKind::Builtin(b) => &b.slots,
        _ => &[],
    }
}

pub fn set_builtin_slot(agent: &mut Agent, f: Object, i: usize, v: Value) {
    if let ObjectKind::Builtin(b) = &mut agent.obj_mut(f).kind
        && let Some(slot) = b.slots.get_mut(i)
    {
        *slot = v;
    }
}

/// SetFunctionName(F, name [, prefix]) (§10.2.9)
pub fn set_function_name(agent: &mut Agent, f: Object, name: PropertyKey, prefix: Option<&str>) {
    // 2. If name is a Symbol, then
    let mut name_str = match name {
        PropertyKey::Symbol(s) => {
            // a. Let description be name's [[Description]] value.
            // b. If description is undefined, set name to the empty String.
            // c. Else, set name to the string-concatenation of "[", description, and "]".
            match agent.symbol_description(s) {
                Some(d) => {
                    let mut v: Vec<u16> = "[".encode_utf16().collect();
                    v.extend_from_slice(agent.str(d));
                    v.push(']' as u16);
                    agent.intern(&v)
                }
                None => agent.names.empty,
            }
        }
        PropertyKey::String(s) => s,
        PropertyKey::Index(i) => agent.string(&i.to_string()),
    };
    // 5. If prefix is present, then set name to the string-concatenation of prefix, " ", and name.
    if let Some(prefix) = prefix {
        let p = agent.string(&format!("{prefix} "));
        name_str = agent.concat(p, name_str);
    }
    // 6. Perform ! DefinePropertyOrThrow(F, "name", PropertyDescriptor { [[Value]]: name,
    //    [[Writable]]: false, [[Enumerable]]: false, [[Configurable]]: true }).
    let key = PropertyKey::String(agent.names.name);
    agent.obj_mut(f).properties.insert(
        key,
        Property::Data {
            value: Value::String(name_str),
            writable: false,
            enumerable: false,
            configurable: true,
        },
    );
}

/// SetFunctionLength(F, length) (§10.2.10)
pub fn set_function_length(agent: &mut Agent, f: Object, length: f64) {
    // 2. Perform ! DefinePropertyOrThrow(F, "length", PropertyDescriptor { [[Value]]: 𝔽(length),
    //    [[Writable]]: false, [[Enumerable]]: false, [[Configurable]]: true }).
    let key = PropertyKey::String(agent.names.length);
    agent.obj_mut(f).properties.insert(
        key,
        Property::Data {
            value: Value::Number(length),
            writable: false,
            enumerable: false,
            configurable: true,
        },
    );
}

/// MakeConstructor(F [, writablePrototype [, prototype]]) (§10.2.5)
pub fn make_constructor(
    agent: &mut Agent,
    f: Object,
    writable_prototype: bool,
    prototype: Option<Object>,
) {
    // 1-3. Set F.[[Construct]] (record that it is a constructor).
    match &mut agent.obj_mut(f).kind {
        ObjectKind::Function(e) => e.constructor = true,
        ObjectKind::Builtin(b) => b.constructor = true,
        _ => {}
    }
    // 4. If prototype is not present, then
    let prototype = match prototype {
        Some(p) => p,
        None => {
            // a. Set prototype to OrdinaryObjectCreate(%Object.prototype%).
            let op = agent.intrinsic("%Object.prototype%");
            let p = agent.ordinary_object_create(Some(op));
            // b. Perform ! DefinePropertyOrThrow(prototype, "constructor", PropertyDescriptor
            //    { [[Value]]: F, [[Writable]]: writablePrototype, [[Enumerable]]: false, [[Configurable]]: true }).
            let key = PropertyKey::String(agent.names.constructor);
            agent.obj_mut(p).properties.insert(
                key,
                Property::Data {
                    value: Value::Object(f),
                    writable: writable_prototype,
                    enumerable: false,
                    configurable: true,
                },
            );
            p
        }
    };
    // 5. Perform ! DefinePropertyOrThrow(F, "prototype", PropertyDescriptor { [[Value]]: prototype,
    //    [[Writable]]: writablePrototype, [[Enumerable]]: false, [[Configurable]]: false }).
    let key = PropertyKey::String(agent.names.prototype);
    agent.obj_mut(f).properties.insert(
        key,
        Property::Data {
            value: Value::Object(prototype),
            writable: writable_prototype,
            enumerable: false,
            configurable: false,
        },
    );
}

/// GetFunctionRealm(obj) (§7.3.24)
pub fn get_function_realm(agent: &mut Agent, f: Object) -> JsResult<RealmId> {
    match &agent.obj(f).kind {
        // 1. If obj has a [[Realm]] internal slot, return obj.[[Realm]].
        ObjectKind::Function(e) => Ok(e.realm),
        ObjectKind::Builtin(b) => Ok(b.realm),
        // 2. If obj is a bound function exotic object, then return ? GetFunctionRealm(target).
        ObjectKind::BoundFunction(b) => {
            let target = b.target;
            get_function_realm(agent, target)
        }
        _ => {
            // 3. If obj is a Proxy exotic object, then
            if let Some(p) = crate::object::proxy::proxy_data(agent, f) {
                // a. Perform ? ValidateNonRevokedProxy(obj).
                let Some(target) = p.target else {
                    return Err(agent.throw_type_error("proxy has been revoked"));
                };
                // b. Let proxyTarget be obj.[[ProxyTarget]].
                // c. Return ? GetFunctionRealm(proxyTarget).
                return get_function_realm(agent, target);
            }
            // 4. Return the current Realm Record.
            Ok(agent.current_realm())
        }
    }
}

fn enter_call(agent: &mut Agent) -> JsResult<()> {
    agent.call_depth += 1;
    if agent.call_depth > MAX_CALL_DEPTH {
        agent.call_depth -= 1;
        return Err(agent.throw_range_error("Maximum call stack size exceeded"));
    }
    Ok(())
}

impl Object {
    /// F.[[Call]](thisArgument, argumentsList). Rust code calling into
    /// JavaScript holds its unrooted handles: nothing is collected until the
    /// call returns (docs/GC.md).
    pub fn internal_call(self, agent: &mut Agent, this: Value, args: &[Value]) -> JsResult<Value> {
        agent.holding(|agent| self.internal_call_rooted(agent, this, args))
    }

    /// F.[[Call]] for a caller that keeps `this`, `args` and anything it uses
    /// afterwards rooted (the interpreter's calls, jobs, tail calls): the
    /// JavaScript it runs may collect garbage.
    pub fn internal_call_rooted(
        self,
        agent: &mut Agent,
        this: Value,
        args: &[Value],
    ) -> JsResult<Value> {
        enter_call(agent)?;
        let result = self.call_inner(agent, this, args);
        agent.call_depth -= 1;
        result
    }

    fn call_inner(self, agent: &mut Agent, this: Value, args: &[Value]) -> JsResult<Value> {
        match &agent.obj(self).kind {
            ObjectKind::Builtin(b) => {
                let (behaviour, realm) = (b.behaviour, b.realm);
                // §10.3.1: a new execution context with Function = F and Realm = F.[[Realm]].
                agent.contexts.push(ExecutionContext {
                    function: Some(self),
                    realm,
                    script_or_module: None,
                });
                let r = behaviour(agent, this, args, None);
                agent.contexts.pop();
                r
            }
            ObjectKind::Function(_) => crate::vm::call_ecma_function(agent, self, this, args),
            ObjectKind::BoundFunction(b) => {
                // §10.4.1.1: Call(target, boundThis, boundArgs ++ args)
                let (target, bound_this) = (b.target, b.bound_this);
                let mut all = b.bound_args.clone();
                all.extend_from_slice(args);
                // A tail call: the arguments are copied into the target's frame.
                target.internal_call_rooted(agent, bound_this, &all)
            }
            _ => {
                if let Some(r) = crate::object::proxy::call(agent, self, this, args) {
                    return r;
                }
                Err(agent.throw_type_error("not a function"))
            }
        }
    }

    /// F.[[Construct]](argumentsList, newTarget)
    pub fn internal_construct(
        self,
        agent: &mut Agent,
        args: &[Value],
        new_target: Object,
    ) -> JsResult<Object> {
        agent.holding(|agent| self.internal_construct_rooted(agent, args, new_target))
    }

    /// F.[[Construct]] for a caller that keeps its handles rooted (see
    /// `internal_call_rooted`).
    pub fn internal_construct_rooted(
        self,
        agent: &mut Agent,
        args: &[Value],
        new_target: Object,
    ) -> JsResult<Object> {
        enter_call(agent)?;
        let result = self.construct_inner(agent, args, new_target);
        agent.call_depth -= 1;
        result
    }

    fn construct_inner(
        self,
        agent: &mut Agent,
        args: &[Value],
        new_target: Object,
    ) -> JsResult<Object> {
        match &agent.obj(self).kind {
            ObjectKind::Builtin(b) => {
                let (behaviour, realm) = (b.behaviour, b.realm);
                agent.contexts.push(ExecutionContext {
                    function: Some(self),
                    realm,
                    script_or_module: None,
                });
                let r = behaviour(agent, Value::Undefined, args, Some(new_target));
                agent.contexts.pop();
                match r? {
                    Value::Object(o) => Ok(o),
                    _ => {
                        Err(agent.throw_type_error("built-in constructor did not return an object"))
                    }
                }
            }
            ObjectKind::Function(_) => {
                crate::vm::construct_ecma_function(agent, self, args, new_target)
            }
            ObjectKind::BoundFunction(b) => {
                // §10.4.1.2
                let target = b.target;
                let mut all = b.bound_args.clone();
                all.extend_from_slice(args);
                // 5. If SameValue(F, newTarget) is true, set newTarget to target.
                let new_target = if new_target == self {
                    target
                } else {
                    new_target
                };
                target.internal_construct_rooted(agent, &all, new_target)
            }
            _ => {
                if let Some(r) = crate::object::proxy::construct(agent, self, args, new_target) {
                    return r;
                }
                Err(agent.throw_type_error("not a constructor"))
            }
        }
    }
}

// ---- arguments exotic objects (§10.4.4) ---------------------------------------

fn mapped_name(agent: &Agent, o: Object, key: PropertyKey) -> Option<(EnvId, JsString)> {
    let ObjectKind::Arguments(Some(m)) = &agent.obj(o).kind else {
        return None;
    };
    let PropertyKey::Index(i) = key else {
        return None;
    };
    m.mapped
        .get(i as usize)
        .copied()
        .flatten()
        .map(|n| (m.env, n))
}

fn unmap(agent: &mut Agent, o: Object, key: PropertyKey) {
    if let (ObjectKind::Arguments(Some(m)), PropertyKey::Index(i)) =
        (&mut agent.obj_mut(o).kind, key)
        && let Some(slot) = m.mapped.get_mut(i as usize)
    {
        *slot = None;
    }
}

/// [[GetOwnProperty]] (§10.4.4.1)
pub fn arguments_get_own_property(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
) -> Option<PropertyDescriptor> {
    // 1. Let desc be OrdinaryGetOwnProperty(args, P).
    // 2. If desc is undefined, return undefined.
    let mut desc = ordinary_get_own_property(agent, o, key)?;
    // 3. Let map be args.[[ParameterMap]].
    // 4. Let isMapped be ! HasOwnProperty(map, P).
    // 5. If isMapped is true, then
    if let Some((env, name)) = mapped_name(agent, o, key) {
        // a. Set desc.[[Value]] to ! Get(map, P).
        desc.value = env.get_binding_value(agent, name, false).ok();
    }
    // 6. Return desc.
    Some(desc)
}

/// [[DefineOwnProperty]] (§10.4.4.2)
pub fn arguments_define_own_property(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    desc: PropertyDescriptor,
) -> JsResult<bool> {
    // 2. Let isMapped be ! HasOwnProperty(map, P).
    let mapped = mapped_name(agent, o, key);
    // 3. Let newArgDesc be Desc.
    let mut new_arg_desc = desc;
    // 4. If isMapped is true and IsDataDescriptor(Desc) is true, then
    if let Some((env, name)) = mapped
        && desc.is_data_descriptor()
        // a. If Desc does not have a [[Value]] field, Desc has a [[Writable]] field, and Desc.[[Writable]] is false, then
        && desc.value.is_none()
        && desc.writable == Some(false)
    {
        // i. Set newArgDesc to a copy of Desc with [[Value]] = ! Get(map, P).
        new_arg_desc.value = Some(env.get_binding_value(agent, name, false)?);
    }
    // 5. Let allowed be ! OrdinaryDefineOwnProperty(args, P, newArgDesc).
    // 6. If allowed is false, return false.
    if !ordinary_define_own_property(agent, o, key, new_arg_desc)? {
        return Ok(false);
    }
    // 7. If isMapped is true, then
    if let Some((env, name)) = mapped {
        if desc.is_accessor_descriptor() {
            // a. If IsAccessorDescriptor(Desc) is true, then i. Perform ! map.[[Delete]](P).
            unmap(agent, o, key);
        } else {
            // b. Else, i. If Desc has a [[Value]] field, then perform ! Set(map, P, Desc.[[Value]], false).
            if let Some(v) = desc.value {
                env.set_mutable_binding(agent, name, v, false)?;
            }
            // ii. If Desc has a [[Writable]] field and Desc.[[Writable]] is false, then perform ! map.[[Delete]](P).
            if desc.writable == Some(false) {
                unmap(agent, o, key);
            }
        }
    }
    // 8. Return true.
    Ok(true)
}

/// [[Get]] (§10.4.4.3): `Some` when the property is mapped.
pub fn arguments_get(agent: &mut Agent, o: Object, key: PropertyKey) -> JsResult<Option<Value>> {
    match mapped_name(agent, o, key) {
        // 4. Else, a. Assert: map contains a formal parameter mapping for P. b. Return ! Get(map, P).
        Some((env, name)) => Ok(Some(env.get_binding_value(agent, name, false)?)),
        // 3. If isMapped is false, then return ? OrdinaryGet(args, P, Receiver).
        None => Ok(None),
    }
}

/// [[Set]] (§10.4.4.4)
pub fn arguments_set(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    value: Value,
    receiver: Value,
) -> JsResult<bool> {
    // 1. If SameValue(args, Receiver) is false, then isMapped is false.
    let same = matches!(receiver, Value::Object(r) if r == o);
    if same && let Some((env, name)) = mapped_name(agent, o, key) {
        // 3. If isMapped is true, then perform ! Set(map, P, V, false).
        env.set_mutable_binding(agent, name, value, false)?;
    }
    // 4. Return ? OrdinarySet(args, P, V, Receiver).
    ordinary_set(agent, o, key, value, receiver)
}

/// [[Delete]] (§10.4.4.5)
pub fn arguments_delete(agent: &mut Agent, o: Object, key: PropertyKey) -> JsResult<bool> {
    // 3. Let result be ? OrdinaryDelete(args, P).
    let result = ordinary_delete(agent, o, key)?;
    // 4. If result is true and isMapped is true, then perform ! map.[[Delete]](P).
    if result {
        unmap(agent, o, key);
    }
    // 5. Return result.
    Ok(result)
}

/// OrdinaryGet without the arguments-object special case (used by the VM).
pub fn plain_get(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    receiver: Value,
) -> JsResult<Value> {
    ordinary_get(agent, o, key, receiver)
}

/// CreateUnmappedArgumentsObject(argumentsList) (§10.4.4.6)
pub fn create_unmapped_arguments_object(agent: &mut Agent, args: &[Value]) -> JsResult<Object> {
    let proto = agent.intrinsic("%Object.prototype%");
    let obj = agent.alloc(ObjectData::new(Some(proto), ObjectKind::Arguments(None)));
    let len_key = PropertyKey::String(agent.names.length);
    agent.obj_mut(obj).properties.insert(
        len_key,
        Property::Data {
            value: Value::Number(args.len() as f64),
            writable: true,
            enumerable: false,
            configurable: true,
        },
    );
    for (i, v) in args.iter().enumerate() {
        let key = agent.key_from_index(i as f64);
        ops::create_data_property_or_throw(agent, obj, key, *v)?;
    }
    let iter_key = PropertyKey::Symbol(agent.symbols_wk.iterator);
    let values = agent.intrinsic("%Array.prototype.values%");
    agent.obj_mut(obj).properties.insert(
        iter_key,
        Property::Data {
            value: Value::Object(values),
            writable: true,
            enumerable: false,
            configurable: true,
        },
    );
    let thrower = agent.intrinsic("%ThrowTypeError%");
    let callee_key = PropertyKey::String(agent.names.callee);
    agent.obj_mut(obj).properties.insert(
        callee_key,
        Property::Accessor {
            get: Value::Object(thrower),
            set: Value::Object(thrower),
            enumerable: false,
            configurable: false,
        },
    );
    Ok(obj)
}

/// CreateMappedArgumentsObject(func, formals, argumentsList, env) (§10.4.4.7)
pub fn create_mapped_arguments_object(
    agent: &mut Agent,
    func: Object,
    formals: &[JsString],
    args: &[Value],
    env: EnvId,
) -> JsResult<Object> {
    let proto = agent.intrinsic("%Object.prototype%");
    let mut mapped = vec![None; args.len()];
    // 15-17. Map each argument index to the last formal with that name, if any.
    let mut seen: Vec<JsString> = Vec::new();
    for (index, name) in formals.iter().enumerate().rev() {
        if !seen.contains(name) {
            seen.push(*name);
            if index < args.len() {
                mapped[index] = Some(*name);
            }
        }
    }
    let obj = agent.alloc(ObjectData::new(
        Some(proto),
        ObjectKind::Arguments(Some(Box::new(MappedArguments { env, mapped }))),
    ));
    for (i, v) in args.iter().enumerate() {
        let key = agent.key_from_index(i as f64);
        agent.obj_mut(obj).properties.insert(
            key,
            Property::Data {
                value: *v,
                writable: true,
                enumerable: true,
                configurable: true,
            },
        );
    }
    let len_key = PropertyKey::String(agent.names.length);
    agent.obj_mut(obj).properties.insert(
        len_key,
        Property::Data {
            value: Value::Number(args.len() as f64),
            writable: true,
            enumerable: false,
            configurable: true,
        },
    );
    let iter_key = PropertyKey::Symbol(agent.symbols_wk.iterator);
    let values = agent.intrinsic("%Array.prototype.values%");
    agent.obj_mut(obj).properties.insert(
        iter_key,
        Property::Data {
            value: Value::Object(values),
            writable: true,
            enumerable: false,
            configurable: true,
        },
    );
    let callee_key = PropertyKey::String(agent.names.callee);
    agent.obj_mut(obj).properties.insert(
        callee_key,
        Property::Data {
            value: Value::Object(func),
            writable: true,
            enumerable: false,
            configurable: true,
        },
    );
    Ok(obj)
}

// ---- garbage collection ----------------------------------------------------

use crate::gc::{Trace, Tracer};

impl Trace for PrivateName {
    fn trace(&self, t: &mut Tracer) {
        t.string(self.description);
    }
}

impl Trace for PrivateElement {
    fn trace(&self, t: &mut Tracer) {
        self.key.trace(t);
        t.value(self.value);
        t.value(self.get);
        t.value(self.set);
    }
}

impl Trace for FieldName {
    fn trace(&self, t: &mut Tracer) {
        match self {
            FieldName::Key(k) => t.key(*k),
            FieldName::Private(p) => p.trace(t),
        }
    }
}

impl Trace for ClassFieldDefinition {
    fn trace(&self, t: &mut Tracer) {
        self.name.trace(t);
        self.initializer.trace(t);
    }
}

impl Trace for ClassPending {
    fn trace(&self, t: &mut Tracer) {
        self.static_private_methods.trace(t);
        self.static_elements.trace(t);
    }
}

impl Trace for EcmaFunction {
    fn trace(&self, t: &mut Tracer) {
        // Realms and scripts are not collected.
        t.code(&self.code);
        t.env(self.environment);
        self.private_environment.trace(t);
        self.home_object.trace(t);
        self.fields.trace(t);
        self.private_methods.trace(t);
        self.class_field_initializer_name.trace(t);
        self.class_pending.trace(t);
    }
}

impl Trace for BuiltinFunction {
    fn trace(&self, t: &mut Tracer) {
        self.initial_name.trace(t);
        self.slots.trace(t);
    }
}

impl Trace for BoundFunction {
    fn trace(&self, t: &mut Tracer) {
        t.object(self.target);
        t.value(self.bound_this);
        self.bound_args.trace(t);
    }
}

impl Trace for MappedArguments {
    fn trace(&self, t: &mut Tracer) {
        t.env(self.env);
        self.mapped.trace(t);
    }
}
