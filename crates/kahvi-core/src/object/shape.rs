//! Shapes: the named-property layouts objects share.
//!
//! Objects given the same string and symbol keys, with the same
//! attributes, in the same order share one [`Shape`], which lists the keys
//! and attributes once; each object keeps only the values, at the keys'
//! positions. A shape never changes: adding a key moves the object to a
//! child shape, found again through the parent's transitions, so every
//! object built the same way ends up sharing it.
//!
//! Shapes are plain sequences of key handles and know nothing of the
//! agent, so they are shared by every agent in the process: a key handle
//! means whatever it means in the agent of the object using the shape, and
//! is kept alive there by that object's tracing. A shape no object uses,
//! itself or through a longer shape extending it, is freed.

use std::sync::{Arc, LazyLock, Mutex, OnceLock, Weak};

use super::PropertyKey;
use crate::hash::FxHashMap;

/// The most keys a shape holds; an object given more keeps its own map.
/// (A chain of shapes copies its keys once per shape, so they stay small.)
pub(crate) const MAX_SHAPE_KEYS: usize = 32;

/// The most keys looked up by scanning; longer shapes build an index.
const SCAN_KEYS: usize = 12;

/// A property's attributes, as stored in a shape.
pub(crate) type Attrs = u8;
pub(crate) const WRITABLE: Attrs = 1;
pub(crate) const ENUMERABLE: Attrs = 2;
pub(crate) const CONFIGURABLE: Attrs = 4;
/// An accessor property (else a data property).
pub(crate) const ACCESSOR: Attrs = 8;

/// The empty shape's transitions (the empty shape itself is no allocation).
static ROOT: LazyLock<Transitions> = LazyLock::new(Transitions::default);

pub(crate) struct Shape {
    /// String and symbol keys (as [`key_bits`]), in the order they were
    /// added.
    bits: Box<[u64]>,
    /// The attributes of each key.
    attrs: Box<[Attrs]>,
    /// Key bits to position, for shapes too long to scan, built on first
    /// use.
    index: OnceLock<FxHashMap<u64, u32>>,
    /// The shape this one extends, kept alive so that objects built the
    /// same way later find it again (parents only hold their children
    /// weakly).
    parent: Option<Arc<Shape>>,
    transitions: Transitions,
}

/// The shapes one key further on, by that key and its attributes. Most
/// shapes only ever get one child, which is found without taking the lock.
#[derive(Default)]
struct Transitions {
    first: OnceLock<((u64, Attrs), Weak<Shape>)>,
    rest: Mutex<TransitionMap>,
}

#[derive(Default)]
struct TransitionMap {
    children: FxHashMap<(u64, Attrs), Weak<Shape>>,
    /// Dead children are dropped from `children` when it reaches this size.
    prune_at: usize,
}

impl std::fmt::Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Shape")
            .field("keys", &self.keys().collect::<Vec<_>>())
            .field("attrs", &self.attrs)
            .finish()
    }
}

impl Shape {
    /// The shape this one extends by its last key (`None`: the empty
    /// shape).
    pub(crate) fn parent(&self) -> Option<&Arc<Shape>> {
        self.parent.as_ref()
    }

    /// The key at position `i`.
    #[inline]
    pub(crate) fn key(&self, i: usize) -> PropertyKey {
        key_of_bits(self.bits[i])
    }

    /// Whether the key at position `i` is `bits` (see [`key_bits`]).
    #[inline]
    pub(crate) fn key_is(&self, i: usize, bits: u64) -> bool {
        self.bits.get(i) == Some(&bits)
    }

    /// The attributes of the key at position `i`.
    #[inline]
    pub(crate) fn attrs(&self, i: usize) -> Attrs {
        self.attrs[i]
    }

    /// The keys in order.
    pub(crate) fn keys(&self) -> impl Iterator<Item = PropertyKey> + '_ {
        self.bits.iter().map(|&b| key_of_bits(b))
    }

    /// The position of the key `bits` (see [`key_bits`]).
    #[inline]
    pub(crate) fn find(&self, bits: u64) -> Option<usize> {
        if self.bits.len() <= SCAN_KEYS {
            return self.bits.iter().position(|&b| b == bits);
        }
        self.find_indexed(bits)
    }

    fn find_indexed(&self, bits: u64) -> Option<usize> {
        self.index
            .get_or_init(|| {
                self.bits
                    .iter()
                    .enumerate()
                    .map(|(i, &b)| (b, i as u32))
                    .collect()
            })
            .get(&bits)
            .map(|&i| i as usize)
    }

    /// The shape of `parent` (`None` for the empty shape) plus `key` with
    /// `attrs`; `key` must be a string or symbol key `parent` lacks.
    pub(crate) fn child(parent: Option<&Arc<Shape>>, key: PropertyKey, attrs: Attrs) -> Arc<Shape> {
        debug_assert!(key.as_index().is_none());
        let transitions = match parent {
            Some(p) => &p.transitions,
            None => &*ROOT,
        };
        let edge = (key_bits(&key), attrs);
        if let Some((e, w)) = transitions.first.get()
            && *e == edge
            && let Some(s) = w.upgrade()
        {
            return s;
        }
        let mut t = transitions.rest.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(s) = t.children.get(&edge).and_then(Weak::upgrade) {
            return s;
        }
        let (parent_bits, parent_attrs): (&[u64], &[Attrs]) =
            parent.map_or((&[], &[]), |p| (&p.bits, &p.attrs));
        let mut bits = Vec::with_capacity(parent_bits.len() + 1);
        bits.extend_from_slice(parent_bits);
        bits.push(edge.0);
        let mut all_attrs = Vec::with_capacity(parent_attrs.len() + 1);
        all_attrs.extend_from_slice(parent_attrs);
        all_attrs.push(attrs);
        let shape = Arc::new(Shape {
            bits: bits.into_boxed_slice(),
            attrs: all_attrs.into_boxed_slice(),
            index: OnceLock::new(),
            parent: parent.cloned(),
            transitions: Transitions::default(),
        });
        if transitions
            .first
            .set((edge, Arc::downgrade(&shape)))
            .is_ok()
        {
            return shape;
        }
        t.children.insert(edge, Arc::downgrade(&shape));
        if t.children.len() >= t.prune_at {
            t.children.retain(|_, w| w.strong_count() > 0);
            t.prune_at = (t.children.len() * 2).max(8);
        }
        shape
    }
}

/// A key as one number, for comparing keys in a single instruction.
#[inline]
pub(crate) fn key_bits(key: &PropertyKey) -> u64 {
    match key {
        PropertyKey::Index(i) => *i as u64,
        PropertyKey::String(s) => 1 << 32 | s.0 as u64,
        PropertyKey::Symbol(s) => 2 << 32 | s.0 as u64,
    }
}

fn key_of_bits(bits: u64) -> PropertyKey {
    let n = bits as u32;
    match bits >> 32 {
        0 => PropertyKey::Index(n),
        1 => PropertyKey::String(crate::value::JsString(n)),
        _ => PropertyKey::Symbol(crate::value::Symbol(n)),
    }
}
