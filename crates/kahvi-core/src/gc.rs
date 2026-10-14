//! The garbage collector: mark-sweep over the agent's arenas (docs/GC.md).
//!
//! [`Trace`] is implemented by every type that holds handles; the
//! [`Tracer`] marks what it is shown and walks the heap with an explicit
//! worklist. Collection itself is `Agent::collect_garbage` (agent.rs).

use std::collections::VecDeque;
use std::sync::Arc;

use crate::env::EnvId;
use crate::object::PropertyKey;
use crate::value::{BigInt, JsString, Object, Symbol, Value};

/// A set of arena indices.
#[derive(Default, Clone)]
pub(crate) struct BitSet {
    words: Vec<u64>,
}

impl BitSet {
    pub(crate) fn with_len(n: usize) -> BitSet {
        BitSet {
            words: vec![0; n.div_ceil(64)],
        }
    }

    pub(crate) fn get(&self, i: u32) -> bool {
        let (w, b) = (i as usize / 64, i % 64);
        self.words.get(w).is_some_and(|x| x & (1 << b) != 0)
    }

    /// Sets bit `i`; returns whether it was clear.
    pub(crate) fn insert(&mut self, i: u32) -> bool {
        let (w, b) = (i as usize / 64, i % 64);
        if w >= self.words.len() {
            self.words.resize(w + 1, 0);
        }
        let was_clear = self.words[w] & (1 << b) == 0;
        self.words[w] |= 1 << b;
        was_clear
    }

    pub(crate) fn remove(&mut self, i: u32) {
        let (w, b) = (i as usize / 64, i % 64);
        if let Some(x) = self.words.get_mut(w) {
            *x &= !(1 << b);
        }
    }
}

/// Marks reachable handles. Objects, environments and symbols found live are
/// queued and their contents traced by the collector's drain loop.
#[derive(Default)]
pub struct Tracer {
    pub(crate) objects: BitSet,
    pub(crate) strings: BitSet,
    pub(crate) symbols: BitSet,
    pub(crate) bigints: BitSet,
    pub(crate) envs: BitSet,
    pub(crate) object_work: Vec<u32>,
    pub(crate) env_work: Vec<u32>,
    pub(crate) symbol_work: Vec<u32>,
    /// (key, value) pairs whose value is live only if the key is.
    pub(crate) ephemerons: Vec<(Value, Value)>,
}

impl Tracer {
    pub fn value(&mut self, v: Value) {
        match v {
            Value::String(s) => self.string(s),
            Value::Symbol(s) => self.symbol(s),
            Value::BigInt(b) => self.bigint(b),
            Value::Object(o) => self.object(o),
            Value::Undefined | Value::Null | Value::Boolean(_) | Value::Number(_) => {}
        }
    }

    pub fn object(&mut self, o: Object) {
        if self.objects.insert(o.0) {
            self.object_work.push(o.0);
        }
    }

    pub fn string(&mut self, s: JsString) {
        self.strings.insert(s.0);
    }

    pub fn symbol(&mut self, s: Symbol) {
        if self.symbols.insert(s.0) {
            self.symbol_work.push(s.0);
        }
    }

    pub fn bigint(&mut self, b: BigInt) {
        self.bigints.insert(b.0);
    }

    pub fn env(&mut self, e: EnvId) {
        if self.envs.insert(e.0) {
            self.env_work.push(e.0);
        }
    }

    pub fn key(&mut self, k: PropertyKey) {
        match k {
            PropertyKey::Index(_) => {}
            PropertyKey::String(s) => self.string(s),
            PropertyKey::Symbol(s) => self.symbol(s),
        }
    }

    /// A WeakMap/WeakSet entry: `value` is live only while `key` is.
    pub fn ephemeron(&mut self, key: Value, value: Value) {
        self.ephemerons.push((key, value));
    }

    pub(crate) fn is_live(&self, v: Value) -> bool {
        match v {
            Value::String(s) => self.strings.get(s.0),
            Value::Symbol(s) => self.symbols.get(s.0),
            Value::BigInt(b) => self.bigints.get(b.0),
            Value::Object(o) => self.objects.get(o.0),
            Value::Undefined | Value::Null | Value::Boolean(_) | Value::Number(_) => true,
        }
    }
}

/// What survived marking, for [`Trace::sweep_weak`].
pub struct Liveness<'a> {
    pub(crate) tracer: &'a Tracer,
}

impl Liveness<'_> {
    pub fn is_live(&self, v: Value) -> bool {
        self.tracer.is_live(v)
    }

    pub fn is_live_object(&self, o: Object) -> bool {
        self.tracer.objects.get(o.0)
    }
}

/// A type that holds handles into the heap.
pub trait Trace {
    /// Reports every handle held, strongly, to `t` (weak edges are reported
    /// with [`Tracer::ephemeron`] or not at all).
    fn trace(&self, t: &mut Tracer);

    /// After marking: drop weak edges to what died.
    fn sweep_weak(&mut self, _live: &Liveness) {}
}

macro_rules! no_handles {
    ($($t:ty),*) => {$(
        impl Trace for $t {
            fn trace(&self, _t: &mut Tracer) {}
        }
    )*};
}
no_handles!(
    (),
    bool,
    u8,
    u16,
    u32,
    u64,
    usize,
    i32,
    i64,
    f64,
    String,
    Box<str>
);

impl Trace for Value {
    fn trace(&self, t: &mut Tracer) {
        t.value(*self);
    }
}

impl Trace for Object {
    fn trace(&self, t: &mut Tracer) {
        t.object(*self);
    }
}

impl Trace for JsString {
    fn trace(&self, t: &mut Tracer) {
        t.string(*self);
    }
}

impl Trace for Symbol {
    fn trace(&self, t: &mut Tracer) {
        t.symbol(*self);
    }
}

impl Trace for BigInt {
    fn trace(&self, t: &mut Tracer) {
        t.bigint(*self);
    }
}

impl Trace for EnvId {
    fn trace(&self, t: &mut Tracer) {
        t.env(*self);
    }
}

impl Trace for PropertyKey {
    fn trace(&self, t: &mut Tracer) {
        t.key(*self);
    }
}

impl<T: Trace> Trace for Option<T> {
    fn trace(&self, t: &mut Tracer) {
        if let Some(x) = self {
            x.trace(t);
        }
    }
    fn sweep_weak(&mut self, live: &Liveness) {
        if let Some(x) = self {
            x.sweep_weak(live);
        }
    }
}

impl<T: Trace + ?Sized> Trace for Box<T> {
    fn trace(&self, t: &mut Tracer) {
        (**self).trace(t);
    }
    fn sweep_weak(&mut self, live: &Liveness) {
        (**self).sweep_weak(live);
    }
}

impl<T: Trace> Trace for Arc<T> {
    fn trace(&self, t: &mut Tracer) {
        (**self).trace(t);
    }
}

impl<T: Trace> Trace for [T] {
    fn trace(&self, t: &mut Tracer) {
        for x in self {
            x.trace(t);
        }
    }
    fn sweep_weak(&mut self, live: &Liveness) {
        for x in self {
            x.sweep_weak(live);
        }
    }
}

impl<T: Trace> Trace for Vec<T> {
    fn trace(&self, t: &mut Tracer) {
        self.as_slice().trace(t);
    }
    fn sweep_weak(&mut self, live: &Liveness) {
        self.as_mut_slice().sweep_weak(live);
    }
}

impl<T: Trace> Trace for VecDeque<T> {
    fn trace(&self, t: &mut Tracer) {
        for x in self {
            x.trace(t);
        }
    }
    fn sweep_weak(&mut self, live: &Liveness) {
        for x in self {
            x.sweep_weak(live);
        }
    }
}

impl<A: Trace, B: Trace> Trace for (A, B) {
    fn trace(&self, t: &mut Tracer) {
        self.0.trace(t);
        self.1.trace(t);
    }
    fn sweep_weak(&mut self, live: &Liveness) {
        self.0.sweep_weak(live);
        self.1.sweep_weak(live);
    }
}

impl<A: Trace, B: Trace, C: Trace> Trace for (A, B, C) {
    fn trace(&self, t: &mut Tracer) {
        self.0.trace(t);
        self.1.trace(t);
        self.2.trace(t);
    }
}

impl<K: Trace, V: Trace, S> Trace for std::collections::HashMap<K, V, S> {
    fn trace(&self, t: &mut Tracer) {
        for (k, v) in self {
            k.trace(t);
            v.trace(t);
        }
    }
}

impl<K: Trace, S> Trace for std::collections::HashSet<K, S> {
    fn trace(&self, t: &mut Tracer) {
        for k in self {
            k.trace(t);
        }
    }
}

impl<T: Trace> Trace for std::cell::RefCell<T> {
    fn trace(&self, t: &mut Tracer) {
        self.borrow().trace(t);
    }
}
