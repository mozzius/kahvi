//! Objects: property keys, property storage, descriptors and object kinds
//! (ECMA-262 §6.1.7, §10).

pub mod array;
pub mod internal;
pub mod namespace;
pub mod proxy;

use std::any::Any;

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
#[derive(Debug, Default, Clone)]
pub struct PropertyMap {
    elements: Vec<Option<Property>>,
    /// How many of `elements` are present.
    element_count: usize,
    index: crate::hash::FxHashMap<PropertyKey, usize>,
    entries: Vec<Option<(PropertyKey, Property)>>,
    removed: usize,
    /// How many index keys are in the map rather than in `elements`.
    sparse_indices: usize,
}

/// How far past the end of the dense elements a new index may land and
/// still extend them (filling the gap with holes).
const DENSE_GAP: usize = 64;

impl PropertyMap {
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
        self.index
            .get(key)
            .and_then(|&i| self.entries[i].as_ref().map(|(_, p)| p))
    }

    pub fn get_mut(&mut self, key: &PropertyKey) -> Option<&mut Property> {
        if let Some(i) = self.dense_slot(key) {
            return self.elements[i].as_mut();
        }
        let i = *self.index.get(key)?;
        self.entries[i].as_mut().map(|(_, p)| p)
    }

    pub fn contains(&self, key: &PropertyKey) -> bool {
        if let Some(i) = self.dense_slot(key) {
            return self.elements[i].is_some();
        }
        self.index.contains_key(key)
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
            if i <= self.elements.len() + DENSE_GAP && !self.index.contains_key(&key) {
                self.grow_elements(i + 1);
                self.elements[i] = Some(prop);
                self.element_count += 1;
                return;
            }
        }
        match self.index.get(&key) {
            Some(&i) => self.entries[i] = Some((key, prop)),
            None => {
                if key.as_index().is_some() {
                    self.sparse_indices += 1;
                }
                self.index.insert(key, self.entries.len());
                self.entries.push(Some((key, prop)));
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
        let i = self.index.remove(key)?;
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
        self.index.clear();
        for (i, (k, _)) in self.entries.iter().flatten().enumerate() {
            self.index.insert(*k, i);
        }
        self.removed = 0;
    }

    pub fn len(&self) -> usize {
        self.element_count + self.index.len()
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

/// Data for built-in objects whose internal methods are all ordinary.
pub trait NativeData: Any + std::fmt::Debug + Send {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any + std::fmt::Debug + Send> NativeData for T {
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
