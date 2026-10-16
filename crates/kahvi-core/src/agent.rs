//! The agent (ECMA-262 §9.7): owns the heap, realms, the execution context
//! stack and the job queue. Every engine operation takes `&mut Agent`.

use std::collections::VecDeque;
use std::sync::Arc;

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

/// One frame of a captured call stack (`Agent::error_stack`).
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// The function's own "name" data property; None for script, module
    /// and eval code, empty for a function without a name.
    pub name: Option<Box<str>>,
    /// A built-in function: no source position.
    pub native: bool,
    /// Called as a constructor.
    pub construct: bool,
    /// The instruction running in the frame, when known.
    pub location: Option<StackLocation>,
}

/// Where a stack frame was: its script and the source offset of the
/// instruction it was running.
#[derive(Debug, Clone)]
pub struct StackLocation {
    pub script: u32,
    pub source: Arc<str>,
    pub offset: u32,
}

/// An execution context (§9.4): which function and realm are running.
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub function: Option<Object>,
    pub realm: RealmId,
    pub script_or_module: Option<crate::ScriptOrModule>,
}

/// A pending job (§9.5), run when the execution context stack is empty.
/// Jobs are data (not closures) so the collector can trace them.
#[derive(Debug, Clone, Copy)]
pub enum Job {
    /// NewPromiseReactionJob(reaction, argument)
    PromiseReaction {
        realm: RealmId,
        reaction: crate::ops::promise::PromiseReaction,
        argument: Value,
    },
    /// NewPromiseResolveThenableJob(promiseToResolve, thenable, then)
    PromiseResolveThenable {
        realm: RealmId,
        promise: Object,
        thenable: Value,
        then: Value,
    },
    /// HostEnqueueFinalizationRegistryCleanupJob(finalizationRegistry)
    FinalizationRegistryCleanup { realm: RealmId, registry: Object },
    /// A host's job (HostEnqueueGenericJob, e.g. `queueMicrotask`): calls
    /// `callback` with no arguments; an exception is the job's result.
    Callback { realm: RealmId, callback: Value },
}

impl crate::gc::Trace for Job {
    fn trace(&self, t: &mut crate::gc::Tracer) {
        match self {
            Job::PromiseReaction {
                reaction, argument, ..
            } => {
                reaction.trace(t);
                t.value(*argument);
            }
            Job::PromiseResolveThenable {
                promise,
                thenable,
                then,
                ..
            } => {
                t.object(*promise);
                t.value(*thenable);
                t.value(*then);
            }
            Job::FinalizationRegistryCleanup { registry, .. } => t.object(*registry),
            Job::Callback { callback, .. } => t.value(*callback),
        }
    }
}

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
    pub stack_trace_limit: JsString,
}

/// Installs the built-in objects into a fresh realm. Provided by the
/// `kahvi-builtins` crate when the runtime is created.
pub type IntrinsicsInstaller = fn(&mut Agent, RealmId) -> JsResult<()>;

pub struct Agent {
    pub(crate) objects: Vec<ObjectData>,
    pub(crate) strings: Vec<Box<[u16]>>,
    /// The intern table: handles hashed by the contents they name (stored once, in `strings`).
    pub(crate) string_index: crate::hash::StringTable,
    pub(crate) symbols: Vec<SymbolData>,
    /// The garbage collector's state (docs/GC.md).
    pub(crate) gc: crate::gc::GcState,
    /// Empty vectors kept from finished frames (operand stacks, argument
    /// lists) so a call does not allocate new ones. They hold no values.
    pub(crate) spare_vecs: Vec<Vec<Value>>,
    /// Exception-handler lists kept for reuse, like `spare_vecs`.
    pub(crate) spare_handlers: Vec<Vec<crate::vm::Handler>>,
    /// Collect at one safe point in `gc_stress` on average, regardless of
    /// heap growth (finds rooting bugs; set by the host, see
    /// [`Agent::gc_stress_from_env`]). 0 is off, 1 collects at every safe
    /// point.
    pub gc_stress: u32,
    /// Realms created from now on have the `Intl` global (ECMA-402). Off by
    /// default while ECMA-402 is incomplete (docs/INTL.md): code that
    /// feature-tests `Intl` would otherwise reach not-implemented stubs.
    pub intl: bool,
    /// [[KeptAlive]]: WeakRef targets kept until the end of the current job.
    pub(crate) kept_objects: Vec<Object>,
    pub(crate) kept_symbols: Vec<Value>,
    /// Live FinalizationRegistry objects (held weakly), for cleanup scheduling.
    pub(crate) finalization_registries: Vec<Object>,
    pub(crate) envs: Vec<Environment>,
    pub realms: Vec<Realm>,
    pub contexts: Vec<ExecutionContext>,
    pub jobs: VecDeque<Job>,
    /// The jobs being run (a job can run others only through the host, so
    /// this is at most a few deep); their data is a root.
    pub(crate) running_jobs: Vec<Job>,
    /// [[GlobalSymbolRegistry]]: Symbol.for keys.
    pub symbol_registry: Vec<(JsString, Symbol)>,
    pub symbols_wk: WellKnownSymbols,
    pub names: Names,
    pub call_depth: usize,
    /// The native stack address where the outermost call began (set when
    /// `call_depth` leaves 0).
    pub(crate) stack_base: usize,
    /// How many bytes of native stack calls may use below `stack_base`
    /// before a RangeError; 0 is no limit beyond [`MAX_CALL_DEPTH`]. Hosts
    /// that run JavaScript on small thread stacks set it (React Native's
    /// JS thread on Android has about 1 MiB).
    pub native_stack_limit: usize,
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
    /// Host hook: runs its argument on the current thread, after whatever
    /// setup the host needs on a thread the engine made (a deep call on a
    /// larger stack, see `native_stack_limit`), such as attaching it to the
    /// JVM so host functions can use JNI.
    pub host_thread_wrapper: Option<fn(&mut dyn FnMut())>,
    /// Host hook: LocalTZA(t, isUTC) (§21.4.1.25), the local time zone
    /// offset in milliseconds. Without it, local time is UTC.
    pub host_local_tza: Option<fn(f64, bool) -> f64>,
    /// Module Records, by index.
    pub modules: Vec<crate::vm::ModuleRecord>,
    /// The module loaded for each (realm, host key), so a module is loaded once.
    pub module_map: crate::hash::FxHashMap<(crate::realm::RealmId, Box<str>), u32>,
    /// IncrementModuleAsyncEvaluationCount's counter.
    pub module_async_evaluation_count: u64,
    /// Host hook: HostLoadImportedModule's loading step. Without it, every
    /// import fails.
    pub host_load_module: Option<crate::vm::HostLoadModule>,
    /// Host hook: HostGetImportMetaProperties, called with the new import.meta object.
    pub host_import_meta: Option<fn(&mut Agent, u32, Object)>,
    /// Values the host holds across calls (`Agent::persist`).
    pub(crate) persistent: crate::host::PersistentTable,
    /// The closures behind `Agent::host_function` objects.
    pub(crate) host_functions: Vec<crate::host::HostFn>,
    /// Data the host attached to objects (`Agent::set_native_state`).
    pub(crate) native_states: crate::hash::FxHashMap<Object, crate::host::NativeState>,
    /// The call stack when each error object was created, innermost frame
    /// first (`Agent::error_stack`).
    pub(crate) error_stacks: crate::hash::FxHashMap<Object, Box<[StackFrame]>>,
}

impl std::fmt::Debug for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Agent")
            .field("objects", &self.objects.len())
            .finish()
    }
}

impl Agent {
    /// An empty vector, reusing one a finished frame gave back.
    pub(crate) fn spare_vec(&mut self) -> Vec<Value> {
        self.spare_vecs.pop().unwrap_or_default()
    }

    /// Keeps `v`'s allocation for `spare_vec` (a bounded number of them).
    pub(crate) fn give_back_vec(&mut self, mut v: Vec<Value>) {
        if self.spare_vecs.len() < 64 && v.capacity() <= 1024 {
            v.clear();
            self.spare_vecs.push(v);
        }
    }

    pub fn new() -> Agent {
        let placeholder = JsString(0);
        let mut agent = Agent {
            objects: Vec::new(),
            strings: Vec::new(),
            string_index: Default::default(),
            symbols: Vec::new(),
            gc: Default::default(),
            spare_vecs: Vec::new(),
            spare_handlers: Vec::new(),
            gc_stress: 0,
            intl: false,
            kept_objects: Vec::new(),
            kept_symbols: Vec::new(),
            finalization_registries: Vec::new(),
            envs: Vec::new(),
            realms: Vec::new(),
            contexts: Vec::new(),
            jobs: VecDeque::new(),
            running_jobs: Vec::new(),
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
                stack_trace_limit: placeholder,
            },
            call_depth: 0,
            stack_base: 0,
            native_stack_limit: 0,
            installer: None,
            rejection_tracker: Vec::new(),
            scripts: Vec::new(),
            next_private_name: 0,
            bigints: Vec::new(),
            regexp_matcher_check: None,
            modules: Vec::new(),
            module_map: Default::default(),
            module_async_evaluation_count: 0,
            host_load_module: None,
            host_import_meta: None,
            persistent: Default::default(),
            host_functions: Vec::new(),
            native_states: Default::default(),
            error_stacks: Default::default(),
            host_now: None,
            host_thread_wrapper: None,
            host_local_tza: None,
        };
        // The common names and well-known symbols live as long as the agent.
        agent.gc.pinning = true;
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
            stack_trace_limit: agent.string("stackTraceLimit"),
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
        agent.gc.pinning = false;
        agent.gc.pinned_symbols = agent.symbols.len() as u32;
        agent
    }

    /// Sets the function that installs built-ins into new realms.
    pub fn set_intrinsics_installer(&mut self, f: IntrinsicsInstaller) {
        self.installer = Some(f);
    }

    // ---- objects -------------------------------------------------------

    pub fn alloc(&mut self, data: ObjectData) -> Object {
        self.gc.allocated += 1;
        let is_error = matches!(data.kind, ObjectKind::Error);
        let o = if let Some(i) = self.gc.free_objects.pop() {
            self.gc.is_free_object.remove(i);
            self.objects[i as usize] = data;
            Object(i)
        } else {
            self.objects.push(data);
            Object((self.objects.len() - 1) as u32)
        };
        if is_error && let Some(stack) = self.capture_stack(None) {
            self.error_stacks.insert(o, stack);
        }
        o
    }

    /// The execution context stack, innermost first, as V8 and Hermes
    /// capture it: at most `Error.stackTraceLimit` frames (read as a data
    /// property of the current realm's %Error%, without running code), and
    /// nothing when that is not a number. The built-in error constructor
    /// creating the object, when it is the innermost context, is left out.
    /// With `skip_through`, frames up to and including the innermost call
    /// of that function are left out too (`Error.captureStackTrace`), and
    /// the stack is empty when it is not on the stack.
    #[cold]
    #[inline(never)]
    pub fn capture_stack(&self, skip_through: Option<Object>) -> Option<Box<[StackFrame]>> {
        let error = *self.realm(self.current_realm()).intrinsics.get("%Error%")?;
        let limit = match self
            .obj(error)
            .properties
            .get(&PropertyKey::String(self.names.stack_trace_limit))
        {
            Some(crate::object::Property::Data {
                value: Value::Number(n),
                ..
            }) if n.is_nan() || *n <= 0.0 => 0,
            Some(crate::object::Property::Data {
                value: Value::Number(n),
                ..
            }) => n.min(usize::MAX as f64) as usize,
            _ => return None,
        };
        let name_of = |f: Object| match self
            .obj(f)
            .properties
            .get(&PropertyKey::String(self.names.name))
        {
            Some(crate::object::Property::Data {
                value: Value::String(s),
                ..
            }) => self.str_lossy(*s),
            _ => String::new(),
        };
        let is_native = |f: Object| matches!(self.obj(f).kind, ObjectKind::Builtin(_));
        let mut contexts = self.contexts.iter().rev().peekable();
        if skip_through.is_none()
            && let Some(f) = contexts.peek().and_then(|c| c.function)
            && is_native(f)
            && name_of(f).ends_with("Error")
        {
            contexts.next();
        }
        // The frames running ECMAScript code, innermost first: each such
        // context's frame is the next one running the same function.
        // SAFETY: see FramePtr; the frames outlive this call.
        let frames: Vec<&crate::vm::Frame> = self
            .gc
            .frames
            .iter()
            .rev()
            .map(|f| unsafe { &*f.0 })
            .collect();
        let mut next_frame = 0;
        let mut skipping = skip_through.is_some();
        let mut out = Vec::new();
        for ctx in contexts {
            let native = ctx.function.is_some_and(is_native);
            let mut frame = None;
            if !native
                && let Some(i) = frames[next_frame..]
                    .iter()
                    .position(|f| f.function == ctx.function)
            {
                frame = Some(frames[next_frame + i]);
                next_frame += i + 1;
            }
            if skipping {
                skipping = ctx.function != skip_through;
                continue;
            }
            let name = ctx.function.map(|f| name_of(f).into_boxed_str());
            // Left out, as V8 does: the host's own contexts (running no
            // code, such as a job's) and anonymous built-in functions (such
            // as a promise's resolving functions).
            let shown = match (native, frame) {
                (true, _) => name.as_deref().is_some_and(|n| !n.is_empty()),
                (false, frame) => frame.is_some() || ctx.function.is_some(),
            };
            if !shown {
                continue;
            }
            if out.len() >= limit {
                break;
            }
            // A function's own function environment holds its new.target.
            let construct = frame.is_some_and(|frame| {
                let env = self.get_this_environment(frame.env);
                env.function_data(self).is_some_and(|d| {
                    Some(d.function_object) == frame.function
                        && matches!(d.new_target, Value::Object(_))
                })
            });
            let location = frame.and_then(|frame| {
                frame
                    .code
                    .position_of(frame.pc)
                    .map(|offset| StackLocation {
                        script: frame.code.script,
                        source: frame.code.source.clone(),
                        offset,
                    })
            });
            out.push(StackFrame {
                name,
                native,
                construct,
                location,
            });
        }
        Some(out.into_boxed_slice())
    }

    /// `frames` as V8 writes them, one `    at ...` line each:
    /// `    at name (file:line:column)`, and `name (native)` for built-in
    /// functions. Code outside any named function is
    /// `    at <anonymous> (file:line:column)` (where V8 leaves out the name
    /// and parentheses), so that React Native's Hermes stack parser, which
    /// it uses when `HermesInternal` exists, reads every frame too.
    pub fn format_stack(&self, frames: &[StackFrame]) -> String {
        let mut out = String::new();
        for frame in frames {
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str("    at ");
            let place = match &frame.location {
                Some(l) => {
                    let (line, col) = crate::vm::line_col(&l.source, l.offset as usize);
                    let file = self
                        .scripts
                        .get(l.script as usize)
                        .map_or("<anonymous>", |s| s.name.as_str());
                    format!("{file}:{line}:{col}")
                }
                None if frame.native => "native".to_string(),
                None => "<anonymous>".to_string(),
            };
            let name = match frame.name.as_deref() {
                Some(name) if !name.is_empty() => name,
                _ => "<anonymous>",
            };
            if frame.construct {
                out.push_str("new ");
            }
            out.push_str(name);
            out.push_str(" (");
            out.push_str(&place);
            out.push(')');
        }
        out
    }

    /// Records `stack` as error object `o`'s captured stack.
    pub fn set_error_stack(&mut self, o: Object, stack: Box<[StackFrame]>) {
        self.error_stacks.insert(o, stack);
    }

    /// The stack captured when error object `o` was created (an object with
    /// an [[ErrorData]] internal slot), for `Error.prototype.stack`.
    pub fn error_stack(&self, o: Object) -> Option<&[StackFrame]> {
        self.error_stacks.get(&o).map(|s| &**s)
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
        if let Some(id) = self.string_index.find(&self.strings, units) {
            if self.gc.pinning {
                self.gc.pinned_strings.insert(id);
            } else if let Some(unit) = &mut self.gc.compiling {
                unit.strings.push(id);
            }
            return JsString(id);
        }
        self.gc.allocated += 1;
        let id = match self.gc.free_strings.pop() {
            Some(i) => {
                self.gc.is_free_string.remove(i);
                self.strings[i as usize] = units.into();
                i
            }
            None => {
                self.strings.push(units.into());
                (self.strings.len() - 1) as u32
            }
        };
        self.string_index.insert(&self.strings, id);
        if self.gc.pinning {
            self.gc.pinned_strings.insert(id);
        } else if let Some(unit) = &mut self.gc.compiling {
            unit.strings.push(id);
        }
        JsString(id)
    }

    /// A String value from Rust text.
    pub fn string(&mut self, s: &str) -> JsString {
        // A string has no more UTF-16 code units than UTF-8 bytes, so short
        // ones (most names and keys) fit on the stack.
        if s.len() <= 64 {
            let mut buf = [0u16; 64];
            let mut n = 0;
            for u in s.encode_utf16() {
                buf[n] = u;
                n += 1;
            }
            return self.intern(&buf[..n]);
        }
        let mut units = Vec::with_capacity(s.len());
        units.extend(s.encode_utf16());
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
        self.gc.allocated += 1;
        if let Some(i) = self.gc.free_symbols.pop() {
            self.gc.is_free_symbol.remove(i);
            self.symbols[i as usize] = SymbolData { description };
            return Symbol(i);
        }
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
        self.gc.allocated += 1;
        if let Some(i) = self.gc.free_envs.pop() {
            self.gc.is_free_env.remove(i);
            self.envs[i as usize] = env;
            return EnvId(i);
        }
        self.envs.push(env);
        EnvId((self.envs.len() - 1) as u32)
    }

    /// A new declarative (function, module) environment record. A collected
    /// record's slot is refilled in place, so its allocations are reused.
    pub(crate) fn alloc_declarative(
        &mut self,
        outer: Option<EnvId>,
        function: Option<crate::env::FunctionEnvData>,
        is_module: bool,
    ) -> EnvId {
        if let Some(&i) = self.gc.free_envs.last()
            && let Environment::Declarative {
                bindings,
                outer: o,
                function: f,
                imports,
                is_module: m,
                is_catch,
                disposable_resource_stack,
            } = &mut self.envs[i as usize]
        {
            self.gc.free_envs.pop();
            self.gc.is_free_env.remove(i);
            self.gc.allocated += 1;
            bindings.clear();
            *o = outer;
            match (f.as_mut(), function) {
                (Some(b), Some(data)) => **b = data,
                (_, data) => *f = data.map(Box::new),
            }
            imports.clear();
            *m = is_module;
            *is_catch = false;
            disposable_resource_stack.clear();
            return EnvId(i);
        }
        self.alloc_env(Environment::Declarative {
            bindings: Default::default(),
            outer,
            function: function.map(Box::new),
            imports: Default::default(),
            is_module,
            is_catch: false,
            disposable_resource_stack: Vec::new(),
        })
    }

    /// Frees environment `id` now, for code that knows nothing references
    /// it any more (`FunctionCode::envs_die_with_call`): its slot and
    /// allocations go back to `alloc_declarative` without waiting for a
    /// collection.
    pub(crate) fn free_env(&mut self, id: EnvId) {
        let i = id.0;
        debug_assert!(!self.gc.is_free_env.get(i), "environment freed twice");
        match &mut self.envs[i as usize] {
            Environment::Declarative {
                bindings,
                outer,
                imports,
                disposable_resource_stack,
                ..
            } if bindings.capacity() <= 64 => {
                bindings.clear();
                *outer = None;
                imports.clear();
                disposable_resource_stack.clear();
            }
            e => *e = Environment::dead(),
        }
        self.gc.is_free_env.insert(i);
        self.gc.free_envs.push(i);
        self.gc.allocated = self.gc.allocated.saturating_sub(1);
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
        // ClearKeptObjects(): the synchronous run that queued these jobs is over.
        self.kept_objects.clear();
        self.kept_symbols.clear();
        while let Some(job) = self.jobs.pop_front() {
            self.running_jobs.push(job);
            let r = crate::ops::promise::run_job(self, job);
            self.running_jobs.pop();
            // ClearKeptObjects(): the job is over.
            self.kept_objects.clear();
            self.kept_symbols.clear();
            r?;
            // Between jobs nothing but the roots holds handles.
            self.gc_safe_point();
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
