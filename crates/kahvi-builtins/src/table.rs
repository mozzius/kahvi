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
    ("%IntlSegmentIteratorPrototype%", ITER, Shape::Ordinary),
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
        "%TypedArray.prototype%",
        Key::Str("toString"),
        "%Array.prototype.toString%",
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
    (
        "%DisposableStack.prototype%",
        Key::Symbol("dispose"),
        "%DisposableStack.prototype.dispose%",
    ),
    (
        "%AsyncDisposableStack.prototype%",
        Key::Symbol("asyncDispose"),
        "%AsyncDisposableStack.prototype.disposeAsync%",
    ),
    ("%Number%", Key::Str("parseFloat"), "%parseFloat%"),
    ("%Number%", Key::Str("parseInt"), "%parseInt%"),
];

/// A data property whose value is not a function.
#[derive(Clone, Copy)]
enum Val {
    Str(&'static str),
    Num(f64),
    /// Another intrinsic object.
    Intrinsic(&'static str),
}

/// Value properties: (owner, key, value, writable, enumerable, configurable).
const VALUES: &[(&str, Key, Val, bool, bool, bool)] = &[
    (
        "%Error.prototype%",
        Key::Str("name"),
        Val::Str("Error"),
        true,
        false,
        true,
    ),
    (
        "%Error.prototype%",
        Key::Str("message"),
        Val::Str(""),
        true,
        false,
        true,
    ),
    (
        "%EvalError.prototype%",
        Key::Str("name"),
        Val::Str("EvalError"),
        true,
        false,
        true,
    ),
    (
        "%EvalError.prototype%",
        Key::Str("message"),
        Val::Str(""),
        true,
        false,
        true,
    ),
    (
        "%RangeError.prototype%",
        Key::Str("name"),
        Val::Str("RangeError"),
        true,
        false,
        true,
    ),
    (
        "%RangeError.prototype%",
        Key::Str("message"),
        Val::Str(""),
        true,
        false,
        true,
    ),
    (
        "%ReferenceError.prototype%",
        Key::Str("name"),
        Val::Str("ReferenceError"),
        true,
        false,
        true,
    ),
    (
        "%ReferenceError.prototype%",
        Key::Str("message"),
        Val::Str(""),
        true,
        false,
        true,
    ),
    (
        "%SyntaxError.prototype%",
        Key::Str("name"),
        Val::Str("SyntaxError"),
        true,
        false,
        true,
    ),
    (
        "%SyntaxError.prototype%",
        Key::Str("message"),
        Val::Str(""),
        true,
        false,
        true,
    ),
    (
        "%TypeError.prototype%",
        Key::Str("name"),
        Val::Str("TypeError"),
        true,
        false,
        true,
    ),
    (
        "%TypeError.prototype%",
        Key::Str("message"),
        Val::Str(""),
        true,
        false,
        true,
    ),
    (
        "%URIError.prototype%",
        Key::Str("name"),
        Val::Str("URIError"),
        true,
        false,
        true,
    ),
    (
        "%URIError.prototype%",
        Key::Str("message"),
        Val::Str(""),
        true,
        false,
        true,
    ),
    (
        "%AggregateError.prototype%",
        Key::Str("name"),
        Val::Str("AggregateError"),
        true,
        false,
        true,
    ),
    (
        "%AggregateError.prototype%",
        Key::Str("message"),
        Val::Str(""),
        true,
        false,
        true,
    ),
    (
        "%SuppressedError.prototype%",
        Key::Str("name"),
        Val::Str("SuppressedError"),
        true,
        false,
        true,
    ),
    (
        "%SuppressedError.prototype%",
        Key::Str("message"),
        Val::Str(""),
        true,
        false,
        true,
    ),
    (
        "%Number%",
        Key::Str("EPSILON"),
        Val::Num(f64::EPSILON),
        false,
        false,
        false,
    ),
    (
        "%Number%",
        Key::Str("MAX_SAFE_INTEGER"),
        Val::Num(9007199254740991.0),
        false,
        false,
        false,
    ),
    (
        "%Number%",
        Key::Str("MAX_VALUE"),
        Val::Num(f64::MAX),
        false,
        false,
        false,
    ),
    (
        "%Number%",
        Key::Str("MIN_SAFE_INTEGER"),
        Val::Num(-9007199254740991.0),
        false,
        false,
        false,
    ),
    (
        "%Number%",
        Key::Str("MIN_VALUE"),
        Val::Num(5e-324),
        false,
        false,
        false,
    ),
    (
        "%Number%",
        Key::Str("NaN"),
        Val::Num(f64::NAN),
        false,
        false,
        false,
    ),
    (
        "%Number%",
        Key::Str("NEGATIVE_INFINITY"),
        Val::Num(f64::NEG_INFINITY),
        false,
        false,
        false,
    ),
    (
        "%Number%",
        Key::Str("POSITIVE_INFINITY"),
        Val::Num(f64::INFINITY),
        false,
        false,
        false,
    ),
    (
        "%Math%",
        Key::Str("E"),
        Val::Num(std::f64::consts::E),
        false,
        false,
        false,
    ),
    (
        "%Math%",
        Key::Str("LN10"),
        Val::Num(std::f64::consts::LN_10),
        false,
        false,
        false,
    ),
    (
        "%Math%",
        Key::Str("LN2"),
        Val::Num(std::f64::consts::LN_2),
        false,
        false,
        false,
    ),
    (
        "%Math%",
        Key::Str("LOG10E"),
        Val::Num(std::f64::consts::LOG10_E),
        false,
        false,
        false,
    ),
    (
        "%Math%",
        Key::Str("LOG2E"),
        Val::Num(std::f64::consts::LOG2_E),
        false,
        false,
        false,
    ),
    (
        "%Math%",
        Key::Str("PI"),
        Val::Num(std::f64::consts::PI),
        false,
        false,
        false,
    ),
    (
        "%Math%",
        Key::Str("SQRT1_2"),
        Val::Num(std::f64::consts::FRAC_1_SQRT_2),
        false,
        false,
        false,
    ),
    (
        "%Math%",
        Key::Str("SQRT2"),
        Val::Num(std::f64::consts::SQRT_2),
        false,
        false,
        false,
    ),
    // GeneratorFunction.prototype.prototype and friends (§27.3.3, §27.4.3).
    (
        "%GeneratorFunction.prototype%",
        Key::Str("prototype"),
        Val::Intrinsic("%GeneratorPrototype%"),
        false,
        false,
        true,
    ),
    (
        "%GeneratorPrototype%",
        Key::Str("constructor"),
        Val::Intrinsic("%GeneratorFunction.prototype%"),
        false,
        false,
        true,
    ),
    (
        "%AsyncGeneratorFunction.prototype%",
        Key::Str("prototype"),
        Val::Intrinsic("%AsyncGeneratorPrototype%"),
        false,
        false,
        true,
    ),
    (
        "%AsyncGeneratorPrototype%",
        Key::Str("constructor"),
        Val::Intrinsic("%AsyncGeneratorFunction.prototype%"),
        false,
        false,
        true,
    ),
    (
        "%GeneratorFunction%",
        Key::Str("prototype"),
        Val::Intrinsic("%GeneratorFunction.prototype%"),
        false,
        false,
        false,
    ),
    (
        "%GeneratorFunction.prototype%",
        Key::Str("constructor"),
        Val::Intrinsic("%GeneratorFunction%"),
        false,
        false,
        true,
    ),
    (
        "%AsyncGeneratorFunction%",
        Key::Str("prototype"),
        Val::Intrinsic("%AsyncGeneratorFunction.prototype%"),
        false,
        false,
        false,
    ),
    (
        "%AsyncGeneratorFunction.prototype%",
        Key::Str("constructor"),
        Val::Intrinsic("%AsyncGeneratorFunction%"),
        false,
        false,
        true,
    ),
    (
        "%AsyncFunction%",
        Key::Str("prototype"),
        Val::Intrinsic("%AsyncFunction.prototype%"),
        false,
        false,
        false,
    ),
    (
        "%AsyncFunction.prototype%",
        Key::Str("constructor"),
        Val::Intrinsic("%AsyncFunction%"),
        false,
        false,
        true,
    ),
];

/// [Symbol.toStringTag] values: { [[Writable]]: false, [[Enumerable]]: false, [[Configurable]]: true }.
const TO_STRING_TAGS: &[(&str, &str)] = &[
    ("%Math%", "Math"),
    ("%JSON%", "JSON"),
    ("%Reflect%", "Reflect"),
    ("%Atomics%", "Atomics"),
    ("%Symbol.prototype%", "Symbol"),
    ("%BigInt.prototype%", "BigInt"),
    ("%Map.prototype%", "Map"),
    ("%Set.prototype%", "Set"),
    ("%WeakMap.prototype%", "WeakMap"),
    ("%WeakSet.prototype%", "WeakSet"),
    ("%WeakRef.prototype%", "WeakRef"),
    ("%FinalizationRegistry.prototype%", "FinalizationRegistry"),
    ("%Promise.prototype%", "Promise"),
    ("%ArrayBuffer.prototype%", "ArrayBuffer"),
    ("%SharedArrayBuffer.prototype%", "SharedArrayBuffer"),
    ("%DataView.prototype%", "DataView"),
    ("%ArrayIteratorPrototype%", "Array Iterator"),
    ("%MapIteratorPrototype%", "Map Iterator"),
    ("%SetIteratorPrototype%", "Set Iterator"),
    ("%StringIteratorPrototype%", "String Iterator"),
    ("%RegExpStringIteratorPrototype%", "RegExp String Iterator"),
    ("%IteratorHelperPrototype%", "Iterator Helper"),
    ("%GeneratorPrototype%", "Generator"),
    ("%AsyncGeneratorPrototype%", "AsyncGenerator"),
    ("%GeneratorFunction.prototype%", "GeneratorFunction"),
    (
        "%AsyncGeneratorFunction.prototype%",
        "AsyncGeneratorFunction",
    ),
    ("%AsyncFunction.prototype%", "AsyncFunction"),
    ("%DisposableStack.prototype%", "DisposableStack"),
    ("%AsyncDisposableStack.prototype%", "AsyncDisposableStack"),
    // ECMA-402.
    ("%Intl%", "Intl"),
    ("%Intl.Collator.prototype%", "Intl.Collator"),
    ("%Intl.DateTimeFormat.prototype%", "Intl.DateTimeFormat"),
    ("%Intl.DisplayNames.prototype%", "Intl.DisplayNames"),
    ("%Intl.DurationFormat.prototype%", "Intl.DurationFormat"),
    ("%Intl.ListFormat.prototype%", "Intl.ListFormat"),
    ("%Intl.Locale.prototype%", "Intl.Locale"),
    ("%Intl.NumberFormat.prototype%", "Intl.NumberFormat"),
    ("%Intl.PluralRules.prototype%", "Intl.PluralRules"),
    (
        "%Intl.RelativeTimeFormat.prototype%",
        "Intl.RelativeTimeFormat",
    ),
    ("%Intl.Segmenter.prototype%", "Intl.Segmenter"),
    (
        "%IntlSegmentIteratorPrototype%",
        "Segmenter String Iterator",
    ),
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

/// AddRestrictedFunctionProperties(func, realm) (§10.2.4), with `thrower`
/// the realm's %ThrowTypeError%.
fn add_restricted_function_properties(agent: &mut Agent, func: Object, thrower: Object) {
    // 3-4. Define "caller" and "arguments" as accessors { [[Get]]: thrower, [[Set]]: thrower,
    //      [[Enumerable]]: false, [[Configurable]]: true }.
    for name in ["caller", "arguments"] {
        let key = agent.key(name);
        agent.obj_mut(func).properties.insert(
            key,
            Property::Accessor {
                get: Some(thrower),
                set: Some(thrower),
                enumerable: false,
                configurable: true,
            },
        );
    }
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
/// Whether a built-in belongs to ECMA-402 (`%Intl%`, its constructors'
/// objects and the Segmenter iterator prototypes). Without `agent.intl`
/// none of them is created, which keeps realm creation as cheap as before.
fn is_intl(owner: &str) -> bool {
    owner.starts_with("%Intl")
}

pub fn install(agent: &mut Agent, realm: RealmId) -> JsResult<()> {
    agent.regexp_matcher_check = Some(crate::support::regexp::has_regexp_matcher);
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
            // StringCreate("", %Object.prototype%): a String exotic object whose
            // "length" is { [[Value]]: +0, [[Writable]]: false, [[Enumerable]]: false, [[Configurable]]: false }.
            Shape::String => {
                let empty = agent.names.empty;
                let s = agent.object_with_kind(proto, ObjectKind::StringObject(empty));
                let len = PropertyKey::String(agent.names.length);
                define(agent, s, len, Value::Number(0.0), false, false, false);
                s
            }
        };
        agent.set_intrinsic(realm, name, o);
    }
    for ta in TYPED_ARRAYS {
        let proto = agent.intrinsic_in(realm, "%TypedArray.prototype%");
        let o = agent.ordinary_object_create(Some(proto));
        agent.set_intrinsic(realm, &format!("%{ta}.prototype%"), o);
    }

    let intl = agent.intl;
    // Constructors first (other functions' owners may be constructors).
    let function_proto = agent.intrinsic_in(realm, "%Function.prototype%");
    // By intrinsic name without the %s: "Array", "Intl.Collator".
    let mut constructors: Vec<String> = Vec::new();
    for pass in 0..2 {
        for entry in FUNCTIONS
            .iter()
            .filter(|e| e.kind == Kind::Constructor && (intl || !is_intl(e.owner)))
        {
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
            // ECMA-402's constructors are properties of %Intl%.
            let full = match entry.owner {
                "%Intl%" => format!("Intl.{name}"),
                _ => name.to_string(),
            };
            agent.set_intrinsic(realm, &format!("%{full}%"), f);
            constructors.push(full);
        }
    }

    // Every other function property.
    let legacy_regexp = crate::support::regexp_legacy::FUNCTIONS;
    for entry in FUNCTIONS
        .iter()
        .chain(legacy_regexp)
        .filter(|e| e.kind != Kind::Constructor && (intl || !is_intl(e.owner)))
    {
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
                    _ => (None, None),
                };
                if entry.kind == Kind::Getter {
                    get = Some(f);
                } else {
                    set = Some(f);
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
        // A prototype whose constructor is its own entry (Iterator.prototype's
        // accessor, §27.1.4.1) keeps it.
        if agent.obj(proto).properties.contains(&ck) {
            continue;
        }
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

    // Value properties.
    for &(owner, key, value, w, e, c) in VALUES {
        let Some(&o) = agent.realm(realm).intrinsics.get(owner) else {
            continue;
        };
        let value = match value {
            Val::Str(s) => Value::String(agent.string(s)),
            Val::Num(n) => Value::Number(n),
            Val::Intrinsic(name) => match agent.realm(realm).intrinsics.get(name) {
                Some(&v) => Value::Object(v),
                None => continue,
            },
        };
        let key = key_of(agent, key);
        define(agent, o, key, value, w, e, c);
    }
    // Array.prototype [ %Symbol.unscopables% ] (§23.1.3.41).
    if let Some(&array_proto) = agent.realm(realm).intrinsics.get("%Array.prototype%") {
        // 1. Let unscopableList be OrdinaryObjectCreate(null).
        let unscopable_list = agent.object_with_kind(None, ObjectKind::Ordinary);
        // 2-17. Perform ! CreateDataPropertyOrThrow(unscopableList, name, true) for each name.
        for name in [
            "at",
            "copyWithin",
            "entries",
            "fill",
            "find",
            "findIndex",
            "findLast",
            "findLastIndex",
            "flat",
            "flatMap",
            "includes",
            "keys",
            "toReversed",
            "toSorted",
            "toSpliced",
            "values",
        ] {
            let k = agent.key(name);
            define(
                agent,
                unscopable_list,
                k,
                Value::Boolean(true),
                true,
                true,
                true,
            );
        }
        // 18. Return unscopableList.
        // { [[Writable]]: false, [[Enumerable]]: false, [[Configurable]]: true }
        let k = PropertyKey::Symbol(agent.symbols_wk.unscopables);
        define(
            agent,
            array_proto,
            k,
            Value::Object(unscopable_list),
            false,
            false,
            true,
        );
    }

    // BYTES_PER_ELEMENT of each TypedArray constructor and prototype (§23.2.6.1, §23.2.7.1).
    for kind in kahvi_core::object::typed_array::ALL_ELEMENT_TYPES {
        let size = Value::Number(kind.size() as f64);
        for name in [
            format!("%{}%", kind.name()),
            format!("%{}.prototype%", kind.name()),
        ] {
            if let Some(&o) = agent.realm(realm).intrinsics.get(name.as_str()) {
                let key = agent.key("BYTES_PER_ELEMENT");
                define(agent, o, key, size, false, false, false);
            }
        }
    }
    for &(owner, tag) in TO_STRING_TAGS {
        let Some(&o) = agent.realm(realm).intrinsics.get(owner) else {
            continue;
        };
        let key = symbol_key(agent, "toStringTag");
        let value = Value::String(agent.string(tag));
        define(agent, o, key, value, false, false, true);
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

    // CreateIntrinsics: Perform AddRestrictedFunctionProperties(%Function.prototype%, realmRec).
    add_restricted_function_properties(agent, function_proto, tte);

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
        if NOT_GLOBAL.contains(&name.as_str()) {
            continue;
        }
        let ctor = agent.intrinsic_in(realm, &format!("%{name}%"));
        let (owner, key) = match name.rsplit_once('.') {
            Some((owner, key)) => (
                intrinsic_or_create(agent, realm, &format!("%{owner}%")),
                key,
            ),
            None => (global, name.as_str()),
        };
        let k = agent.key(key);
        define(agent, owner, k, Value::Object(ctor), true, false, true);
    }
    // Intl is only a global property while the agent asks for it
    // (`Agent::intl`): its implementation is incomplete (docs/INTL.md).
    let intl: &[&str] = if agent.intl { &["Intl"] } else { &[] };
    for name in NAMESPACES.iter().chain(intl) {
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
    if agent.intl {
        crate::support::intl::install(agent, realm);
    }
    crate::extensions::install(agent, realm);
    Ok(())
}
