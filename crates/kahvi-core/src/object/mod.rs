//! Objects: property keys, property storage, descriptors and object kinds
//! (ECMA-262 §6.1.7, §10).

pub mod array;
pub mod host_object;
pub mod internal;
pub mod namespace;
pub mod proxy;
pub(crate) mod shape;
pub mod typed_array;
pub mod weak;

use std::any::Any;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use shape::{ACCESSOR, Attrs, CONFIGURABLE, ENUMERABLE, MAX_SHAPE_KEYS, Shape, WRITABLE, key_bits};

use crate::gc::{Liveness, Trace, Tracer};

use crate::value::{BigInt, JsString, Object, PackedValue, Symbol, Value};

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
    /// `get`/`set`: a function object, or `None` for undefined (8 bytes
    /// each rather than a Value's 16, which keeps every property small).
    Accessor {
        get: Option<Object>,
        set: Option<Object>,
        enumerable: bool,
        configurable: bool,
    },
}

/// An accessor property's [[Get]] or [[Set]] from a descriptor's field
/// value: a function object, or undefined (`None`).
pub fn accessor_function(v: Option<Value>) -> Option<Object> {
    match v {
        Some(Value::Object(o)) => Some(o),
        _ => None,
    }
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
                get: Some(get.map_or(Value::Undefined, Value::Object)),
                set: Some(set.map_or(Value::Undefined, Value::Object)),
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
/// `None`). Other keys live either in a shared [`Shape`] that lists the
/// keys and attributes, with the object keeping only the values (most
/// objects), or, once the object outgrows shapes (more than
/// [`MAX_SHAPE_KEYS`] keys, a deleted or reconfigured key, an index key
/// outside the dense vector), in its own insertion-ordered map. The split
/// is invisible to callers.
#[derive(Debug, Default, Clone)]
pub struct PropertyMap {
    elements: Option<Box<Elements>>,
    named: Named,
}

enum Named {
    /// `slots` holds the value of each of the keys of `shape`; `None` is
    /// the empty shape.
    Shaped {
        shape: Option<Arc<Shape>>,
        slots: Slots,
    },
    Dict(Box<Dict>),
}

impl Default for Named {
    fn default() -> Self {
        Named::Shaped {
            shape: None,
            slots: Slots::EMPTY,
        }
    }
}

impl Drop for Named {
    fn drop(&mut self) {
        if let Named::Shaped { shape, slots } = self {
            // SAFETY: `slots` holds a slot per key of `shape`.
            unsafe { slots.free(shape_len(shape)) }
        }
    }
}

impl Clone for Named {
    fn clone(&self) -> Self {
        match self {
            Named::Shaped { shape, slots } => Named::Shaped {
                shape: shape.clone(),
                // SAFETY: `slots` holds a slot per key of `shape`.
                slots: unsafe { slots.clone(shape_len(shape)) },
            },
            Named::Dict(d) => Named::Dict(d.clone()),
        }
    }
}

impl std::fmt::Debug for Named {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Named::Shaped { shape, slots } => f
                .debug_struct("Shaped")
                .field("shape", shape)
                .field("slots", &shaped(shape, slots))
                .finish(),
            Named::Dict(d) => f.debug_tuple("Dict").field(d).finish(),
        }
    }
}

fn shape_len(shape: &Option<Arc<Shape>>) -> usize {
    shape.as_ref().map_or(0, |s| s.len())
}

/// The slots of a shaped object.
#[inline]
fn shaped<'a>(shape: &Option<Arc<Shape>>, slots: &'a Slots) -> &'a [Slot] {
    // SAFETY: a shaped object's `slots` holds a slot per key of its shape.
    unsafe { slots.as_slice(shape_len(shape)) }
}

/// The slots of a shaped object, to change.
#[inline]
fn shaped_mut<'a>(shape: &Option<Arc<Shape>>, slots: &'a mut Slots) -> &'a mut [Slot] {
    // SAFETY: a shaped object's `slots` holds a slot per key of its shape.
    unsafe { slots.as_mut_slice(shape_len(shape)) }
}

/// The values of a shaped object's keys, in the order of its shape's keys:
/// a single pointer, as the shape says how many there are and the
/// allocation's size follows from that ([`slot_capacity`]).
struct Slots(std::ptr::NonNull<Slot>);

// SAFETY: `Slots` owns its allocation like a `Box<[Slot]>`.
unsafe impl Send for Slots {}
unsafe impl Sync for Slots {}

/// How many slots are allocated for `len`: doubling while small, then
/// growing by half, up to [`MAX_SHAPE_KEYS`].
fn slot_capacity(len: usize) -> usize {
    let mut cap = 0;
    while cap < len {
        cap = if cap < 8 {
            (cap * 2).max(2)
        } else {
            cap + cap / 2
        };
    }
    cap.min(MAX_SHAPE_KEYS.max(len))
}

impl Slots {
    const EMPTY: Slots = Slots(std::ptr::NonNull::dangling());

    fn layout(cap: usize) -> std::alloc::Layout {
        std::alloc::Layout::array::<Slot>(cap).expect("at most MAX_SHAPE_KEYS slots")
    }

    /// # Safety
    /// `len` slots are initialized.
    unsafe fn as_slice(&self, len: usize) -> &[Slot] {
        unsafe { std::slice::from_raw_parts(self.0.as_ptr(), len) }
    }

    /// # Safety
    /// `len` slots are initialized.
    unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [Slot] {
        unsafe { std::slice::from_raw_parts_mut(self.0.as_ptr(), len) }
    }

    /// Reallocates for `new_cap` slots, keeping the first `keep`.
    ///
    /// # Safety
    /// The allocation is for `old_cap` slots and `keep` of them, at most
    /// both capacities, are initialized.
    unsafe fn resize(&mut self, old_cap: usize, new_cap: usize, keep: usize) {
        use std::alloc::{alloc, dealloc, handle_alloc_error};
        let new = if new_cap == 0 {
            std::ptr::NonNull::dangling()
        } else {
            let layout = Self::layout(new_cap);
            // SAFETY: the layout has a non-zero size.
            let p = unsafe { alloc(layout) } as *mut Slot;
            let Some(p) = std::ptr::NonNull::new(p) else {
                handle_alloc_error(layout)
            };
            // SAFETY: both allocations hold at least `keep` slots.
            unsafe { std::ptr::copy_nonoverlapping(self.0.as_ptr(), p.as_ptr(), keep) };
            p
        };
        if old_cap != 0 {
            // SAFETY: the old allocation was made with this layout.
            unsafe { dealloc(self.0.as_ptr() as *mut u8, Self::layout(old_cap)) };
        }
        self.0 = new;
    }

    /// Slot `i`.
    ///
    /// # Safety
    /// Slot `i` is initialized.
    #[inline]
    unsafe fn get(&self, i: usize) -> Slot {
        unsafe { *self.0.as_ptr().add(i) }
    }

    /// Slot `i`, to change.
    ///
    /// # Safety
    /// Slot `i` is initialized.
    #[inline]
    unsafe fn get_mut(&mut self, i: usize) -> &mut Slot {
        unsafe { &mut *self.0.as_ptr().add(i) }
    }

    /// Appends `slot` after the `len` slots.
    ///
    /// # Safety
    /// `len` slots are initialized (and the allocation is the one
    /// [`slot_capacity`] gives for `len`).
    unsafe fn push(&mut self, len: usize, slot: Slot) {
        let cap = slot_capacity(len);
        if len == cap {
            unsafe { self.resize(cap, slot_capacity(len + 1), len) };
        }
        // SAFETY: there is room for `len + 1` slots.
        unsafe { self.0.as_ptr().add(len).write(slot) };
    }

    /// Removes and returns the last of the `len` slots.
    ///
    /// # Safety
    /// `len` > 0 slots are initialized.
    unsafe fn pop(&mut self, len: usize) -> Slot {
        // SAFETY: slot `len - 1` is initialized.
        let slot = unsafe { self.0.as_ptr().add(len - 1).read() };
        let (cap, new_cap) = (slot_capacity(len), slot_capacity(len - 1));
        if new_cap != cap {
            unsafe { self.resize(cap, new_cap, len - 1) };
        }
        slot
    }

    /// Frees the allocation for `len` slots.
    ///
    /// # Safety
    /// `len` is the number of slots, and the slots are not used again.
    unsafe fn free(&mut self, len: usize) {
        unsafe { self.resize(slot_capacity(len), 0, 0) };
    }

    /// A copy of the `len` slots.
    ///
    /// # Safety
    /// `len` slots are initialized.
    unsafe fn clone(&self, len: usize) -> Slots {
        let mut out = Slots::EMPTY;
        // SAFETY: the copy is allocated for `len` and gets `len` slots.
        unsafe {
            out.resize(0, slot_capacity(len), 0);
            std::ptr::copy_nonoverlapping(self.0.as_ptr(), out.0.as_ptr(), len);
        }
        out
    }
}

/// A property's value in a shaped object, in 8 bytes: a data property's
/// value (a [`PackedValue`]) or an accessor's [`AccessorPair`]. Its
/// attributes, which say which it is, are in the shape.
#[derive(Debug, Clone, Copy)]
struct Slot(u64);

impl Slot {
    #[inline]
    fn data(value: Value) -> Slot {
        Slot(PackedValue::pack(value).bits())
    }
    #[inline]
    fn accessor(pair: AccessorPair) -> Slot {
        Slot(pair.0 as u64 | (pair.1 as u64) << 32)
    }
    /// The value of a data property's slot.
    #[inline]
    fn value(self) -> Value {
        PackedValue::from_bits(self.0).unpack()
    }
    /// The getter and setter of an accessor's slot.
    #[inline]
    fn pair(self) -> AccessorPair {
        AccessorPair(self.0 as u32, (self.0 >> 32) as u32)
    }
}

/// A data property's value, to replace.
pub struct ValueMut<'a>(ValueRef<'a>);

enum ValueRef<'a> {
    Value(&'a mut Value),
    Slot(&'a mut Slot),
}

impl ValueMut<'_> {
    /// Replaces the value.
    #[inline]
    pub fn set(self, value: Value) {
        match self.0 {
            ValueRef::Value(v) => *v = value,
            ValueRef::Slot(s) => *s = Slot::data(value),
        }
    }
}

/// An accessor's getter and setter object handles, [`NO_FUNCTION`] for
/// undefined (packed so that a slot stays the size of a value).
#[derive(Debug, Clone, Copy)]
struct AccessorPair(u32, u32);

const NO_FUNCTION: u32 = u32::MAX;

impl AccessorPair {
    fn new(get: Option<Object>, set: Option<Object>) -> Self {
        AccessorPair(
            get.map_or(NO_FUNCTION, |o| o.0),
            set.map_or(NO_FUNCTION, |o| o.0),
        )
    }
    fn get(self) -> Option<Object> {
        (self.0 != NO_FUNCTION).then_some(Object(self.0))
    }
    fn set(self) -> Option<Object> {
        (self.1 != NO_FUNCTION).then_some(Object(self.1))
    }
}

/// An object's own map of named properties.
///
/// A map with at most [`SMALL_MAP`] entries has no hash index: lookups scan
/// `entries`, which is faster at that size and saves an allocation.
#[derive(Debug, Default, Clone)]
struct Dict {
    entries: Vec<Option<(PropertyKey, Property)>>,
    /// Key to position in `entries`; `None` while `entries` has at most
    /// [`SMALL_MAP`] slots.
    index: Option<crate::hash::FxHashMap<PropertyKey, u32>>,
    removed: u32,
    /// How many index keys are in the map rather than in `elements`.
    sparse_indices: u32,
}

/// The dense elements of a map that has any (boxed, as most objects have
/// none).
#[derive(Debug, Default, Clone)]
struct Elements {
    /// Each element's [`Slot`], or [`HOLE`].
    items: Vec<Slot>,
    /// Each element's attributes ([`HOLE_ATTRS`] for a hole), once any
    /// element is other than a writable, enumerable, configurable data
    /// property; until then a slot is a value or [`HOLE`] (boxed, as few
    /// arrays have any such element).
    #[allow(clippy::box_collection)]
    attrs: Option<Box<Vec<Attrs>>>,
    /// How many of `items` are present.
    count: u32,
    /// The object's [[PrivateElements]] (kept here, as few objects have
    /// any, so that objects need no field of their own for them; boxed, as
    /// few arrays have any either).
    #[allow(clippy::box_collection)]
    private: Option<Box<Vec<crate::function::PrivateElement>>>,
}

/// A plain element's attributes.
const PLAIN: Attrs = WRITABLE | ENUMERABLE | CONFIGURABLE;
/// The attributes of a hole in [`Elements::attrs`].
const HOLE_ATTRS: Attrs = 0x80;
/// A hole's slot while elements have no attributes of their own (no value
/// packs to these bits).
const HOLE: Slot = Slot(PackedValue::EMPTY.bits());

impl Elements {
    fn len(&self) -> usize {
        self.items.len()
    }

    #[inline]
    fn is_hole(&self, i: usize) -> bool {
        match &self.attrs {
            None => self.items[i].0 == HOLE.0,
            Some(a) => a[i] == HOLE_ATTRS,
        }
    }

    #[inline]
    fn get(&self, i: usize) -> Option<Property> {
        let a = match &self.attrs {
            None => PLAIN,
            Some(a) => a[i],
        };
        (!self.is_hole(i)).then(|| join(a, self.items[i]))
    }

    /// The value of element `i` if it is a data property.
    #[inline]
    fn value(&self, i: usize) -> Option<Option<Value>> {
        match &self.attrs {
            None => (self.items[i].0 != HOLE.0).then(|| Some(self.items[i].value())),
            Some(a) => match a[i] {
                HOLE_ATTRS => None,
                a if a & ACCESSOR != 0 => Some(None),
                _ => Some(Some(self.items[i].value())),
            },
        }
    }

    /// The value of data element `i`, to replace it, and whether it is
    /// writable.
    #[inline]
    fn data_mut(&mut self, i: usize) -> Option<(ValueMut<'_>, bool)> {
        let a = match &self.attrs {
            None if self.items[i].0 == HOLE.0 => return None,
            None => PLAIN,
            Some(a) if a[i] == HOLE_ATTRS || a[i] & ACCESSOR != 0 => return None,
            Some(a) => a[i],
        };
        Some((
            ValueMut(ValueRef::Slot(&mut self.items[i])),
            a & WRITABLE != 0,
        ))
    }

    /// Makes element `i` `prop`, or a hole; keeps `count`.
    fn set(&mut self, i: usize, prop: Option<Property>) {
        let was = !self.is_hole(i);
        let (a, slot) = match prop {
            Some(p) => split(p),
            None => (HOLE_ATTRS, HOLE),
        };
        if a != PLAIN && a != HOLE_ATTRS && self.attrs.is_none() {
            let attrs = self
                .items
                .iter()
                .map(|s| if s.0 == HOLE.0 { HOLE_ATTRS } else { PLAIN })
                .collect();
            self.attrs = Some(Box::new(attrs));
        }
        self.items[i] = slot;
        if let Some(attrs) = &mut self.attrs {
            attrs[i] = a;
        }
        match (was, prop.is_some()) {
            (false, true) => self.count += 1,
            (true, false) => self.count -= 1,
            _ => {}
        }
    }

    /// Extends the elements to `len` with holes.
    fn grow(&mut self, len: usize) {
        self.items.resize(len, HOLE);
        if let Some(a) = &mut self.attrs {
            a.resize(len, HOLE_ATTRS);
        }
    }

    /// Drops the holes at the end.
    fn trim(&mut self) {
        while !self.items.is_empty() && self.is_hole(self.items.len() - 1) {
            self.items.pop();
            if let Some(a) = &mut self.attrs {
                a.pop();
            }
        }
        if self.items.is_empty() {
            self.attrs = None;
        }
    }

    fn reserve_exact(&mut self, n: usize) {
        self.items.reserve_exact(n);
        if let Some(a) = &mut self.attrs {
            a.reserve_exact(n);
        }
    }

    /// The present elements, by index.
    fn present(&self) -> impl Iterator<Item = (usize, Property)> + '_ {
        (0..self.len()).filter_map(|i| self.get(i).map(|p| (i, p)))
    }
}

/// How far past the end of the dense elements a new index may land and
/// still extend them (filling the gap with holes).
const DENSE_GAP: usize = 64;

/// The most entries a map keeps without a hash index.
const SMALL_MAP: usize = 8;

/// `prop` split into its shape attributes and its slot.
fn split(prop: Property) -> (Attrs, Slot) {
    match prop {
        Property::Data {
            value,
            writable,
            enumerable,
            configurable,
        } => (attrs(writable, enumerable, configurable), Slot::data(value)),
        Property::Accessor {
            get,
            set,
            enumerable,
            configurable,
        } => (
            ACCESSOR | attrs(false, enumerable, configurable),
            Slot::accessor(AccessorPair::new(get, set)),
        ),
    }
}

fn attrs(writable: bool, enumerable: bool, configurable: bool) -> Attrs {
    (writable as Attrs * WRITABLE)
        | (enumerable as Attrs * ENUMERABLE)
        | (configurable as Attrs * CONFIGURABLE)
}

/// The property a shape's attributes and a slot make up.
#[inline]
fn join(a: Attrs, slot: Slot) -> Property {
    if a & ACCESSOR != 0 {
        let pair = slot.pair();
        Property::Accessor {
            get: pair.get(),
            set: pair.set(),
            enumerable: a & ENUMERABLE != 0,
            configurable: a & CONFIGURABLE != 0,
        }
    } else {
        Property::Data {
            value: slot.value(),
            writable: a & WRITABLE != 0,
            enumerable: a & ENUMERABLE != 0,
            configurable: a & CONFIGURABLE != 0,
        }
    }
}

impl Dict {
    #[inline]
    fn get_hinted(&self, key: &PropertyKey, hint: Option<&AtomicU32>) -> Option<Property> {
        if let Some(hint) = hint {
            let h = hint.load(Ordering::Relaxed) as usize;
            if let Some(Some((k, p))) = h.checked_sub(1).and_then(|i| self.entries.get(i))
                && k == key
            {
                return Some(*p);
            }
            let i = self.find(key)?;
            hint.store(i as u32 + 1, Ordering::Relaxed);
            return self.entries[i].as_ref().map(|(_, p)| *p);
        }
        let i = self.find(key)?;
        self.entries[i].as_ref().map(|(_, p)| *p)
    }

    /// The position of a key in `entries`.
    fn find(&self, key: &PropertyKey) -> Option<usize> {
        if self.entries.len() <= SMALL_MAP {
            self.entries
                .iter()
                .position(|e| matches!(e, Some((k, _)) if k == key))
        } else {
            self.index
                .as_ref()
                .and_then(|m| m.get(key))
                .map(|&i| i as usize)
        }
    }

    fn insert(&mut self, key: PropertyKey, prop: Property) {
        match self.find(&key) {
            Some(i) => self.entries[i] = Some((key, prop)),
            None => {
                if key.as_index().is_some() {
                    self.sparse_indices += 1;
                }
                self.entries.push(Some((key, prop)));
                let len = self.entries.len();
                if len == SMALL_MAP + 1 {
                    self.rebuild_index();
                } else if len > SMALL_MAP + 1
                    && let Some(m) = &mut self.index
                {
                    m.insert(key, len as u32 - 1);
                }
            }
        }
    }

    fn remove(&mut self, key: &PropertyKey) -> Option<Property> {
        let i = self.find(key)?;
        if self.entries.len() > SMALL_MAP
            && let Some(m) = &mut self.index
        {
            m.remove(key);
        }
        if key.as_index().is_some() {
            self.sparse_indices -= 1;
        }
        let old = self.entries[i].take().map(|(_, p)| p);
        self.removed += 1;
        if self.removed > 16 && self.removed as usize * 2 > self.entries.len() {
            self.entries.retain(Option::is_some);
            self.removed = 0;
            self.rebuild_index();
        }
        old
    }

    /// Rebuilds `index` from `entries`, or drops it when the map is small.
    fn rebuild_index(&mut self) {
        if self.entries.len() <= SMALL_MAP {
            self.index = None;
            return;
        }
        let m = self.index.get_or_insert_with(Default::default);
        m.clear();
        for (i, e) in self.entries.iter().enumerate() {
            if let Some((k, _)) = e {
                m.insert(*k, i as u32);
            }
        }
    }
}

impl PropertyMap {
    fn private_elements(&self) -> &[crate::function::PrivateElement] {
        self.elements
            .as_deref()
            .and_then(|e| e.private.as_deref())
            .map_or(&[], |p| p.as_slice())
    }

    fn dense_len(&self) -> usize {
        self.elements.as_deref().map_or(0, Elements::len)
    }

    /// The dense elements (`dense_slot` found an index in them).
    fn dense(&self) -> &Elements {
        self.elements.as_deref().expect("a dense index")
    }

    fn dense_mut(&mut self) -> &mut Elements {
        self.elements.as_deref_mut().expect("a dense index")
    }

    fn dense_slot(&self, key: &PropertyKey) -> Option<usize> {
        match key {
            PropertyKey::Index(i) if (*i as usize) < self.dense_len() => Some(*i as usize),
            _ => None,
        }
    }

    /// The named property `key`, with `hint` (if any) remembering the
    /// position it was last found at (a property instruction's cache).
    fn get_named(&self, key: &PropertyKey, hint: Option<&AtomicU32>) -> Option<Property> {
        match &self.named {
            Named::Shaped { shape, slots } => {
                let shape = shape.as_ref()?;
                let i = shape_find_hinted(shape, key, hint)?;
                // SAFETY: `i` is a position in the shape.
                Some(join(shape.attrs(i), unsafe { slots.get(i) }))
            }
            Named::Dict(d) => d.get_hinted(key, hint),
        }
    }

    /// The value of the own property `key`, with `hint` (if any)
    /// remembering where a property instruction last found its key, updated
    /// when the key is found elsewhere: `None` when there is no such
    /// property, `Some(None)` for an accessor property. (Smaller than a
    /// [`Property`], for the paths that read values.)
    pub(crate) fn value_of(
        &self,
        key: &PropertyKey,
        hint: Option<&AtomicU32>,
    ) -> Option<Option<Value>> {
        if let Some(i) = self.dense_slot(key) {
            return self.dense().value(i);
        }
        let value = |p: &Property| match p {
            Property::Data { value, .. } => Some(*value),
            Property::Accessor { .. } => None,
        };
        match &self.named {
            Named::Shaped { shape, slots } => {
                let shape = shape.as_ref()?;
                let i = shape_find_hinted(shape, key, hint)?;
                // SAFETY: `i` is a position in the shape.
                Some((shape.attrs(i) & ACCESSOR == 0).then(|| unsafe { slots.get(i) }.value()))
            }
            Named::Dict(d) => d.get_hinted(key, hint).as_ref().map(value),
        }
    }

    /// The value and writability of the data property `key`.
    #[inline]
    fn named_data_mut(
        &mut self,
        key: &PropertyKey,
        hint: Option<&AtomicU32>,
    ) -> Option<(ValueMut<'_>, bool)> {
        match &mut self.named {
            Named::Shaped { shape, slots } => {
                let shape = shape.as_ref()?;
                let i = shape_find_hinted(shape, key, hint)?;
                let a = shape.attrs(i);
                if a & ACCESSOR != 0 {
                    return None;
                }
                // SAFETY: `i` is a position in the shape.
                let slot = unsafe { slots.get_mut(i) };
                Some((ValueMut(ValueRef::Slot(slot)), a & WRITABLE != 0))
            }
            Named::Dict(d) => {
                let i = find_hinted(
                    hint,
                    key,
                    &d.entries,
                    |e| e.as_ref().map_or(&NO_KEY, |(k, _)| k),
                    |k| d.find(k),
                )?;
                match &mut d.entries[i] {
                    Some((
                        _,
                        Property::Data {
                            value, writable, ..
                        },
                    )) => Some((ValueMut(ValueRef::Value(value)), *writable)),
                    _ => None,
                }
            }
        }
    }

    #[inline]
    pub fn get(&self, key: &PropertyKey) -> Option<Property> {
        if let Some(i) = self.dense_slot(key) {
            return self.dense().get(i);
        }
        self.get_named(key, None)
    }

    /// The value of the data property `key`, to replace it, and whether
    /// the property is writable.
    pub fn data_value_mut(&mut self, key: &PropertyKey) -> Option<(ValueMut<'_>, bool)> {
        self.data_value_mut_inner(key, None)
    }

    /// [`data_value_mut`](Self::data_value_mut) with a position hint (see
    /// [`value_of`](Self::value_of)).
    #[inline]
    pub(crate) fn data_value_mut_hinted(
        &mut self,
        key: &PropertyKey,
        hint: &AtomicU32,
    ) -> Option<(ValueMut<'_>, bool)> {
        self.data_value_mut_inner(key, Some(hint))
    }

    #[inline]
    fn data_value_mut_inner(
        &mut self,
        key: &PropertyKey,
        hint: Option<&AtomicU32>,
    ) -> Option<(ValueMut<'_>, bool)> {
        if let Some(i) = self.dense_slot(key) {
            return self.dense_mut().data_mut(i);
        }
        self.named_data_mut(key, hint)
    }

    pub fn contains(&self, key: &PropertyKey) -> bool {
        if let Some(i) = self.dense_slot(key) {
            return !self.dense().is_hole(i);
        }
        match &self.named {
            Named::Shaped { shape, .. } => {
                key.as_index().is_none()
                    && shape
                        .as_ref()
                        .is_some_and(|s| s.find(key_bits(key)).is_some())
            }
            Named::Dict(d) => d.find(key).is_some(),
        }
    }

    /// Makes room for `n` more dense elements.
    pub(crate) fn reserve_elements(&mut self, n: usize) {
        if n > 0 {
            let e = self.elements.get_or_insert_with(Default::default);
            e.reserve_exact(n);
        }
    }

    /// Inserts or replaces, keeping the original position on replacement.
    pub fn insert(&mut self, key: PropertyKey, prop: Property) {
        if let PropertyKey::Index(i) = key {
            let i = i as usize;
            if i < self.dense_len() {
                self.dense_mut().set(i, Some(prop));
                return;
            }
            if i <= self.dense_len() + DENSE_GAP
                && !matches!(&self.named, Named::Dict(d) if d.find(&key).is_some())
            {
                self.grow_elements(i + 1);
                self.dense_mut().set(i, Some(prop));
                return;
            }
        }
        if let Named::Shaped { shape, slots } = &mut self.named {
            let (a, slot) = split(prop);
            let found = match key {
                PropertyKey::Index(_) => None,
                _ => shape.as_ref().and_then(|s| s.find(key_bits(&key))),
            };
            let len = shape_len(shape);
            match found {
                Some(i) if shape.as_ref().expect("found in it").attrs(i) == a => {
                    shaped_mut(shape, slots)[i] = slot;
                    return;
                }
                Some(_) => {}
                None if key.as_index().is_none() && len < MAX_SHAPE_KEYS => {
                    let child = Shape::child(shape.as_ref(), key, a);
                    // SAFETY: `slots` holds a slot per key of `shape`, and
                    // gets the new key's next to them as `shape` becomes
                    // the child with that key.
                    unsafe { slots.push(len, slot) };
                    *shape = Some(child);
                    return;
                }
                None => {}
            }
            self.make_dict();
        }
        let Named::Dict(d) = &mut self.named else {
            unreachable!("made a dictionary above")
        };
        d.insert(key, prop);
    }

    /// Moves the named properties out of their shape into the object's own
    /// map.
    fn make_dict(&mut self) {
        let Named::Shaped { shape, slots } = &self.named else {
            return;
        };
        let mut d = Dict::default();
        let slots = shaped(shape, slots);
        if let Some(shape) = shape {
            d.entries.reserve_exact(slots.len() + 1);
            for (i, &slot) in slots.iter().enumerate() {
                d.entries
                    .push(Some((shape.key(i), join(shape.attrs(i), slot))));
            }
        }
        d.rebuild_index();
        self.named = Named::Dict(Box::new(d));
    }

    /// Extends the dense elements to `len`, moving any map entries for the
    /// newly covered indices into them.
    fn grow_elements(&mut self, len: usize) {
        let e = self.elements.get_or_insert_with(Default::default);
        let old = e.len();
        e.grow(len);
        if let Named::Dict(d) = &mut self.named
            && d.sparse_indices > 0
        {
            for i in old..len {
                if let Some(p) = d.remove(&PropertyKey::Index(i as u32)) {
                    let e = self.elements.as_mut().expect("grown elements");
                    e.set(i, Some(p));
                }
            }
        }
    }

    pub fn remove(&mut self, key: &PropertyKey) -> Option<Property> {
        if let Some(i) = self.dense_slot(key) {
            let e = self.elements.as_mut().expect("a dense index");
            let old = e.get(i);
            e.set(i, None);
            e.trim();
            if e.items.is_empty() && e.private.is_none() {
                self.elements = None;
            }
            return old;
        }
        if let Named::Shaped { shape, slots } = &mut self.named {
            let shape_ref = shape.as_ref()?;
            if key.as_index().is_some() {
                return None;
            }
            let i = shape_ref.find(key_bits(key))?;
            // The last key added goes back to the shape before it.
            let len = shape_ref.len();
            if i + 1 == len {
                let a = shape_ref.attrs(i);
                let parent = shape_ref.parent().cloned();
                // SAFETY: `slots` holds a slot per key of `shape`, and
                // gives up the last as `shape` becomes its parent.
                let slot = unsafe { slots.pop(len) };
                *shape = parent;
                return Some(join(a, slot));
            }
            self.make_dict();
        }
        let Named::Dict(d) = &mut self.named else {
            unreachable!("made a dictionary above")
        };
        d.remove(key)
    }

    pub fn len(&self) -> usize {
        self.elements.as_ref().map_or(0, |e| e.count as usize)
            + match &self.named {
                Named::Shaped { shape, .. } => shape_len(shape),
                Named::Dict(d) => d.entries.len() - d.removed as usize,
            }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The bytes this map holds outside its object's slot (for
    /// [`crate::gc::HeapStats`]; shapes, being shared, are not counted).
    pub(crate) fn heap_bytes(&self) -> usize {
        use std::mem::size_of;
        let elements = self.elements.as_ref().map_or(0, |e| {
            size_of::<Elements>()
                + e.items.capacity() * size_of::<Slot>()
                + e.attrs
                    .as_ref()
                    .map_or(0, |a| size_of::<Vec<Attrs>>() + a.capacity())
                + e.private.as_ref().map_or(0, |p| {
                    size_of::<Vec<crate::function::PrivateElement>>()
                        + p.capacity() * size_of::<crate::function::PrivateElement>()
                })
        });
        let named = match &self.named {
            Named::Shaped { shape, .. } => slot_capacity(shape_len(shape)) * size_of::<Slot>(),
            Named::Dict(d) => {
                size_of::<Dict>()
                    + d.entries.capacity() * size_of::<Option<(PropertyKey, Property)>>()
                    + d.index
                        .as_ref()
                        .map_or(0, |m| m.capacity() * (size_of::<(PropertyKey, u32)>() + 1))
            }
        };
        elements + named
    }

    /// Visits the named keys in creation order.
    fn named_keys(&self, mut f: impl FnMut(PropertyKey)) {
        match &self.named {
            Named::Shaped { shape, .. } => {
                for k in shape.iter().flat_map(|s| s.keys()) {
                    f(k);
                }
            }
            Named::Dict(d) => {
                for (k, _) in d.entries.iter().flatten() {
                    f(*k);
                }
            }
        }
    }

    /// Keys in OrdinaryOwnPropertyKeys order: array indices ascending, then
    /// strings in creation order, then symbols in creation order.
    pub fn keys_in_spec_order(&self) -> Vec<PropertyKey> {
        let mut keys: Vec<PropertyKey> = Vec::with_capacity(self.len());
        if let Some(e) = self.elements.as_deref() {
            keys.extend(
                (0..e.len())
                    .filter(|&i| !e.is_hole(i))
                    .map(|i| PropertyKey::Index(i as u32)),
            );
        }
        let mut indices: Vec<u32> = Vec::new();
        let mut strings = Vec::new();
        let mut symbols = Vec::new();
        self.named_keys(|k| match k {
            PropertyKey::Index(i) => indices.push(i),
            PropertyKey::String(_) => strings.push(k),
            PropertyKey::Symbol(_) => symbols.push(k),
        });
        indices.sort_unstable();
        keys.extend(indices.into_iter().map(PropertyKey::Index));
        keys.extend(strings);
        keys.extend(symbols);
        keys
    }

    /// Array-index keys at or above `from`, descending (for ArraySetLength).
    pub fn index_keys_from(&self, from: u32) -> Vec<u32> {
        let mut v: Vec<u32> = Vec::new();
        self.named_keys(|k| {
            if let Some(i) = k.as_index()
                && i >= from
            {
                v.push(i);
            }
        });
        v.sort_unstable_by(|a, b| b.cmp(a));
        if let Some(e) = self.elements.as_deref() {
            let dense_from = (from as usize).min(e.len());
            v.extend(
                (dense_from..e.len())
                    .rev()
                    .filter(|&i| !e.is_hole(i))
                    .map(|i| i as u32),
            );
        }
        v
    }
}

/// The position of the named `key` in `shape`, looked for first where
/// `hint` remembers it, which is updated when it is found elsewhere.
#[inline]
fn shape_find_hinted(shape: &Shape, key: &PropertyKey, hint: Option<&AtomicU32>) -> Option<usize> {
    if key.as_index().is_some() {
        return None;
    }
    let bits = key_bits(key);
    let Some(hint) = hint else {
        return shape.find(bits);
    };
    let h = hint.load(Ordering::Relaxed) as usize;
    if h > 0 && shape.key_is(h - 1, bits) {
        return Some(h - 1);
    }
    let i = shape.find(bits)?;
    hint.store(i as u32 + 1, Ordering::Relaxed);
    Some(i)
}

/// A key no property has (for a removed entry's place).
const NO_KEY: PropertyKey = PropertyKey::Index(u32::MAX);

/// The position of `key` in `items`, looked for first where `hint`
/// remembers it, which is updated when it is found elsewhere.
#[inline]
fn find_hinted<T>(
    hint: Option<&AtomicU32>,
    key: &PropertyKey,
    items: &[T],
    key_of: impl Fn(&T) -> &PropertyKey,
    find: impl FnOnce(&PropertyKey) -> Option<usize>,
) -> Option<usize> {
    let Some(hint) = hint else {
        return find(key);
    };
    let h = hint.load(Ordering::Relaxed) as usize;
    if let Some(e) = h.checked_sub(1).and_then(|i| items.get(i))
        && key_of(e) == key
    {
        return Some(h - 1);
    }
    let i = find(key)?;
    hint.store(i as u32 + 1, Ordering::Relaxed);
    Some(i)
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

impl ObjectKind {
    /// Bytes of the boxed internal-slot data, for heap statistics: the box
    /// itself, plus an ArrayBuffer's owned bytes.
    pub(crate) fn boxed_bytes(&self) -> usize {
        use std::mem::{size_of, size_of_val};
        match self {
            ObjectKind::Function(b) => size_of_val(&**b),
            ObjectKind::Builtin(b) => size_of_val(&**b),
            ObjectKind::BoundFunction(b) => size_of_val(&**b),
            ObjectKind::Arguments(Some(b)) => size_of_val(&**b),
            ObjectKind::ModuleNamespace(b) => size_of_val(&**b),
            ObjectKind::ArrayBuffer(b) => {
                size_of::<typed_array::ArrayBufferData>()
                    + match &b.data {
                        Some(typed_array::DataBlock::Owned(v)) => v.capacity(),
                        _ => 0,
                    }
            }
            ObjectKind::TypedArray(b) => size_of_val(&**b),
            ObjectKind::DataView(b) => size_of_val(&**b),
            ObjectKind::Native(b) => size_of_val(&**b),
            _ => 0,
        }
    }
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
    /// The properties, and the [[PrivateElements]] (see
    /// [`ObjectData::private_elements`]).
    pub properties: PropertyMap,
    pub kind: ObjectKind,
}

impl Trace for Property {
    fn trace(&self, t: &mut Tracer) {
        match self {
            Property::Data { value, .. } => t.value(*value),
            Property::Accessor { get, set, .. } => {
                get.trace(t);
                set.trace(t);
            }
        }
    }
}

impl Trace for PropertyMap {
    fn trace(&self, t: &mut Tracer) {
        if let Some(e) = self.elements.as_deref() {
            for (_, p) in e.present() {
                p.trace(t);
            }
        }
        self.private_elements().trace(t);
        match &self.named {
            Named::Shaped { shape, slots } => {
                for k in shape.iter().flat_map(|s| s.keys()) {
                    t.key(k);
                }
                if let Some(sh) = shape {
                    for (i, slot) in shaped(shape, slots).iter().enumerate() {
                        if sh.attrs(i) & ACCESSOR != 0 {
                            let pair = slot.pair();
                            pair.get().trace(t);
                            pair.set().trace(t);
                        } else {
                            t.value(slot.value());
                        }
                    }
                }
            }
            Named::Dict(d) => {
                for (k, p) in d.entries.iter().flatten() {
                    t.key(*k);
                    p.trace(t);
                }
            }
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
        self.kind.trace(t);
    }

    fn sweep_weak(&mut self, live: &Liveness) {
        self.kind.sweep_weak(live);
    }
}

impl ObjectData {
    /// [[PrivateElements]]
    pub fn private_elements(&self) -> &[crate::function::PrivateElement] {
        self.properties.private_elements()
    }

    pub fn private_elements_mut(&mut self) -> &mut [crate::function::PrivateElement] {
        self.properties
            .elements
            .as_deref_mut()
            .and_then(|e| e.private.as_deref_mut())
            .map_or(&mut [], |p| p.as_mut_slice())
    }

    /// Appends to [[PrivateElements]].
    pub fn push_private_element(&mut self, e: crate::function::PrivateElement) {
        self.properties
            .elements
            .get_or_insert_with(Default::default)
            .private
            .get_or_insert_with(Default::default)
            .push(e);
    }

    pub fn new(prototype: Option<Object>, kind: ObjectKind) -> ObjectData {
        ObjectData {
            prototype,
            extensible: true,
            properties: PropertyMap::default(),
            kind,
        }
    }
}

// Every object is a slot of this size in the heap, live or not.
const _: () = assert!(std::mem::size_of::<ObjectData>() <= 64);
// A shaped object's property value.
const _: () = assert!(std::mem::size_of::<Slot>() == 8);
// Every property (and array element) is one of these.
const _: () = assert!(std::mem::size_of::<Property>() <= 24);
