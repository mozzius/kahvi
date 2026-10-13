//! Installs the intrinsic objects into a realm (CreateIntrinsics §9.3.2,
//! SetDefaultGlobalBindings §9.3.4) from the generated function table plus
//! the hand-written object shapes below.

use kahvi_core::function::{NativeFn, create_builtin_function};
use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;

use crate::generated::FUNCTIONS;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Key {
    Str(&'static str),
    /// A well-known symbol by its name without "Symbol." (e.g. "iterator").
    Symbol(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    Method,
    Getter,
    Setter,
    Constructor,
}

/// One built-in function property.
pub(crate) struct FnEntry {
    /// The intrinsic it is a property of; "%GlobalObject%" for globals and
    /// "%Intrinsic%" for constructors that are not global properties.
    pub owner: &'static str,
    pub key: Key,
    pub kind: Kind,
    pub length: u32,
    pub writable: bool,
    pub enumerable: bool,
    pub configurable: bool,
    pub behaviour: NativeFn,
}

#[derive(Clone, Copy)]
enum Shape {
    Ordinary,
    ImmutablePrototype,
    /// A built-in function that returns undefined (%Function.prototype%).
    Function,
    Array,
    Boolean,
    Number,
    String,
}

const OP: Option<&str> = Some("%Object.prototype%");
const ITER: Option<&str> = Some("%Iterator.prototype%");

/// Intrinsic objects whose [[Prototype]] or kind is not the default
/// (an ordinary object inheriting from %Object.prototype%), in creation order.
const OBJECTS: &[(&str, Option<&str>, Shape)] = &[
    ("%Object.prototype%", None, Shape::ImmutablePrototype),
    ("%Function.prototype%", OP, Shape::Function),
    ("%Array.prototype%", OP, Shape::Array),
    ("%Boolean.prototype%", OP, Shape::Boolean),
    ("%Number.prototype%", OP, Shape::Number),
    ("%String.prototype%", OP, Shape::String),
    ("%Error.prototype%", OP, Shape::Ordinary),
    (
        "%EvalError.prototype%",
        Some("%Error.prototype%"),
        Shape::Ordinary,
    ),
    (
        "%RangeError.prototype%",
        Some("%Error.prototype%"),
        Shape::Ordinary,
    ),
    (
        "%ReferenceError.prototype%",
        Some("%Error.prototype%"),
        Shape::Ordinary,
    ),
    (
        "%SyntaxError.prototype%",
        Some("%Error.prototype%"),
        Shape::Ordinary,
    ),
    (
        "%TypeError.prototype%",
        Some("%Error.prototype%"),
        Shape::Ordinary,
    ),
    (
        "%URIError.prototype%",
        Some("%Error.prototype%"),
        Shape::Ordinary,
    ),
    (
        "%AggregateError.prototype%",
        Some("%Error.prototype%"),
        Shape::Ordinary,
    ),
    (
        "%SuppressedError.prototype%",
        Some("%Error.prototype%"),
        Shape::Ordinary,
    ),
    ("%Iterator.prototype%", OP, Shape::Ordinary),
    ("%AsyncIteratorPrototype%", OP, Shape::Ordinary),
    ("%ArrayIteratorPrototype%", ITER, Shape::Ordinary),
    ("%MapIteratorPrototype%", ITER, Shape::Ordinary),
    ("%SetIteratorPrototype%", ITER, Shape::Ordinary),
    ("%StringIteratorPrototype%", ITER, Shape::Ordinary),
    ("%RegExpStringIteratorPrototype%", ITER, Shape::Ordinary),
    ("%IteratorHelperPrototype%", ITER, Shape::Ordinary),
    ("%WrapForValidIteratorPrototype%", ITER, Shape::Ordinary),
    ("%ForInIteratorPrototype%", ITER, Shape::Ordinary),
    (
        "%AsyncFromSyncIteratorPrototype%",
        Some("%AsyncIteratorPrototype%"),
        Shape::Ordinary,
    ),
    (
        "%GeneratorFunction.prototype%",
        Some("%Function.prototype%"),
        Shape::Ordinary,
    ),
    (
        "%AsyncGeneratorFunction.prototype%",
        Some("%Function.prototype%"),
        Shape::Ordinary,
    ),
    (
        "%AsyncFunction.prototype%",
        Some("%Function.prototype%"),
        Shape::Ordinary,
    ),
    ("%GeneratorPrototype%", ITER, Shape::Ordinary),
    (
        "%AsyncGeneratorPrototype%",
        Some("%AsyncIteratorPrototype%"),
        Shape::Ordinary,
    ),
    ("%TypedArray.prototype%", OP, Shape::Ordinary),
];

/// Constructors whose [[Prototype]] is not %Function.prototype%.
const CONSTRUCTOR_PROTOS: &[(&str, &str)] = &[
    ("EvalError", "%Error%"),
    ("RangeError", "%Error%"),
    ("ReferenceError", "%Error%"),
    ("SyntaxError", "%Error%"),
    ("TypeError", "%Error%"),
    ("URIError", "%Error%"),
    ("AggregateError", "%Error%"),
    ("SuppressedError", "%Error%"),
    ("GeneratorFunction", "%Function%"),
    ("AsyncGeneratorFunction", "%Function%"),
    ("AsyncFunction", "%Function%"),
];

const TYPED_ARRAYS: &[&str] = &[
    "Int8Array",
    "Uint8Array",
    "Uint8ClampedArray",
    "Int16Array",
    "Uint16Array",
    "Int32Array",
    "Uint32Array",
    "BigInt64Array",
    "BigUint64Array",
    "Float16Array",
    "Float32Array",
    "Float64Array",
];

/// Constructors that are intrinsics but not properties of the global object.
const NOT_GLOBAL: &[&str] = &[
    "TypedArray",
    "GeneratorFunction",
    "AsyncGeneratorFunction",
    "AsyncFunction",
];

/// Properties whose initial value is another built-in function (the same object).
const ALIASES: &[(&str, Key, &str)] = &[
    (
        "%Array.prototype%",
        Key::Symbol("iterator"),
        "%Array.prototype.values%",
    ),
    (
        "%TypedArray.prototype%",
        Key::Symbol("iterator"),
        "%TypedArray.prototype.values%",
    ),
    (
        "%Map.prototype%",
        Key::Symbol("iterator"),
        "%Map.prototype.entries%",
    ),
    (
        "%Set.prototype%",
        Key::Symbol("iterator"),
        "%Set.prototype.values%",
    ),
    (
        "%Set.prototype%",
        Key::Str("keys"),
        "%Set.prototype.values%",
    ),
    (
        "%String.prototype%",
        Key::Str("trimLeft"),
        "%String.prototype.trimStart%",
    ),
    (
        "%String.prototype%",
        Key::Str("trimRight"),
        "%String.prototype.trimEnd%",
    ),
    (
        "%Date.prototype%",
        Key::Str("toGMTString"),
        "%Date.prototype.toUTCString%",
    ),
    ("%Number%", Key::Str("parseFloat"), "%parseFloat%"),
    ("%Number%", Key::Str("parseInt"), "%parseInt%"),
];

/// Namespace objects installed on the global object.
const NAMESPACES: &[&str] = &["Math", "JSON", "Reflect", "Atomics"];

fn undefined_behaviour(_: &mut Agent, _: Value, _: &[Value], _: Option<Object>) -> JsResult<Value> {
    Ok(Value::Undefined)
}

/// %ThrowTypeError% (§10.2.4.1)
fn throw_type_error(
    agent: &mut Agent,
    _: Value,
    _: &[Value],
    _: Option<Object>,
) -> JsResult<Value> {
    Err(agent
        .throw_type_error("'caller', 'callee', and 'arguments' properties may not be accessed"))
}

fn symbol_key(agent: &Agent, name: &str) -> PropertyKey {
    let wk = &agent.symbols_wk;
    let s = match name {
        "asyncIterator" => wk.async_iterator,
        "hasInstance" => wk.has_instance,
        "isConcatSpreadable" => wk.is_concat_spreadable,
        "iterator" => wk.iterator,
        "match" => wk.r#match,
        "matchAll" => wk.match_all,
        "replace" => wk.replace,
        "search" => wk.search,
        "species" => wk.species,
        "split" => wk.split,
        "toPrimitive" => wk.to_primitive,
        "toStringTag" => wk.to_string_tag,
        "unscopables" => wk.unscopables,
        "dispose" => wk.dispose,
        "asyncDispose" => wk.async_dispose,
        other => unreachable!("unknown well-known symbol {other}"),
    };
    PropertyKey::Symbol(s)
}

fn key_of(agent: &mut Agent, key: Key) -> PropertyKey {
    match key {
        Key::Str(s) => agent.key(s),
        Key::Symbol(s) => symbol_key(agent, s),
    }
}

fn define(agent: &mut Agent, o: Object, key: PropertyKey, value: Value, w: bool, e: bool, c: bool) {
    agent.obj_mut(o).properties.insert(
        key,
        Property::Data {
            value,
            writable: w,
            enumerable: e,
            configurable: c,
        },
    );
}

/// The intrinsic named `name`, created as an ordinary object if it does not exist yet.
fn intrinsic_or_create(agent: &mut Agent, realm: RealmId, name: &str) -> Object {
    if let Some(&o) = agent.realm(realm).intrinsics.get(name) {
        return o;
    }
    let proto = agent.intrinsic_in(realm, "%Object.prototype%");
    let o = agent.ordinary_object_create(Some(proto));
    agent.set_intrinsic(realm, name, o);
    o
}

/// CreateIntrinsics + SetDefaultGlobalBindings for `realm`. Pass this to
/// `Agent::set_intrinsics_installer`.
pub fn install(agent: &mut Agent, realm: RealmId) -> JsResult<()> {
    // The objects with special shapes, in order.
    for &(name, proto, shape) in OBJECTS {
        let proto = proto.map(|p| agent.intrinsic_in(realm, p));
        let o = match shape {
            Shape::Ordinary => agent.ordinary_object_create(proto),
            Shape::ImmutablePrototype => {
                agent.object_with_kind(proto, ObjectKind::ImmutablePrototype)
            }
            Shape::Function => {
                let empty = PropertyKey::String(agent.names.empty);
                create_builtin_function(
                    agent,
                    undefined_behaviour,
                    0,
                    empty,
                    Vec::new(),
                    Some(realm),
                    proto,
                    None,
                    false,
                )
            }
            Shape::Array => {
                let a = agent.object_with_kind(proto, ObjectKind::Array);
                let len = PropertyKey::String(agent.names.length);
                define(agent, a, len, Value::Number(0.0), true, false, false);
                a
            }
            Shape::Boolean => agent.object_with_kind(proto, ObjectKind::BooleanObject(false)),
            Shape::Number => agent.object_with_kind(proto, ObjectKind::NumberObject(0.0)),
            Shape::String => {
                let empty = agent.names.empty;
                agent.object_with_kind(proto, ObjectKind::StringObject(empty))
            }
        };
        agent.set_intrinsic(realm, name, o);
    }
    for ta in TYPED_ARRAYS {
        let proto = agent.intrinsic_in(realm, "%TypedArray.prototype%");
        let o = agent.ordinary_object_create(Some(proto));
        agent.set_intrinsic(realm, &format!("%{ta}.prototype%"), o);
    }

    // Constructors first (other functions' owners may be constructors).
    let function_proto = agent.intrinsic_in(realm, "%Function.prototype%");
    let mut constructors: Vec<&'static str> = Vec::new();
    for pass in 0..2 {
        for entry in FUNCTIONS.iter().filter(|e| e.kind == Kind::Constructor) {
            let Key::Str(name) = entry.key else { continue };
            let proto_name = CONSTRUCTOR_PROTOS
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, p)| *p);
            let proto_name = proto_name.or(TYPED_ARRAYS.contains(&name).then_some("%TypedArray%"));
            // Pass 0 creates constructors inheriting from %Function.prototype%; pass 1 the rest.
            if (pass == 0) != proto_name.is_none() {
                continue;
            }
            let proto = match proto_name {
                Some(p) => agent.intrinsic_in(realm, p),
                None => function_proto,
            };
            let key = agent.key(name);
            let f = create_builtin_function(
                agent,
                entry.behaviour,
                entry.length,
                key,
                Vec::new(),
                Some(realm),
                Some(proto),
                None,
                true,
            );
            agent.set_intrinsic(realm, &format!("%{name}%"), f);
            constructors.push(name);
        }
    }

    // Every other function property.
    for entry in FUNCTIONS.iter().filter(|e| e.kind != Kind::Constructor) {
        let owner = if entry.owner == "%GlobalObject%" {
            agent.realm(realm).global_object
        } else {
            intrinsic_or_create(agent, realm, entry.owner)
        };
        let key = key_of(agent, entry.key);
        let prefix = match entry.kind {
            Kind::Getter => Some("get"),
            Kind::Setter => Some("set"),
            _ => None,
        };
        let f = create_builtin_function(
            agent,
            entry.behaviour,
            entry.length,
            key,
            Vec::new(),
            Some(realm),
            Some(function_proto),
            prefix,
            false,
        );
        match entry.kind {
            Kind::Method => {
                define(
                    agent,
                    owner,
                    key,
                    Value::Object(f),
                    entry.writable,
                    entry.enumerable,
                    entry.configurable,
                );
                if let Key::Str(k) = entry.key {
                    let name = if entry.owner == "%GlobalObject%" {
                        format!("%{k}%")
                    } else {
                        format!("%{}.{k}%", entry.owner.trim_matches('%'))
                    };
                    agent.set_intrinsic(realm, &name, f);
                }
            }
            Kind::Getter | Kind::Setter => {
                let existing = agent.obj(owner).properties.get(&key).copied();
                let (mut get, mut set) = match existing {
                    Some(Property::Accessor { get, set, .. }) => (get, set),
                    _ => (Value::Undefined, Value::Undefined),
                };
                if entry.kind == Kind::Getter {
                    get = Value::Object(f);
                } else {
                    set = Value::Object(f);
                }
                agent.obj_mut(owner).properties.insert(
                    key,
                    Property::Accessor {
                        get,
                        set,
                        enumerable: entry.enumerable,
                        configurable: entry.configurable,
                    },
                );
            }
            Kind::Constructor => unreachable!(),
        }
    }

    // constructor.prototype and prototype.constructor links.
    for name in &constructors {
        let ctor = agent.intrinsic_in(realm, &format!("%{name}%"));
        let proto_name = format!("%{name}.prototype%");
        let Some(&proto) = agent.realm(realm).intrinsics.get(proto_name.as_str()) else {
            continue;
        };
        let pk = PropertyKey::String(agent.names.prototype);
        define(agent, ctor, pk, Value::Object(proto), false, false, false);
        let ck = PropertyKey::String(agent.names.constructor);
        define(agent, proto, ck, Value::Object(ctor), true, false, true);
    }

    // Aliased function properties.
    for &(owner, key, target) in ALIASES {
        let (Some(&o), Some(&f)) = (
            agent.realm(realm).intrinsics.get(owner),
            agent.realm(realm).intrinsics.get(target),
        ) else {
            continue;
        };
        let key = key_of(agent, key);
        define(agent, o, key, Value::Object(f), true, false, true);
    }

    // %ThrowTypeError% (§10.2.4.1): frozen, with non-configurable length and name.
    let empty = PropertyKey::String(agent.names.empty);
    let tte = create_builtin_function(
        agent,
        throw_type_error,
        0,
        empty,
        Vec::new(),
        Some(realm),
        Some(function_proto),
        None,
        false,
    );
    for k in [agent.names.length, agent.names.name] {
        if let Some(Property::Data { configurable, .. }) = agent
            .obj_mut(tte)
            .properties
            .get_mut(&PropertyKey::String(k))
        {
            *configurable = false;
        }
    }
    agent.obj_mut(tte).extensible = false;
    agent.set_intrinsic(realm, "%ThrowTypeError%", tte);

    // Well-known symbols as properties of %Symbol% (§20.4.2).
    if let Some(&symbol) = agent.realm(realm).intrinsics.get("%Symbol%") {
        for name in [
            "asyncIterator",
            "hasInstance",
            "isConcatSpreadable",
            "iterator",
            "match",
            "matchAll",
            "replace",
            "search",
            "species",
            "split",
            "toPrimitive",
            "toStringTag",
            "unscopables",
            "dispose",
            "asyncDispose",
        ] {
            let PropertyKey::Symbol(s) = symbol_key(agent, name) else {
                unreachable!()
            };
            let k = agent.key(name);
            define(agent, symbol, k, Value::Symbol(s), false, false, false);
        }
    }

    // The global object (SetDefaultGlobalBindings).
    let global = agent.realm(realm).global_object;
    let object_proto = agent.intrinsic_in(realm, "%Object.prototype%");
    agent.obj_mut(global).prototype = Some(object_proto);
    agent.set_intrinsic(realm, "%GlobalObject%", global);
    for name in &constructors {
        if NOT_GLOBAL.contains(name) {
            continue;
        }
        let ctor = agent.intrinsic_in(realm, &format!("%{name}%"));
        let k = agent.key(name);
        define(agent, global, k, Value::Object(ctor), true, false, true);
    }
    for name in NAMESPACES {
        let o = intrinsic_or_create(agent, realm, &format!("%{name}%"));
        let k = agent.key(name);
        define(agent, global, k, Value::Object(o), true, false, true);
    }
    let k = agent.key("globalThis");
    define(agent, global, k, Value::Object(global), true, false, true);
    for (name, value) in [
        ("Infinity", Value::Number(f64::INFINITY)),
        ("NaN", Value::Number(f64::NAN)),
        ("undefined", Value::Undefined),
    ] {
        let k = agent.key(name);
        define(agent, global, k, value, false, false, false);
    }
    Ok(())
}
