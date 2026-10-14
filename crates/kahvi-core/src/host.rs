//! What an embedder needs beyond the spec: values held across calls
//! (`Persistent`) and functions implemented by Rust closures.

use std::sync::{Arc, Weak};

use crate::agent::Agent;
use crate::error::JsResult;
use crate::function::{builtin_slots, create_builtin_function};
use crate::value::{Object, Value};

/// Host data attached to an object (JSI's NativeState); dropped when the
/// object is collected.
pub type NativeState = Arc<dyn std::any::Any + Send + Sync>;

/// A host function: called with the agent, the this value and the arguments.
pub type HostFn = Arc<dyn Fn(&mut Agent, Value, &[Value]) -> JsResult<Value> + Send + Sync>;

/// A value the host holds across calls. The value is a garbage collection
/// root while any clone of its `Persistent` exists.
#[derive(Clone)]
pub struct Persistent {
    slot: u32,
    token: Arc<()>,
}

impl std::fmt::Debug for Persistent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Persistent({})", self.slot)
    }
}

/// The persistent values: one slot per `persist`, freed by the first
/// collection after the last `Persistent` for it is dropped.
#[derive(Default)]
pub(crate) struct PersistentTable {
    slots: Vec<Option<(Value, Weak<()>)>>,
    free: Vec<u32>,
}

impl PersistentTable {
    /// Frees the slots no `Persistent` refers to any more.
    pub(crate) fn release_dropped(&mut self) {
        for (i, s) in self.slots.iter_mut().enumerate() {
            if matches!(s, Some((_, w)) if w.strong_count() == 0) {
                *s = None;
                self.free.push(i as u32);
            }
        }
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = Value> + '_ {
        self.slots.iter().flatten().map(|(v, _)| *v)
    }
}

impl Agent {
    /// Holds `v` for the host until the returned `Persistent` (and all its
    /// clones) are dropped.
    pub fn persist(&mut self, v: Value) -> Persistent {
        let token = Arc::new(());
        let entry = Some((v, Arc::downgrade(&token)));
        let table = &mut self.persistent;
        let slot = match table.free.pop() {
            Some(i) => {
                table.slots[i as usize] = entry;
                i
            }
            None => {
                table.slots.push(entry);
                (table.slots.len() - 1) as u32
            }
        };
        Persistent { slot, token }
    }

    /// The value a `Persistent` holds.
    pub fn persisted(&self, p: &Persistent) -> Value {
        match &self.persistent.slots[p.slot as usize] {
            Some((v, w)) if std::ptr::eq(w.as_ptr(), Arc::as_ptr(&p.token)) => *v,
            _ => panic!("Persistent used with another agent"),
        }
    }

    /// A built-in function object (in the current realm) whose behaviour is
    /// the closure `f`. The closure lives as long as the agent.
    pub fn host_function(
        &mut self,
        name: &str,
        length: u32,
        f: impl Fn(&mut Agent, Value, &[Value]) -> JsResult<Value> + Send + Sync + 'static,
    ) -> Object {
        self.host_functions.push(Arc::new(f));
        let index = (self.host_functions.len() - 1) as f64;
        let key = self.key(name);
        create_builtin_function(
            self,
            call_host_function,
            length,
            key,
            vec![Value::Number(index)],
            None,
            None,
            None,
            false,
        )
    }
}

impl Agent {
    /// Attaches `state` to `o`, replacing any earlier state.
    pub fn set_native_state(&mut self, o: Object, state: NativeState) {
        self.native_states.insert(o, state);
    }

    /// The state attached to `o`.
    pub fn native_state(&self, o: Object) -> Option<NativeState> {
        self.native_states.get(&o).cloned()
    }

    /// The index of a `host_function` object's closure (in creation order),
    /// or `None` for any other value.
    pub fn host_function_index(&self, v: Value) -> Option<usize> {
        let Value::Object(f) = v else { return None };
        match &self.obj(f).kind {
            crate::object::ObjectKind::Builtin(b)
                if std::ptr::fn_addr_eq(b.behaviour, call_host_function as crate::function::NativeFn) =>
            {
                b.slots.first().and_then(|v| v.as_number()).map(|n| n as usize)
            }
            _ => None,
        }
    }
}

/// The behaviour of every `host_function`: its first slot names the closure.
fn call_host_function(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let f = agent.active_function().expect("a host function is running");
    let index = builtin_slots(agent, f)
        .first()
        .and_then(|v| v.as_number())
        .expect("host function index") as usize;
    let behaviour = agent.host_functions[index].clone();
    behaviour(agent, this, args)
}
