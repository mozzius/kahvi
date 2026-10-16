//! Shapes: the named-property layouts objects share.
//!
//! Objects given the same string and symbol keys in the same order share
//! one [`Shape`], which lists the keys once; each object keeps only the
//! properties, at the keys' positions. A shape never changes: adding a key
//! moves the object to a child shape, found again through the parent's
//! transitions so every object built the same way ends up sharing it.
//!
//! A shape's id names that layout for as long as the process runs (ids are
//! never reused), so a cache may remember "objects of shape N have key K at
//! position P" without holding the shape.
//!
//! Shapes are plain sequences of key handles and know nothing of the agent:
//! a handle in a shape is kept alive by the objects using the shape (their
//! tracing reports it), and a shape no object uses, itself or through a
//! longer shape extending it, is freed.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, LazyLock, Mutex, OnceLock, Weak};

use super::PropertyKey;
use crate::hash::FxHashMap;

/// The most keys a shape holds; an object given more keeps its own map.
/// (Shapes are looked up by scanning, and a chain of them copies its keys
/// once per shape, so they stay small.)
pub(crate) const MAX_SHAPE_KEYS: usize = 32;

/// The id of the empty shape, which objects with no named properties have.
pub(crate) const EMPTY_SHAPE_ID: u64 = 1;

static NEXT_ID: AtomicU64 = AtomicU64::new(EMPTY_SHAPE_ID + 1);

/// The empty shape's transitions (the empty shape itself is no allocation).
static ROOT: LazyLock<Transitions> = LazyLock::new(Transitions::default);

pub(crate) struct Shape {
    pub(crate) id: u64,
    /// String and symbol keys, in the order they were added.
    pub(crate) keys: Box<[PropertyKey]>,
    /// The shape this one extends, kept alive so that objects built the
    /// same way later find it again (parents only hold their children
    /// weakly).
    _parent: Option<Arc<Shape>>,
    transitions: Transitions,
}

/// The shapes one key further on, by that key. Most shapes only ever get
/// one child, which is found without taking the lock.
#[derive(Default)]
struct Transitions {
    first: OnceLock<(PropertyKey, Weak<Shape>)>,
    rest: Mutex<TransitionMap>,
}

#[derive(Default)]
struct TransitionMap {
    children: FxHashMap<PropertyKey, Weak<Shape>>,
    /// Dead children are dropped from `children` when it reaches this size.
    prune_at: usize,
}

impl std::fmt::Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Shape")
            .field("id", &self.id)
            .field("keys", &self.keys)
            .finish()
    }
}

impl Shape {
    /// The position of `key` in `keys`.
    #[inline]
    pub(crate) fn find(&self, key: &PropertyKey) -> Option<usize> {
        self.keys.iter().position(|k| k == key)
    }

    /// The shape of `parent` (`None` for the empty shape) plus `key`, which
    /// must be a string or symbol key `parent` lacks.
    pub(crate) fn child(parent: Option<&Arc<Shape>>, key: PropertyKey) -> Arc<Shape> {
        debug_assert!(key.as_index().is_none());
        let transitions = match parent {
            Some(p) => &p.transitions,
            None => &*ROOT,
        };
        if let Some((k, w)) = transitions.first.get()
            && *k == key
            && let Some(s) = w.upgrade()
        {
            return s;
        }
        let mut t = transitions.rest.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(s) = t.children.get(&key).and_then(Weak::upgrade) {
            return s;
        }
        let parent_keys: &[PropertyKey] = parent.map_or(&[], |p| &p.keys);
        let mut keys = Vec::with_capacity(parent_keys.len() + 1);
        keys.extend_from_slice(parent_keys);
        keys.push(key);
        let shape = Arc::new(Shape {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            keys: keys.into_boxed_slice(),
            _parent: parent.cloned(),
            transitions: Transitions::default(),
        });
        if transitions
            .first
            .set((key, Arc::downgrade(&shape)))
            .is_ok()
        {
            return shape;
        }
        t.children.insert(key, Arc::downgrade(&shape));
        if t.children.len() >= t.prune_at {
            t.children.retain(|_, w| w.strong_count() > 0);
            t.prune_at = (t.children.len() * 2).max(8);
        }
        shape
    }
}
