//! Objects: property keys, property storage, descriptors and object kinds
//! (ECMA-262 §6.1.7, §10).

pub mod array;
pub mod internal;
pub mod proxy;

use std::any::Any;
use std::collections::HashMap;

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
#[derive(Debug, Default, Clone)]
pub struct PropertyMap {
    index: HashMap<PropertyKey, usize>,
    entries: Vec<Option<(PropertyKey, Property)>>,
    removed: usize,
}

impl PropertyMap {
    pub fn get(&self, key: &PropertyKey) -> Option<&Property> {
        self.index
            .get(key)
            .and_then(|&i| self.entries[i].as_ref().map(|(_, p)| p))
    }

    pub fn get_mut(&mut self, key: &PropertyKey) -> Option<&mut Property> {
        let i = *self.index.get(key)?;
        self.entries[i].as_mut().map(|(_, p)| p)
    }

    pub fn contains(&self, key: &PropertyKey) -> bool {
        self.index.contains_key(key)
    }

    /// Inserts or replaces, keeping the original position on replacement.
    pub fn insert(&mut self, key: PropertyKey, prop: Property) {
        match self.index.get(&key) {
            Some(&i) => self.entries[i] = Some((key, prop)),
            None => {
                self.index.insert(key, self.entries.len());
                self.entries.push(Some((key, prop)));
            }
        }
    }

    pub fn remove(&mut self, key: &PropertyKey) -> Option<Property> {
        let i = self.index.remove(key)?;
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
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// Keys in OrdinaryOwnPropertyKeys order: array indices ascending, then
    /// strings in creation order, then symbols in creation order.
    pub fn keys_in_spec_order(&self) -> Vec<PropertyKey> {
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
        indices
            .into_iter()
            .map(PropertyKey::Index)
            .chain(strings)
            .chain(symbols)
            .collect()
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
