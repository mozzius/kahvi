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
/// What the compiler has referred to so far in the unit being compiled.
#[derive(Default)]
pub(crate) struct CompileUnit {
    pub(crate) constants: Arc<crate::compiler::CodeConstants>,
    pub(crate) strings: Vec<u32>,
    pub(crate) bigints: Vec<u32>,
    /// Nesting depth of `with` bodies and direct-eval functions being
    /// compiled (no binding caches inside; see `FunctionCode::name_caches`).
    pub(crate) dynamic_scopes: u32,
}

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
    /// Compilation units whose constants are already marked.
    pub(crate) code_seen: crate::hash::FxHashSet<usize>,
    /// Script records some live code belongs to.
    pub(crate) scripts: BitSet,
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

    /// Marks the constants of `code`'s compilation unit (once per collection).
    pub fn code(&mut self, code: &crate::compiler::FunctionCode) {
        if self.code_seen.insert(Arc::as_ptr(&code.constants) as usize) {
            self.scripts.insert(code.script);
            for &v in code.constants.values() {
                self.value(v);
            }
            for lazy in code.constants.lazy_code() {
                self.code(&lazy);
            }
        }
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

// ---- the collector ---------------------------------------------------------

use crate::agent::Agent;
use crate::env::Environment;
use crate::object::{ObjectData, ObjectKind};
use crate::vm::Frame;

/// A running frame, registered for the duration of `run_frame` (docs/GC.md).
#[derive(Clone, Copy)]
pub(crate) struct FramePtr(pub(crate) *const Frame);

// SAFETY: frames are only read through these pointers by the collector,
// on the thread that runs the agent, while the frames are alive (each is
// unregistered before its `run_frame` returns).
unsafe impl Send for FramePtr {}

/// Collector state kept on the agent.
/// A census of an agent's heap ([`Agent::heap_stats`]). Byte counts are of
/// the engine's own tables, not of what the allocator keeps around them.
#[derive(Debug, Default, Clone, Copy)]
pub struct HeapStats {
    pub collections: u64,
    pub object_slots: usize,
    pub objects_live: usize,
    /// The object table's allocation, slots in use or not.
    pub object_slot_bytes: usize,
    /// Property storage of the live objects, outside their slots.
    pub property_bytes: usize,
    pub string_slots: usize,
    pub strings_live: usize,
    /// The live strings' UTF-16 contents.
    pub string_bytes: usize,
    pub env_slots: usize,
    pub envs_live: usize,
    pub env_slot_bytes: usize,
    /// The source texts of the loaded scripts and modules (each counted
    /// once, though another runtime may share it).
    pub source_bytes: usize,
    /// Compiled functions reachable from live function objects, and
    /// roughly the bytes of their code.
    pub code_functions: usize,
    pub code_bytes: usize,
}

pub(crate) struct GcState {
    pub(crate) free_objects: Vec<u32>,
    pub(crate) free_strings: Vec<u32>,
    pub(crate) free_symbols: Vec<u32>,
    pub(crate) free_bigints: Vec<u32>,
    pub(crate) free_envs: Vec<u32>,
    /// Slots currently on a free list.
    pub(crate) is_free_object: BitSet,
    pub(crate) is_free_string: BitSet,
    pub(crate) is_free_symbol: BitSet,
    pub(crate) is_free_bigint: BitSet,
    pub(crate) is_free_env: BitSet,
    /// The agent's common names and other strings and BigInts made during
    /// its setup: never collected.
    pub(crate) pinned_strings: BitSet,
    pub(crate) pinned_bigints: BitSet,
    /// Symbols below this index (the well-known symbols) are never collected.
    pub(crate) pinned_symbols: u32,
    /// While set, new and looked-up strings and BigInts are pinned (the
    /// agent's setup: common names).
    pub(crate) pinning: bool,
    /// The compilation unit being compiled: the strings and BigInts it
    /// creates or looks up become its constants.
    pub(crate) compiling: Option<CompileUnit>,
    /// Freed eval and dynamic-function script records, by (realm, name).
    pub(crate) free_scripts: std::collections::HashMap<(u32, String), Vec<u32>>,
    /// Allocations since the last collection, and the count that triggers one.
    pub(crate) allocated: usize,
    pub(crate) threshold: usize,
    /// Rust code above holds unrooted handles: no collection (docs/GC.md).
    pub(crate) holds: u32,
    pub(crate) frames: Vec<FramePtr>,
    /// Explicit roots (`Agent::push_root`).
    pub(crate) roots: Vec<Value>,
    pub(crate) collections: u64,
    /// Stressed safe points left before the next stress collection (0:
    /// not drawn yet), and the generator that draws it (see
    /// `Agent::gc_stress`).
    pub(crate) stress_left: std::cell::Cell<u32>,
    pub(crate) stress_rng: std::cell::Cell<u32>,
}

const MIN_THRESHOLD: usize = 200_000;

impl Default for GcState {
    fn default() -> GcState {
        GcState {
            free_objects: Vec::new(),
            free_strings: Vec::new(),
            free_symbols: Vec::new(),
            free_bigints: Vec::new(),
            free_envs: Vec::new(),
            is_free_object: BitSet::default(),
            is_free_string: BitSet::default(),
            is_free_symbol: BitSet::default(),
            is_free_bigint: BitSet::default(),
            is_free_env: BitSet::default(),
            pinned_strings: BitSet::default(),
            pinned_bigints: BitSet::default(),
            pinned_symbols: 0,
            pinning: false,
            compiling: None,
            free_scripts: Default::default(),
            allocated: 0,
            threshold: MIN_THRESHOLD,
            holds: 0,
            frames: Vec::new(),
            roots: Vec::new(),
            collections: 0,
            stress_left: std::cell::Cell::new(0),
            stress_rng: std::cell::Cell::new(0x9e37_79b9),
        }
    }
}

impl Agent {
    /// Whether a safe point should collect now.
    #[inline]
    pub(crate) fn gc_wanted(&self) -> bool {
        self.gc.holds == 0
            && (self.gc.allocated >= self.gc.threshold
                || (self.gc_stress != 0 && self.gc_stress_due()))
    }

    /// Counts down a stressed safe point; true when the countdown runs
    /// out. With `gc_stress` N > 1 the next countdown is drawn from
    /// 1..=2N-1 (mean N, fixed seed), so loops whose period divides N are
    /// not always interrupted at the same instruction.
    fn gc_stress_due(&self) -> bool {
        let left = self.gc.stress_left.get();
        if left > 1 {
            self.gc.stress_left.set(left - 1);
            return false;
        }
        let n = self.gc_stress;
        let next = if n <= 1 {
            1
        } else {
            // xorshift32
            let mut x = self.gc.stress_rng.get();
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            self.gc.stress_rng.set(x);
            1 + x % (2 * n - 1)
        };
        self.gc.stress_left.set(next);
        left == 1 || n <= 1
    }

    /// `KAHVI_GC_STRESS` as [`Agent::gc_stress`]: unset is off; a positive
    /// number N collects at one safe point in N on average (1: every one);
    /// any other value is 1.
    pub fn gc_stress_from_env() -> u32 {
        match std::env::var("KAHVI_GC_STRESS") {
            Err(std::env::VarError::NotPresent) => 0,
            Ok(v) => v.trim().parse::<u32>().ok().filter(|&n| n > 0).unwrap_or(1),
            Err(_) => 1,
        }
    }

    /// A safe point: collects if the heap has grown enough and nothing holds.
    #[inline]
    pub fn gc_safe_point(&mut self) {
        if self.gc_wanted() {
            self.collect_garbage();
        }
    }

    /// Roots `v` until `pop_roots` is called with the returned mark.
    pub fn push_root(&mut self, v: Value) -> usize {
        self.gc.roots.push(v);
        self.gc.roots.len() - 1
    }

    /// A mark for `pop_roots` that pushes nothing.
    pub fn gc_roots_len(&self) -> usize {
        self.gc.roots.len()
    }

    /// Drops the roots pushed since `mark` (a value returned by `push_root`).
    pub fn pop_roots(&mut self, mark: usize) {
        self.gc.roots.truncate(mark);
    }

    /// Roots a String or Symbol property key like `push_root` (an array
    /// index holds no handle and pushes nothing). Returns a mark for
    /// `pop_roots`.
    pub fn push_root_key(&mut self, key: PropertyKey) -> usize {
        match key {
            PropertyKey::Index(_) => self.gc.roots.len(),
            PropertyKey::String(s) => self.push_root(Value::String(s)),
            PropertyKey::Symbol(s) => self.push_root(Value::Symbol(s)),
        }
    }

    /// Runs `f` with `roots` rooted, and drops them (and anything `f`
    /// pushed and left behind, e.g. on an early `?` return) afterwards.
    /// For a built-in that calls JavaScript with `ops::call_rooted` so the
    /// callback can collect garbage: everything it uses after the call must
    /// be rooted (docs/GC.md).
    pub fn with_roots<T>(&mut self, roots: &[Value], f: impl FnOnce(&mut Agent) -> T) -> T {
        let mark = self.gc.roots.len();
        self.gc.roots.extend_from_slice(roots);
        let r = f(self);
        self.gc.roots.truncate(mark);
        r
    }

    /// Runs `f` with collection disabled (Rust code holding unrooted handles
    /// while JavaScript may run).
    #[inline]
    pub fn holding<T>(&mut self, f: impl FnOnce(&mut Agent) -> T) -> T {
        self.gc.holds += 1;
        let r = f(self);
        self.gc.holds -= 1;
        r
    }

    /// Collects now unless Rust code on the stack holds unrooted handles
    /// (a host's explicit request, e.g. `$262.gc()`).
    pub fn collect_garbage_if_safe(&mut self) {
        if self.gc.holds == 0 {
            self.collect_garbage();
        }
    }

    /// What the heap holds now, live and free (a walk of the whole heap;
    /// for hosts measuring memory).
    pub fn heap_stats(&self) -> HeapStats {
        use std::mem::size_of;
        let mut h = HeapStats {
            collections: self.gc.collections,
            object_slots: self.objects.len(),
            object_slot_bytes: self.objects.capacity() * size_of::<crate::object::ObjectData>(),
            string_slots: self.strings.len(),
            env_slots: self.envs.len(),
            env_slot_bytes: self.envs.capacity() * size_of::<crate::env::Environment>(),
            ..HeapStats::default()
        };
        let mut seen_code = crate::hash::FxHashSet::default();
        for (i, o) in self.objects.iter().enumerate() {
            if !self.gc.is_free_object.get(i as u32) {
                h.objects_live += 1;
                h.property_bytes += o.properties.heap_bytes();
                if let crate::object::ObjectKind::Function(f) = &o.kind {
                    f.code
                        .census(&mut seen_code, &mut h.code_bytes, &mut h.code_functions);
                }
            }
        }
        for (i, s) in self.strings.iter().enumerate() {
            if !self.gc.is_free_string.get(i as u32) {
                h.strings_live += 1;
                h.string_bytes += s.len() * 2;
            }
        }
        h.envs_live = (0..self.envs.len() as u32)
            .filter(|&i| !self.gc.is_free_env.get(i))
            .count();
        let mut sources: Vec<*const u8> = Vec::new();
        for s in &self.scripts {
            let p = s.source.as_ptr();
            if !sources.contains(&p) {
                sources.push(p);
                h.source_bytes += s.source.len();
            }
        }
        h
    }

    /// How many collections have run.
    pub fn gc_collections(&self) -> u64 {
        self.gc.collections
    }

    /// Collects garbage now. Only call this where nothing outside the roots
    /// holds handles (a safe point; see docs/GC.md).
    pub fn collect_garbage(&mut self) {
        self.persistent.release_dropped();
        let mut t = Tracer::default();
        self.trace_roots(&mut t);
        self.mark(&mut t);
        let live = self.sweep(&t);
        crate::object::weak::schedule_cleanups(self, &Liveness { tracer: &t });
        self.gc.collections += 1;
        self.gc.allocated = 0;
        self.gc.threshold = MIN_THRESHOLD.max(live);
    }

    fn trace_roots(&self, t: &mut Tracer) {
        for r in &self.realms {
            for &o in r.intrinsics.values() {
                t.object(o);
            }
            t.object(r.global_object);
            t.env(r.global_env);
            for &o in r.template_map.values() {
                t.object(o);
            }
            if let Some(s) = r.legacy_regexp.input {
                t.string(s);
            }
            if let Some(l) = &r.legacy_regexp.last_match {
                t.string(l.matched_in);
                t.string(l.last_paren);
                for &p in &l.parens {
                    t.string(p);
                }
            }
        }
        for c in &self.contexts {
            c.function.trace(t);
        }
        self.jobs.trace(t);
        self.running_jobs.trace(t);
        for &(s, sym) in &self.symbol_registry {
            t.string(s);
            t.symbol(sym);
        }
        for &(o, _) in &self.rejection_tracker {
            t.object(o);
        }
        for m in &self.modules {
            m.trace(t);
        }
        for f in &self.gc.frames {
            // SAFETY: see FramePtr.
            unsafe { (*f.0).trace(t) };
        }
        self.gc.roots.trace(t);
        for v in self.persistent.values() {
            t.value(v);
        }
        self.kept_objects.trace(t);
        self.kept_symbols.trace(t);
    }

    fn mark(&self, t: &mut Tracer) {
        loop {
            while let Some(i) = t.object_work.pop() {
                self.objects[i as usize].trace(t);
            }
            while let Some(i) = t.env_work.pop() {
                self.envs[i as usize].trace(t);
            }
            while let Some(i) = t.symbol_work.pop() {
                if let Some(d) = self.symbols[i as usize].description {
                    t.string(d);
                }
            }
            if !t.object_work.is_empty() || !t.env_work.is_empty() {
                continue;
            }
            // Ephemerons: a value is live once its key is (fixed point).
            let mut pending = std::mem::take(&mut t.ephemerons);
            let before = pending.len();
            pending.retain(|&(k, v)| {
                if t.is_live(k) {
                    t.value(v);
                    false
                } else {
                    true
                }
            });
            let progressed = pending.len() != before;
            t.ephemerons.append(&mut pending);
            if !progressed && t.object_work.is_empty() && t.env_work.is_empty() {
                break;
            }
        }
    }

    /// Frees everything unmarked; returns how many slots are live.
    fn sweep(&mut self, t: &Tracer) -> usize {
        let live = Liveness { tracer: t };
        let mut live_count = 0;
        for i in 0..self.objects.len() as u32 {
            if t.objects.get(i) {
                live_count += 1;
                self.objects[i as usize].sweep_weak(&live);
            } else if !self.gc.is_free_object.get(i) {
                self.objects[i as usize] = ObjectData::new(None, ObjectKind::Ordinary);
                self.gc.is_free_object.insert(i);
                self.gc.free_objects.push(i);
            }
        }
        // A collected object's native state goes with it.
        self.native_states.retain(|o, _| t.objects.get(o.0));
        self.error_stacks.retain(|o, _| t.objects.get(o.0));
        for i in 0..self.envs.len() as u32 {
            if t.envs.get(i) {
                live_count += 1;
            } else if !self.gc.is_free_env.get(i) {
                // A declarative record keeps its allocations for
                // `alloc_declarative`; a large one gives them back.
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
            }
        }
        for i in 0..self.strings.len() as u32 {
            if t.strings.get(i) || self.gc.pinned_strings.get(i) {
                live_count += 1;
            } else if !self.gc.is_free_string.get(i) {
                self.string_index.remove(&self.strings, i);
                self.strings[i as usize] = Box::default();
                self.gc.is_free_string.insert(i);
                self.gc.free_strings.push(i);
            }
        }
        for i in self.gc.pinned_symbols..self.symbols.len() as u32 {
            if t.symbols.get(i) {
                live_count += 1;
            } else if !self.gc.is_free_symbol.get(i) {
                self.symbols[i as usize].description = None;
                self.gc.is_free_symbol.insert(i);
                self.gc.free_symbols.push(i);
            }
        }
        for i in 0..self.bigints.len() as u32 {
            if t.bigints.get(i) || self.gc.pinned_bigints.get(i) {
                live_count += 1;
            } else if !self.gc.is_free_bigint.get(i) {
                self.bigints[i as usize] = crate::ops::bigint::BigIntValue::zero();
                self.gc.is_free_bigint.insert(i);
                self.gc.free_bigints.push(i);
            }
        }
        // Eval and dynamic-function script records no live code belongs to:
        // drop their source and template objects, and reuse the index.
        let mut freed = crate::hash::FxHashSet::default();
        for (i, r) in self.scripts.iter_mut().enumerate() {
            let i = i as u32;
            if r.collectable && !t.scripts.get(i) && !r.source.is_empty() {
                r.source = Arc::from("");
                freed.insert(i);
                self.gc
                    .free_scripts
                    .entry((r.realm.0, r.name.clone()))
                    .or_default()
                    .push(i);
            }
        }
        if !freed.is_empty() {
            for realm in &mut self.realms {
                realm
                    .template_map
                    .retain(|&(script, _), _| !freed.contains(&script));
            }
        }
        live_count
    }
}
