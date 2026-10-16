//! kahvi's embedding API: a `Runtime` is one agent with one realm, its
//! built-ins installed. The host owns the outside world (clocks, module
//! loading, I/O, the event loop) and wires it in through the agent's host
//! hooks; `kahvi-runtime` is one such host.
//!
//! Values returned by these methods are handles into the runtime's heap. A
//! handle stays valid until the next call that can run JavaScript (which may
//! collect garbage); to keep one longer, `persist` it.

pub use kahvi_core::agent::Agent;
pub use kahvi_core::host::Persistent;
pub use kahvi_core::vm::{HostLoadModule, LoadedModuleSource};
pub use kahvi_core::{JsError, JsResult, JsString, Object, RealmId, Value};

pub mod encoding;

use kahvi_core::agent::ExecutionContext;
use kahvi_core::object::Property;
use kahvi_core::{ops, vm};

/// One agent and its realm.
pub struct Runtime {
    agent: Agent,
    realm: RealmId,
}

/// A runtime moves between threads (JSI creates it on one and uses it on another).
const _: () = {
    const fn assert_send<T: Send>() {}
    assert_send::<Runtime>();
};

impl Runtime {
    /// A runtime with the ECMAScript built-ins and nothing from the host.
    pub fn new() -> Runtime {
        Runtime::with_intl(false)
    }

    /// As [`Runtime::new`], with the `Intl` global (ECMA-402) when `intl`
    /// (docs/INTL.md: off by default while ECMA-402 is incomplete).
    pub fn with_intl(intl: bool) -> Runtime {
        let mut agent = Agent::new();
        agent.intl = intl;
        agent.set_intrinsics_installer(kahvi_builtins::install);
        let realm = agent
            .create_realm()
            .expect("creating the first realm cannot throw");
        Runtime { agent, realm }
    }

    /// The agent, for what this API does not cover.
    pub fn agent(&mut self) -> &mut Agent {
        &mut self.agent
    }

    pub fn realm(&self) -> RealmId {
        self.realm
    }

    pub fn global(&self) -> Object {
        self.agent.realm(self.realm).global_object
    }

    /// Runs `f` with an execution context for the runtime's realm (the host
    /// calling into the engine from outside any JavaScript).
    pub fn enter<T>(&mut self, f: impl FnOnce(&mut Agent) -> T) -> T {
        self.agent.contexts.push(ExecutionContext {
            function: None,
            realm: self.realm,
            script_or_module: None,
        });
        let r = f(&mut self.agent);
        self.agent.contexts.pop();
        r
    }

    /// Defines `globalThis[name] = value` (writable, configurable, not enumerable).
    pub fn set_global(&mut self, name: &str, value: Value) {
        let global = self.global();
        define_hidden(&mut self.agent, global, name, value);
    }

    /// A function implemented by `f`, in the runtime's realm.
    pub fn function(
        &mut self,
        name: &str,
        length: u32,
        f: impl Fn(&mut Agent, Value, &[Value]) -> JsResult<Value> + Send + Sync + 'static,
    ) -> Object {
        self.enter(|agent| agent.host_function(name, length, f))
    }

    /// Defines a global function implemented by `f`.
    pub fn set_global_function(
        &mut self,
        name: &str,
        length: u32,
        f: impl Fn(&mut Agent, Value, &[Value]) -> JsResult<Value> + Send + Sync + 'static,
    ) {
        let func = self.function(name, length, f);
        self.set_global(name, Value::Object(func));
    }

    /// Parses and evaluates a script; returns its completion value.
    pub fn eval_script(&mut self, source: &str, name: &str) -> JsResult<Value> {
        let realm = self.realm;
        let script = vm::parse_script(&mut self.agent, source, name, realm)?;
        vm::evaluate_script(&mut self.agent, &script)
    }

    /// Loads, links and evaluates the module `key` (its source is `source`;
    /// its imports go through the agent's `host_load_module`). Returns the
    /// evaluation's promise.
    pub fn eval_module(&mut self, key: &str, source: &str) -> JsResult<Object> {
        let realm = self.realm;
        let promise = self.enter(|agent| vm::run_module(agent, realm, key, source))?;
        match promise {
            Value::Object(p) => Ok(p),
            _ => unreachable!("module evaluation returns a promise"),
        }
    }

    /// Call(f, this, args). The arguments are rooted for the call, so the
    /// JavaScript it runs may collect garbage.
    pub fn call(&mut self, f: Value, this: Value, args: &[Value]) -> JsResult<Value> {
        let mut roots = Vec::with_capacity(args.len() + 2);
        roots.push(f);
        roots.push(this);
        roots.extend_from_slice(args);
        self.enter(|agent| agent.with_roots(&roots, |agent| ops::call_rooted(agent, f, this, args)))
    }

    /// Runs the queued jobs (promise reactions) until there are none.
    pub fn run_jobs(&mut self) -> JsResult<()> {
        self.enter(|agent| agent.run_jobs())
    }

    /// Holds `v` across calls until the `Persistent` is dropped.
    pub fn persist(&mut self, v: Value) -> Persistent {
        self.agent.persist(v)
    }

    pub fn persisted(&self, p: &Persistent) -> Value {
        self.agent.persisted(p)
    }

    /// The state of a promise: `None` while pending, else its fulfillment
    /// value or rejection reason.
    pub fn promise_result(&self, p: Object) -> Option<Result<Value, Value>> {
        ops::promise::promise_result(&self.agent, p)
    }

    /// Promises rejected with no handler since the last call, in rejection
    /// order (HostPromiseRejectionTracker, with handlers added later
    /// cancelling the report).
    pub fn take_unhandled_rejections(&mut self) -> Vec<Object> {
        let mut unhandled: Vec<Object> = Vec::new();
        for (p, handled) in std::mem::take(&mut self.agent.rejection_tracker) {
            if handled {
                unhandled.retain(|&q| q != p);
            } else if !unhandled.contains(&p) {
                unhandled.push(p);
            }
        }
        unhandled
    }

    /// `ToString(v)` without running user code that throws: the string, or
    /// a description of the exception it threw.
    pub fn to_display_string(&mut self, v: Value) -> String {
        self.enter(|agent| match ops::to_string(agent, v) {
            Ok(s) => agent.str_lossy(s),
            Err(_) => "<exception while converting to a string>".into(),
        })
    }

    /// One line for an uncaught exception: `Name: message` for errors.
    pub fn describe_error(&mut self, e: &JsError) -> String {
        match e {
            JsError::NotImplemented(what) => format!("InternalError: not implemented: {what}"),
            JsError::Throw(v) => self.enter(|agent| describe_thrown(agent, *v)),
        }
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Runtime::new()
    }
}

/// `Name: message` for a thrown error object, else ToString of the value.
pub fn describe_thrown(agent: &mut Agent, v: Value) -> String {
    if let Value::Object(o) = v {
        let name = error_name(agent, o).unwrap_or_else(|| "Error".into());
        let mk = agent.key("message");
        let message = ops::get(agent, o, mk)
            .ok()
            .filter(|v| !v.is_undefined())
            .and_then(|v| ops::to_string(agent, v).ok())
            .map(|s| agent.str_lossy(s))
            .unwrap_or_default();
        if message.is_empty() {
            return name;
        }
        return format!("{name}: {message}");
    }
    ops::to_string(agent, v)
        .map(|s| agent.str_lossy(s))
        .unwrap_or_else(|_| "exception".into())
}

/// The `name` of the thrown object's constructor, else its own `name`.
fn error_name(agent: &mut Agent, o: Object) -> Option<String> {
    let ck = agent.key("constructor");
    let nk = agent.key("name");
    if let Ok(Value::Object(c)) = ops::get(agent, o, ck)
        && let Ok(Value::String(s)) = ops::get(agent, c, nk)
    {
        return Some(agent.str_lossy(s));
    }
    match ops::get(agent, o, nk) {
        Ok(Value::String(s)) => Some(agent.str_lossy(s)),
        _ => None,
    }
}

/// Defines a writable, configurable, non-enumerable data property (how host
/// globals are installed).
pub fn define_hidden(agent: &mut Agent, o: Object, name: &str, value: Value) {
    let k = agent.key(name);
    agent.obj_mut(o).properties.insert(
        k,
        Property::Data {
            value,
            writable: true,
            enumerable: false,
            configurable: true,
        },
    );
}
