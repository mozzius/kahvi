//! Objects: property keys, property storage, descriptors and object kinds
//! (ECMA-262 §6.1.7, §10).

pub mod array;
pub mod host_object;
pub mod internal;
pub mod namespace;
pub mod proxy;
pub mod typed_array;
pub mod weak;

use std::any::Any;

use crate::gc::{Liveness, Trace, Tracer};

use crate::value::{BigInt, JsString, Object, Symbol, Value};

/// A property key. Canonical array indices (0 ..= 2^32 - 2) are always
/// `Index`, never `String`, so lookups agree however the key was written.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PropertyKey {
    Index(u32),
    String(JsString),
    Symbol(Symbol),
}

impl From<Symbol> for PropertyKey {
    fn from(s: Symbol) -> Self {
        PropertyKey::Symbol(s)
    }
}

impl PropertyKey {
    pub fn is_symbol(self) -> bool {
        matches!(self, PropertyKey::Symbol(_))
    }
    pub fn as_index(self) -> Option<u32> {
        match self {
            PropertyKey::Index(i) => Some(i),
            _ => None,
        }
    }
}

/// A stored property.
#[derive(Clone, Copy, Debug)]
pub enum Property {
    Data {
        value: Value,
        writable: bool,
        enumerable: bool,
        configurable: bool,
    },
    /// `get`/`set` are undefined or a function object.
    Accessor {
        get: Value,
        set: Value,
        enumerable: bool,
        configurable: bool,
    },
}

impl Property {
    pub fn enumerable(&self) -> bool {
        match self {
            Property::Data { enumerable, .. } | Property::Accessor { enumerable, .. } => {
                *enumerable
            }
        }
    }
    pub fn configurable(&self) -> bool {
        match self {
            Property::Data { configurable, .. } | Property::Accessor { configurable, .. } => {
                *configurable
            }
        }
    }

    pub fn to_descriptor(self) -> PropertyDescriptor {
        match self {
            Property::Data {
                value,
                writable,
                enumerable,
                configurable,
            } => PropertyDescriptor {
                value: Some(value),
                writable: Some(writable),
                enumerable: Some(enumerable),
                configurable: Some(configurable),
                ..Default::default()
            },
            Property::Accessor {
                get,
                set,
                enumerable,
                configurable,
            } => PropertyDescriptor {
                get: Some(get),
                set: Some(set),
                enumerable: Some(enumerable),
                configurable: Some(configurable),
                ..Default::default()
            },
        }
    }
}

/// The Property Descriptor specification type (§6.2.6): every field optional.
#[derive(Clone, Copy, Debug, Default)]
pub struct PropertyDescriptor {
    pub value: Option<Value>,
    pub writable: Option<bool>,
    pub get: Option<Value>,
    pub set: Option<Value>,
    pub enumerable: Option<bool>,
    pub configurable: Option<bool>,
}

impl PropertyDescriptor {
    /// A data descriptor with all four fields.
    pub fn data(value: Value, writable: bool, enumerable: bool, configurable: bool) -> Self {
        PropertyDescriptor {
            value: Some(value),
            writable: Some(writable),
            enumerable: Some(enumerable),
            configurable: Some(configurable),
            ..Default::default()
        }
    }

    /// An accessor descriptor with all four fields.
    pub fn accessor(get: Value, set: Value, enumerable: bool, configurable: bool) -> Self {
        PropertyDescriptor {
            get: Some(get),
            set: Some(set),
            enumerable: Some(enumerable),
            configurable: Some(configurable),
            ..Default::default()
        }
    }

    /// IsAccessorDescriptor
    pub fn is_accessor_descriptor(&self) -> bool {
        self.get.is_some() || self.set.is_some()
    }

    /// IsDataDescriptor
    pub fn is_data_descriptor(&self) -> bool {
        self.value.is_some() || self.writable.is_some()
    }

    /// IsGenericDescriptor
    pub fn is_generic_descriptor(&self) -> bool {
        !self.is_accessor_descriptor() && !self.is_data_descriptor()
    }
}

/// Own properties in creation order, with O(1) lookup.
///
/// Array-index keys `0..elements.len()` live in a dense vector (holes are
/// `None`); every other key, including larger indices, lives in the
/// insertion-ordered map. The split is invisible to callers.
///
/// A map with at most [`SMALL_MAP`] entries has no hash index: lookups scan
/// `entries`, which is faster at that size and saves an allocation per
/// object.
#[derive(Debug, Default, Clone)]
pub struct PropertyMap {
    elements: Vec<Option<Property>>,
    /// How many of `elements` are present.
    element_count: usize,
    /// Key to position in `entries`; empty while `entries` has at most
    /// [`SMALL_MAP`] slots.
    index: crate::hash::FxHashMap<PropertyKey, usize>,
    entries: Vec<Option<(PropertyKey, Property)>>,
    removed: usize,
    /// How many index keys are in the map rather than in `elements`.
    sparse_indices: usize,
}

/// How far past the end of the dense elements a new index may land and
/// still extend them (filling the gap with holes).
const DENSE_GAP: usize = 64;

/// The most entries a map keeps without a hash index.
const SMALL_MAP: usize = 8;

impl PropertyMap {
    /// The position of a map key in `entries`.
    fn find(&self, key: &PropertyKey) -> Option<usize> {
        if self.entries.len() <= SMALL_MAP {
            self.entries
                .iter()
                .position(|e| matches!(e, Some((k, _)) if k == key))
        } else {
            self.index.get(key).copied()
        }
    }

    fn dense_slot(&self, key: &PropertyKey) -> Option<usize> {
        match key {
            PropertyKey::Index(i) if (*i as usize) < self.elements.len() => Some(*i as usize),
            _ => None,
        }
    }

    pub fn get(&self, key: &PropertyKey) -> Option<&Property> {
        if let Some(i) = self.dense_slot(key) {
            return self.elements[i].as_ref();
        }
        self.find(key)
            .and_then(|i| self.entries[i].as_ref().map(|(_, p)| p))
    }

    pub fn get_mut(&mut self, key: &PropertyKey) -> Option<&mut Property> {
        if let Some(i) = self.dense_slot(key) {
            return self.elements[i].as_mut();
        }
        let i = self.find(key)?;
        self.entries[i].as_mut().map(|(_, p)| p)
    }

    pub fn contains(&self, key: &PropertyKey) -> bool {
        if let Some(i) = self.dense_slot(key) {
            return self.elements[i].is_some();
        }
        self.find(key).is_some()
    }

    /// Inserts or replaces, keeping the original position on replacement.
    pub fn insert(&mut self, key: PropertyKey, prop: Property) {
        if let PropertyKey::Index(i) = key {
            let i = i as usize;
            if i < self.elements.len() {
                if self.elements[i].is_none() {
                    self.element_count += 1;
                }
                self.elements[i] = Some(prop);
                return;
            }
            if i <= self.elements.len() + DENSE_GAP && self.find(&key).is_none() {
                self.grow_elements(i + 1);
                self.elements[i] = Some(prop);
                self.element_count += 1;
                return;
            }
        }
        match self.find(&key) {
            Some(i) => self.entries[i] = Some((key, prop)),
            None => {
                if key.as_index().is_some() {
                    self.sparse_indices += 1;
                }
                if self.entries.capacity() == 0 {
                    self.entries.reserve_exact(2);
                }
                self.entries.push(Some((key, prop)));
                let len = self.entries.len();
                if len == SMALL_MAP + 1 {
                    self.rebuild_index();
                } else if len > SMALL_MAP + 1 {
                    self.index.insert(key, len - 1);
                }
            }
        }
    }

    /// Extends the dense elements to `len`, moving any map entries for the
    /// newly covered indices into them.
    fn grow_elements(&mut self, len: usize) {
        let old = self.elements.len();
        self.elements.resize(len, None);
        if self.sparse_indices > 0 {
            for i in old..len {
                if let Some(p) = self.remove_from_map(&PropertyKey::Index(i as u32)) {
                    self.elements[i] = Some(p);
                    self.element_count += 1;
                }
            }
        }
    }

    pub fn remove(&mut self, key: &PropertyKey) -> Option<Property> {
        if let Some(i) = self.dense_slot(key) {
            let old = self.elements[i].take();
            if old.is_some() {
                self.element_count -= 1;
            }
            while matches!(self.elements.last(), Some(None)) {
                self.elements.pop();
            }
            return old;
        }
        self.remove_from_map(key)
    }

    fn remove_from_map(&mut self, key: &PropertyKey) -> Option<Property> {
        let i = self.find(key)?;
        if self.entries.len() > SMALL_MAP {
            self.index.remove(key);
        }
        if key.as_index().is_some() {
            self.sparse_indices -= 1;
        }
        let old = self.entries[i].take().map(|(_, p)| p);
        self.removed += 1;
        if self.removed > 16 && self.removed * 2 > self.entries.len() {
            self.compact();
        }
        old
    }

    fn compact(&mut self) {
        self.entries.retain(Option::is_some);
        self.removed = 0;
        self.rebuild_index();
    }

    /// Rebuilds `index` from `entries`, or drops it when the map is small.
    fn rebuild_index(&mut self) {
        self.index.clear();
        if self.entries.len() <= SMALL_MAP {
            self.index.shrink_to_fit();
            return;
        }
        for (i, e) in self.entries.iter().enumerate() {
            if let Some((k, _)) = e {
                self.index.insert(*k, i);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.element_count + self.entries.len() - self.removed
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Keys in OrdinaryOwnPropertyKeys order: array indices ascending, then
    /// strings in creation order, then symbols in creation order.
    pub fn keys_in_spec_order(&self) -> Vec<PropertyKey> {
        let mut keys: Vec<PropertyKey> = Vec::with_capacity(self.len());
        keys.extend(
            self.elements
                .iter()
                .enumerate()
                .filter(|(_, p)| p.is_some())
                .map(|(i, _)| PropertyKey::Index(i as u32)),
        );
        let mut indices: Vec<u32> = Vec::new();
        let mut strings = Vec::new();
        let mut symbols = Vec::new();
        for (k, _) in self.entries.iter().flatten() {
            match k {
                PropertyKey::Index(i) => indices.push(*i),
                PropertyKey::String(_) => strings.push(*k),
                PropertyKey::Symbol(_) => symbols.push(*k),
            }
        }
        indices.sort_unstable();
        keys.extend(indices.into_iter().map(PropertyKey::Index));
        keys.extend(strings);
        keys.extend(symbols);
        keys
    }

    /// Array-index keys at or above `from`, descending (for ArraySetLength).
    pub fn index_keys_from(&self, from: u32) -> Vec<u32> {
        let mut v: Vec<u32> = self
            .entries
            .iter()
            .flatten()
            .filter_map(|(k, _)| k.as_index())
            .filter(|&i| i >= from)
            .collect();
        v.sort_unstable_by(|a, b| b.cmp(a));
        let dense_from = (from as usize).min(self.elements.len());
        v.extend(
            (dense_from..self.elements.len())
                .rev()
                .filter(|&i| self.elements[i].is_some())
                .map(|i| i as u32),
        );
        v
    }
}

/// What kind of object this is: which internal slots it has and which
/// internal methods are exotic.
#[derive(Debug)]
pub enum ObjectKind {
    Ordinary,
    /// Array exotic object (§10.4.2); `length` is an ordinary data property.
    Array,
    /// An ECMAScript function object (§10.2).
    Function(Box<crate::function::EcmaFunction>),
    /// A built-in function object (§10.3).
    Builtin(Box<crate::function::BuiltinFunction>),
    /// Bound function exotic object (§10.4.1).
    BoundFunction(Box<crate::function::BoundFunction>),
    /// Arguments exotic object (§10.4.4); `None` for unmapped arguments.
    Arguments(Option<Box<crate::function::MappedArguments>>),
    /// Module namespace exotic object (§10.4.6).
    ModuleNamespace(Box<namespace::NamespaceData>),
    /// Has [[ArrayBufferData]] (§25.1).
    ArrayBuffer(Box<typed_array::ArrayBufferData>),
    /// TypedArray exotic object (§10.4.5).
    TypedArray(Box<typed_array::TypedArrayData>),
    /// Has [[DataView]] (§25.3).
    DataView(Box<typed_array::DataViewData>),
    /// Has an [[ErrorData]] slot.
    Error,
    /// [[BooleanData]], [[NumberData]], [[StringData]] (String exotic, §10.4.3),
    /// [[SymbolData]], [[BigIntData]] wrappers.
    BooleanObject(bool),
    NumberObject(f64),
    StringObject(JsString),
    SymbolObject(Symbol),
    BigIntObject(BigInt),
    /// Immutable prototype exotic object (%Object.prototype%, §10.4.7).
    ImmutablePrototype,
    /// Internal slots of other built-in objects (Map, Set, Date, Promise…),
    /// defined next to the builtin that uses them. Downcast with
    /// [`crate::Agent::native`]/[`crate::Agent::native_mut`].
    Native(Box<dyn NativeData>),
}

/// Data for built-in objects whose internal methods are all ordinary. It
/// must report its handles to the garbage collector ([`Trace`]).
pub trait NativeData: Any + std::fmt::Debug + Send + Trace {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any + std::fmt::Debug + Send + Trace> NativeData for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct ObjectData {
    /// [[Prototype]]
    pub prototype: Option<Object>,
    /// [[Extensible]]
    pub extensible: bool,
    pub properties: PropertyMap,
    /// [[PrivateElements]]
    pub private_elements: Vec<crate::function::PrivateElement>,
    pub kind: ObjectKind,
}

impl Trace for Property {
    fn trace(&self, t: &mut Tracer) {
        match self {
            Property::Data { value, .. } => t.value(*value),
            Property::Accessor { get, set, .. } => {
                t.value(*get);
                t.value(*set);
            }
        }
    }
}

impl Trace for PropertyMap {
    fn trace(&self, t: &mut Tracer) {
        for p in self.elements.iter().flatten() {
            p.trace(t);
        }
        for (k, p) in self.entries.iter().flatten() {
            t.key(*k);
            p.trace(t);
        }
    }
}

impl Trace for ObjectKind {
    fn trace(&self, t: &mut Tracer) {
        match self {
            ObjectKind::Ordinary
            | ObjectKind::Array
            | ObjectKind::Error
            | ObjectKind::BooleanObject(_)
            | ObjectKind::NumberObject(_)
            | ObjectKind::ImmutablePrototype => {}
            ObjectKind::Function(f) => f.trace(t),
            ObjectKind::Builtin(f) => f.trace(t),
            ObjectKind::BoundFunction(f) => f.trace(t),
            ObjectKind::Arguments(a) => a.trace(t),
            ObjectKind::ModuleNamespace(n) => n.exports.trace(t),
            ObjectKind::ArrayBuffer(b) => t.value(b.detach_key),
            ObjectKind::TypedArray(a) => a.buffer.trace(t),
            ObjectKind::DataView(v) => t.object(v.buffer),
            ObjectKind::StringObject(s) => t.string(*s),
            ObjectKind::SymbolObject(s) => t.symbol(*s),
            ObjectKind::BigIntObject(b) => t.bigint(*b),
            ObjectKind::Native(d) => d.trace(t),
        }
    }

    fn sweep_weak(&mut self, live: &Liveness) {
        if let ObjectKind::Native(d) = self {
            d.sweep_weak(live);
        }
    }
}

impl Trace for ObjectData {
    fn trace(&self, t: &mut Tracer) {
        self.prototype.trace(t);
        self.properties.trace(t);
        self.private_elements.trace(t);
        self.kind.trace(t);
    }

    fn sweep_weak(&mut self, live: &Liveness) {
        self.kind.sweep_weak(live);
    }
}

impl ObjectData {
    pub fn new(prototype: Option<Object>, kind: ObjectKind) -> ObjectData {
        ObjectData {
            prototype,
            extensible: true,
            properties: PropertyMap::default(),
            private_elements: Vec::new(),
            kind,
        }
    }
}
