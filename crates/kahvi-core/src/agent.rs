//! The agent (ECMA-262 §9.7): owns the heap, realms, the execution context
//! stack and the job queue. Every engine operation takes `&mut Agent`.

use std::collections::VecDeque;

use crate::env::{EnvId, Environment};
use crate::error::{JsError, JsResult};
use crate::object::{ObjectData, ObjectKind, PropertyKey};
use crate::realm::{Realm, RealmId};
use crate::value::{JsString, Object, Symbol, Value};

/// Maximum nesting of calls before a RangeError (the Rust stack is finite).
pub const MAX_CALL_DEPTH: usize = 2_000;

#[derive(Debug)]
pub struct SymbolData {
    /// [[Description]]
    pub description: Option<JsString>,
}

/// An execution context (§9.4): which function and realm are running.
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub function: Option<Object>,
    pub realm: RealmId,
    pub script_or_module: Option<crate::ScriptOrModule>,
}

/// A pending job (§9.5): a closure run when the stack is empty.
pub type Job = Box<dyn FnOnce(&mut Agent) -> JsResult<()> + Send>;

/// The well-known symbols (§6.1.5.1).
#[derive(Debug, Clone, Copy)]
pub struct WellKnownSymbols {
    pub async_iterator: Symbol,
    pub has_instance: Symbol,
    pub is_concat_spreadable: Symbol,
    pub iterator: Symbol,
    pub r#match: Symbol,
    pub match_all: Symbol,
    pub replace: Symbol,
    pub search: Symbol,
    pub species: Symbol,
    pub split: Symbol,
    pub to_primitive: Symbol,
    pub to_string_tag: Symbol,
    pub unscopables: Symbol,
    pub dispose: Symbol,
    pub async_dispose: Symbol,
}

/// Frequently used property names, interned once.
#[derive(Debug, Clone, Copy)]
pub struct Names {
    pub length: JsString,
    pub prototype: JsString,
    pub constructor: JsString,
    pub name: JsString,
    pub message: JsString,
    pub cause: JsString,
    pub value: JsString,
    pub writable: JsString,
    pub enumerable: JsString,
    pub configurable: JsString,
    pub get: JsString,
    pub set: JsString,
    pub done: JsString,
    pub next: JsString,
    pub r#return: JsString,
    pub throw: JsString,
    pub then: JsString,
    pub caller: JsString,
    pub arguments: JsString,
    pub callee: JsString,
    pub to_string: JsString,
    pub value_of: JsString,
    pub empty: JsString,
    pub default: JsString,
    pub undefined: JsString,
}

/// Installs the built-in objects into a fresh realm. Provided by the
/// `kahvi-builtins` crate when the runtime is created.
pub type IntrinsicsInstaller = fn(&mut Agent, RealmId) -> JsResult<()>;

pub struct Agent {
    objects: Vec<ObjectData>,
    strings: Vec<Box<[u16]>>,
    string_index: crate::hash::ContentHashMap<Box<[u16]>, u32>,
    symbols: Vec<SymbolData>,
    pub(crate) envs: Vec<Environment>,
    pub realms: Vec<Realm>,
    pub contexts: Vec<ExecutionContext>,
    pub jobs: VecDeque<Job>,
    /// [[GlobalSymbolRegistry]]: Symbol.for keys.
    pub symbol_registry: Vec<(JsString, Symbol)>,
    pub symbols_wk: WellKnownSymbols,
    pub names: Names,
    pub call_depth: usize,
    pub(crate) installer: Option<IntrinsicsInstaller>,
    /// Unhandled promise rejections the host may report.
    pub rejection_tracker: Vec<(Object, bool)>,
    /// Scripts and modules loaded so far.
    pub scripts: Vec<crate::vm::ScriptRecord>,
    next_private_name: u32,
    pub(crate) bigints: Vec<crate::ops::bigint::BigIntValue>,
    /// Set by the RegExp builtin: whether an object has [[RegExpMatcher]].
    pub regexp_matcher_check: Option<crate::ops::RegExpMatcherCheck>,
    /// Host hook: the current time in milliseconds since the epoch (Date.now).
    /// The engine never reads the system clock itself.
    pub host_now: Option<fn() -> f64>,
    /// Host hook: LocalTZA(t, isUTC) (§21.4.1.25), the local time zone
    /// offset in milliseconds. Without it, local time is UTC.
    pub host_local_tza: Option<fn(f64, bool) -> f64>,
}

impl std::fmt::Debug for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Agent")
            .field("objects", &self.objects.len())
            .finish()
    }
}

impl Agent {
    pub fn new() -> Agent {
        let placeholder = JsString(0);
        let mut agent = Agent {
            objects: Vec::new(),
            strings: Vec::new(),
            string_index: Default::default(),
            symbols: Vec::new(),
            envs: Vec::new(),
            realms: Vec::new(),
            contexts: Vec::new(),
            jobs: VecDeque::new(),
            symbol_registry: Vec::new(),
            symbols_wk: WellKnownSymbols {
                async_iterator: Symbol(0),
                has_instance: Symbol(0),
                is_concat_spreadable: Symbol(0),
                iterator: Symbol(0),
                r#match: Symbol(0),
                match_all: Symbol(0),
                replace: Symbol(0),
                search: Symbol(0),
                species: Symbol(0),
                split: Symbol(0),
                to_primitive: Symbol(0),
                to_string_tag: Symbol(0),
                unscopables: Symbol(0),
                dispose: Symbol(0),
                async_dispose: Symbol(0),
            },
            names: Names {
                length: placeholder,
                prototype: placeholder,
                constructor: placeholder,
                name: placeholder,
                message: placeholder,
                cause: placeholder,
                value: placeholder,
                writable: placeholder,
                enumerable: placeholder,
                configurable: placeholder,
                get: placeholder,
                set: placeholder,
                done: placeholder,
                next: placeholder,
                r#return: placeholder,
                throw: placeholder,
                then: placeholder,
                caller: placeholder,
                arguments: placeholder,
                callee: placeholder,
                to_string: placeholder,
                value_of: placeholder,
                empty: placeholder,
                default: placeholder,
                undefined: placeholder,
            },
            call_depth: 0,
            installer: None,
            rejection_tracker: Vec::new(),
            scripts: Vec::new(),
            next_private_name: 0,
            bigints: Vec::new(),
            regexp_matcher_check: None,
            host_now: None,
            host_local_tza: None,
        };
        agent.names = Names {
            length: agent.string("length"),
            prototype: agent.string("prototype"),
            constructor: agent.string("constructor"),
            name: agent.string("name"),
            message: agent.string("message"),
            cause: agent.string("cause"),
            value: agent.string("value"),
            writable: agent.string("writable"),
            enumerable: agent.string("enumerable"),
            configurable: agent.string("configurable"),
            get: agent.string("get"),
            set: agent.string("set"),
            done: agent.string("done"),
            next: agent.string("next"),
            r#return: agent.string("return"),
            throw: agent.string("throw"),
            then: agent.string("then"),
            caller: agent.string("caller"),
            arguments: agent.string("arguments"),
            callee: agent.string("callee"),
            to_string: agent.string("toString"),
            value_of: agent.string("valueOf"),
            empty: agent.string(""),
            default: agent.string("default"),
            undefined: agent.string("undefined"),
        };
        fn wk(agent: &mut Agent, name: &str) -> Symbol {
            let d = agent.string(name);
            agent.new_symbol(Some(d))
        }
        agent.symbols_wk = WellKnownSymbols {
            async_iterator: wk(&mut agent, "Symbol.asyncIterator"),
            has_instance: wk(&mut agent, "Symbol.hasInstance"),
            is_concat_spreadable: wk(&mut agent, "Symbol.isConcatSpreadable"),
            iterator: wk(&mut agent, "Symbol.iterator"),
            r#match: wk(&mut agent, "Symbol.match"),
            match_all: wk(&mut agent, "Symbol.matchAll"),
            replace: wk(&mut agent, "Symbol.replace"),
            search: wk(&mut agent, "Symbol.search"),
            species: wk(&mut agent, "Symbol.species"),
            split: wk(&mut agent, "Symbol.split"),
            to_primitive: wk(&mut agent, "Symbol.toPrimitive"),
            to_string_tag: wk(&mut agent, "Symbol.toStringTag"),
            unscopables: wk(&mut agent, "Symbol.unscopables"),
            dispose: wk(&mut agent, "Symbol.dispose"),
            async_dispose: wk(&mut agent, "Symbol.asyncDispose"),
        };
        agent
    }

    /// Sets the function that installs built-ins into new realms.
    pub fn set_intrinsics_installer(&mut self, f: IntrinsicsInstaller) {
        self.installer = Some(f);
    }

    // ---- objects -------------------------------------------------------

    pub fn alloc(&mut self, data: ObjectData) -> Object {
        self.objects.push(data);
        Object((self.objects.len() - 1) as u32)
    }

    pub fn obj(&self, o: Object) -> &ObjectData {
        &self.objects[o.0 as usize]
    }

    pub fn obj_mut(&mut self, o: Object) -> &mut ObjectData {
        &mut self.objects[o.0 as usize]
    }

    /// OrdinaryObjectCreate(proto) with no additional internal slots.
    pub fn ordinary_object_create(&mut self, proto: Option<Object>) -> Object {
        self.alloc(ObjectData::new(proto, ObjectKind::Ordinary))
    }

    /// OrdinaryObjectCreate(proto) for an object of the given kind.
    pub fn object_with_kind(&mut self, proto: Option<Object>, kind: ObjectKind) -> Object {
        self.alloc(ObjectData::new(proto, kind))
    }

    /// The internal-slot data of a built-in object created with
    /// `ObjectKind::Native`, if it has that type.
    pub fn native<T: 'static>(&self, o: Object) -> Option<&T> {
        match &self.obj(o).kind {
            ObjectKind::Native(d) => d.as_any().downcast_ref::<T>(),
            _ => None,
        }
    }

    pub fn native_mut<T: 'static>(&mut self, o: Object) -> Option<&mut T> {
        match &mut self.obj_mut(o).kind {
            ObjectKind::Native(d) => d.as_any_mut().downcast_mut::<T>(),
            _ => None,
        }
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    // ---- strings ---------------------------------------------------------

    /// Interns UTF-16 code units.
    pub fn intern(&mut self, units: &[u16]) -> JsString {
        if let Some(&id) = self.string_index.get(units) {
            return JsString(id);
        }
        let id = self.strings.len() as u32;
        self.strings.push(units.into());
        self.string_index.insert(units.into(), id);
        JsString(id)
    }

    /// A String value from Rust text.
    pub fn string(&mut self, s: &str) -> JsString {
        let units: Vec<u16> = s.encode_utf16().collect();
        self.intern(&units)
    }

    /// The code units of a String value.
    pub fn str(&self, s: JsString) -> &[u16] {
        &self.strings[s.0 as usize]
    }

    /// The length of a String value in code units.
    pub fn str_len(&self, s: JsString) -> usize {
        self.str(s).len()
    }

    /// Lossy conversion to Rust text (lone surrogates become U+FFFD).
    pub fn str_lossy(&self, s: JsString) -> String {
        String::from_utf16_lossy(self.str(s))
    }

    /// The string-concatenation of `a` and `b`.
    pub fn concat(&mut self, a: JsString, b: JsString) -> JsString {
        let mut v = Vec::with_capacity(self.str_len(a) + self.str_len(b));
        v.extend_from_slice(self.str(a));
        v.extend_from_slice(self.str(b));
        self.intern(&v)
    }

    /// The substring of `s` from `from` to `to` (code unit indices, clamped).
    pub fn substring(&mut self, s: JsString, from: usize, to: usize) -> JsString {
        let len = self.str_len(s);
        let (from, to) = (from.min(len), to.min(len));
        if from >= to {
            return self.names.empty;
        }
        let v = self.str(s)[from..to].to_vec();
        self.intern(&v)
    }

    // ---- symbols ---------------------------------------------------------

    /// A new Private Name (§6.2.12) with the given description.
    pub fn new_private_name(&mut self, description: JsString) -> crate::function::PrivateName {
        self.next_private_name += 1;
        crate::function::PrivateName {
            id: self.next_private_name,
            description,
        }
    }

    pub fn new_symbol(&mut self, description: Option<JsString>) -> Symbol {
        self.symbols.push(SymbolData { description });
        Symbol((self.symbols.len() - 1) as u32)
    }

    pub fn symbol_description(&self, s: Symbol) -> Option<JsString> {
        self.symbols[s.0 as usize].description
    }

    // ---- property keys ---------------------------------------------------

    /// The property key for a String value (canonical array indices become
    /// `PropertyKey::Index`).
    pub fn key_from_string(&self, s: JsString) -> PropertyKey {
        match array_index_of(self.str(s)) {
            Some(i) => PropertyKey::Index(i),
            None => PropertyKey::String(s),
        }
    }

    /// The property key for Rust text, e.g. `agent.key("length")`.
    pub fn key(&mut self, s: &str) -> PropertyKey {
        let js = self.string(s);
        self.key_from_string(js)
    }

    /// The property key for an integral number index `i` (any size, e.g. a
    /// `k` from a spec loop): `! ToString(𝔽(i))`.
    pub fn key_from_index(&mut self, i: f64) -> PropertyKey {
        if (0.0..4_294_967_295.0).contains(&i) && i.fract() == 0.0 {
            return PropertyKey::Index(i as u32);
        }
        let s = crate::ops::number_to_js_string(self, i);
        PropertyKey::String(s)
    }

    /// The String (or Symbol) value of a property key, as a language value.
    pub fn key_to_value(&mut self, key: PropertyKey) -> Value {
        match key {
            PropertyKey::Index(i) => Value::String(self.string(&i.to_string())),
            PropertyKey::String(s) => Value::String(s),
            PropertyKey::Symbol(s) => Value::Symbol(s),
        }
    }

    /// Debug-friendly rendering of a property key.
    pub fn key_display(&self, key: PropertyKey) -> String {
        match key {
            PropertyKey::Index(i) => i.to_string(),
            PropertyKey::String(s) => self.str_lossy(s),
            PropertyKey::Symbol(s) => match self.symbol_description(s) {
                Some(d) => format!("[{}]", self.str_lossy(d)),
                None => "[symbol]".into(),
            },
        }
    }

    // ---- realms and contexts ----------------------------------------------

    /// The realm of the running execution context.
    pub fn current_realm(&self) -> RealmId {
        self.contexts.last().map(|c| c.realm).unwrap_or(RealmId(0))
    }

    pub fn realm(&self, id: RealmId) -> &Realm {
        &self.realms[id.0 as usize]
    }

    pub fn realm_mut(&mut self, id: RealmId) -> &mut Realm {
        &mut self.realms[id.0 as usize]
    }

    /// An intrinsic of the current realm by its spec name, e.g.
    /// `agent.intrinsic("%Array.prototype%")`.
    pub fn intrinsic(&self, name: &str) -> Object {
        self.intrinsic_in(self.current_realm(), name)
    }

    pub fn intrinsic_in(&self, realm: RealmId, name: &str) -> Object {
        match self.realm(realm).intrinsics.get(name) {
            Some(&o) => o,
            None => panic!("intrinsic {name} is not installed"),
        }
    }

    pub fn has_intrinsic(&self, name: &str) -> bool {
        self.realm(self.current_realm())
            .intrinsics
            .contains_key(name)
    }

    /// The active function object (the function the running context runs).
    pub fn active_function(&self) -> Option<Object> {
        self.contexts.last().and_then(|c| c.function)
    }

    // ---- environments ------------------------------------------------------

    pub fn alloc_env(&mut self, env: Environment) -> EnvId {
        self.envs.push(env);
        EnvId((self.envs.len() - 1) as u32)
    }

    pub fn env(&self, id: EnvId) -> &Environment {
        &self.envs[id.0 as usize]
    }

    pub fn env_mut(&mut self, id: EnvId) -> &mut Environment {
        &mut self.envs[id.0 as usize]
    }

    // ---- errors ------------------------------------------------------------

    /// A new error object from the current realm's `%NativeError%` constructor.
    pub fn create_error(&mut self, kind: ErrorKind, message: &str) -> Object {
        let proto = self.intrinsic(kind.prototype_intrinsic());
        let o = self.object_with_kind(Some(proto), ObjectKind::Error);
        let msg = self.string(message);
        let key = PropertyKey::String(self.names.message);
        self.obj_mut(o).properties.insert(
            key,
            crate::object::Property::Data {
                value: Value::String(msg),
                writable: true,
                enumerable: false,
                configurable: true,
            },
        );
        o
    }

    pub fn throw_error(&mut self, kind: ErrorKind, message: &str) -> JsError {
        JsError::Throw(Value::Object(self.create_error(kind, message)))
    }

    pub fn throw_type_error(&mut self, message: &str) -> JsError {
        self.throw_error(ErrorKind::Type, message)
    }

    pub fn throw_range_error(&mut self, message: &str) -> JsError {
        self.throw_error(ErrorKind::Range, message)
    }

    pub fn throw_reference_error(&mut self, message: &str) -> JsError {
        self.throw_error(ErrorKind::Reference, message)
    }

    pub fn throw_syntax_error(&mut self, message: &str) -> JsError {
        self.throw_error(ErrorKind::Syntax, message)
    }

    /// The engine has not implemented `what` yet. Uncatchable by JavaScript.
    pub fn not_implemented(&mut self, what: &str) -> JsError {
        JsError::NotImplemented(what.to_string())
    }

    // ---- jobs --------------------------------------------------------------

    /// HostEnqueuePromiseJob / HostEnqueueGenericJob.
    pub fn enqueue_job(&mut self, job: Job) {
        self.jobs.push_back(job);
    }

    /// Runs queued jobs until the queue is empty.
    pub fn run_jobs(&mut self) -> JsResult<()> {
        while let Some(job) = self.jobs.pop_front() {
            job(self)?;
        }
        Ok(())
    }
}

impl Default for Agent {
    fn default() -> Self {
        Agent::new()
    }
}

/// The NativeError kinds (§20.5.5) plus Error itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Error,
    Eval,
    Range,
    Reference,
    Syntax,
    Type,
    Uri,
}

impl ErrorKind {
    pub fn prototype_intrinsic(self) -> &'static str {
        match self {
            ErrorKind::Error => "%Error.prototype%",
            ErrorKind::Eval => "%EvalError.prototype%",
            ErrorKind::Range => "%RangeError.prototype%",
            ErrorKind::Reference => "%ReferenceError.prototype%",
            ErrorKind::Syntax => "%SyntaxError.prototype%",
            ErrorKind::Type => "%TypeError.prototype%",
            ErrorKind::Uri => "%URIError.prototype%",
        }
    }
}

/// If `units` is a canonical array index ("0" … "4294967294"), its value.
pub fn array_index_of(units: &[u16]) -> Option<u32> {
    if units.is_empty() || units.len() > 10 {
        return None;
    }
    if units.len() > 1 && units[0] == b'0' as u16 {
        return None;
    }
    let mut v: u64 = 0;
    for &u in units {
        if !(b'0' as u16..=b'9' as u16).contains(&u) {
            return None;
        }
        v = v * 10 + (u - b'0' as u16) as u64;
    }
    (v < 4_294_967_295).then_some(v as u32)
}
