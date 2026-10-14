//! Storage for Map, Set, WeakMap and WeakSet (§24.1-24.4) and their iterators.
//!
//! [[MapData]] / [[SetData]] are lists whose deleted entries become empty
//! rather than being removed, so iterators created before a deletion keep
//! their position. [`OrderedTable`] does the same, with a hash index on
//! SameValueZero keys.
//!
//! The Map built-ins use this module; the Set, WeakMap and WeakSet helpers
//! are here for their tickets, hence the dead_code allowance.
#![allow(dead_code)]

use std::collections::HashMap;

use kahvi_core::object::ObjectKind;
use kahvi_core::ops::bigint::BigIntValue;
use kahvi_core::prelude::*;

/// A key compared with SameValueZero, made hashable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum KeyRepr {
    Undefined,
    Null,
    Boolean(bool),
    /// The bits of the number, with -0 as +0 and every NaN the same.
    Number(u64),
    String(JsString),
    Symbol(Symbol),
    BigInt(BigIntValue),
    Object(Object),
}

fn key_repr(agent: &Agent, v: Value) -> KeyRepr {
    match v {
        Value::Undefined => KeyRepr::Undefined,
        Value::Null => KeyRepr::Null,
        Value::Boolean(b) => KeyRepr::Boolean(b),
        Value::Number(n) if n.is_nan() => KeyRepr::Number(f64::NAN.to_bits()),
        // Adding +0 turns -0 into +0 and leaves every other number unchanged.
        Value::Number(n) => KeyRepr::Number((n + 0.0).to_bits()),
        Value::String(s) => KeyRepr::String(s),
        Value::Symbol(s) => KeyRepr::Symbol(s),
        Value::BigInt(b) => KeyRepr::BigInt(agent.bigint(b).clone()),
        Value::Object(o) => KeyRepr::Object(o),
    }
}

/// An insertion-ordered list of (key, value) entries keyed by SameValueZero.
#[derive(Debug, Default)]
pub(crate) struct OrderedTable {
    /// `None` marks a deleted entry (its key and value are empty).
    entries: Vec<Option<(Value, Value)>>,
    index: HashMap<KeyRepr, usize>,
    size: usize,
}

/// A collection key: CanonicalizeKeyedCollectionKey(key) plus its hash form.
/// Make one with [`collection_key`] before touching a table (the table
/// lives inside the agent, so the key must be computed first).
#[derive(Debug, Clone)]
pub(crate) struct CollectionKey {
    repr: KeyRepr,
    value: Value,
}

/// CanonicalizeKeyedCollectionKey(key) (§24.5.1): -0 becomes +0.
pub(crate) fn collection_key(agent: &Agent, key: Value) -> CollectionKey {
    let value = match key {
        // 1. If key is -0𝔽, return +0𝔽.
        Value::Number(n) => Value::Number(n + 0.0),
        k => k,
    };
    CollectionKey {
        repr: key_repr(agent, value),
        value,
    }
}

impl OrderedTable {
    /// The value for `key`, if present.
    pub(crate) fn get(&self, key: &CollectionKey) -> Option<Value> {
        let i = *self.index.get(&key.repr)?;
        self.entries[i].map(|(_, v)| v)
    }

    pub(crate) fn has(&self, key: &CollectionKey) -> bool {
        self.index.contains_key(&key.repr)
    }

    /// Sets the value for `key`, appending a new entry if it is not present.
    pub(crate) fn set(&mut self, key: CollectionKey, value: Value) {
        match self.index.get(&key.repr) {
            Some(&i) => self.entries[i] = Some((key.value, value)),
            None => {
                self.index.insert(key.repr, self.entries.len());
                self.entries.push(Some((key.value, value)));
                self.size += 1;
            }
        }
    }

    /// Removes `key` (its entry becomes empty); returns whether it was present.
    pub(crate) fn delete(&mut self, key: &CollectionKey) -> bool {
        match self.index.remove(&key.repr) {
            Some(i) => {
                self.entries[i] = None;
                self.size -= 1;
                true
            }
            None => false,
        }
    }

    /// Empties every entry (existing iterators see no more entries).
    pub(crate) fn clear(&mut self) {
        for e in &mut self.entries {
            *e = None;
        }
        self.index.clear();
        self.size = 0;
    }

    /// The number of non-empty entries (the `size` getters).
    pub(crate) fn len(&self) -> usize {
        self.size
    }

    /// The number of entries including empty ones: iterate with
    /// `for i in 0..table.slots() { if let Some((k, v)) = table.entry(i) ... }`,
    /// re-reading `slots()` each time because callbacks may add entries.
    pub(crate) fn slots(&self) -> usize {
        self.entries.len()
    }

    /// The entry at position `i`, or None if it was deleted.
    pub(crate) fn entry(&self, i: usize) -> Option<(Value, Value)> {
        self.entries.get(i).copied().flatten()
    }
}

/// [[MapData]] of a Map; [[SetData]] of a Set uses the same table with
/// every value undefined.
#[derive(Debug, Default)]
pub(crate) struct MapData(pub(crate) OrderedTable);

#[derive(Debug, Default)]
pub(crate) struct SetData(pub(crate) OrderedTable);

/// [[WeakMapData]] / [[WeakSetData]]: keys are held weakly (the collector
/// removes entries whose key died; a WeakMap value lives as long as its key).
#[derive(Debug, Default)]
pub(crate) struct WeakMapData(pub(crate) OrderedTable);

#[derive(Debug, Default)]
pub(crate) struct WeakSetData(pub(crate) OrderedTable);

macro_rules! require_slot {
    ($name:ident, $ty:ty, $what:literal) => {
        /// RequireInternalSlot for this kind of collection: the object, or a TypeError.
        pub(crate) fn $name(agent: &mut Agent, v: Value) -> JsResult<Object> {
            match v {
                Value::Object(o) if agent.native::<$ty>(o).is_some() => Ok(o),
                _ => Err(agent.throw_type_error(concat!("receiver is not a ", $what))),
            }
        }
    };
}

require_slot!(this_map, MapData, "Map");
require_slot!(this_set, SetData, "Set");
require_slot!(this_weak_map, WeakMapData, "WeakMap");
require_slot!(this_weak_set, WeakSetData, "WeakSet");

/// The table of a Map (after `this_map`).
pub(crate) fn map_table(agent: &mut Agent, o: Object) -> &mut OrderedTable {
    &mut agent.native_mut::<MapData>(o).expect("[[MapData]]").0
}

/// The table of a Set (after `this_set`).
pub(crate) fn set_table(agent: &mut Agent, o: Object) -> &mut OrderedTable {
    &mut agent.native_mut::<SetData>(o).expect("[[SetData]]").0
}

pub(crate) fn weak_map_table(agent: &mut Agent, o: Object) -> &mut OrderedTable {
    &mut agent
        .native_mut::<WeakMapData>(o)
        .expect("[[WeakMapData]]")
        .0
}

pub(crate) fn weak_set_table(agent: &mut Agent, o: Object) -> &mut OrderedTable {
    &mut agent
        .native_mut::<WeakSetData>(o)
        .expect("[[WeakSetData]]")
        .0
}

/// A new collection object for a constructor: OrdinaryCreateFromConstructor
/// with the given slot type, e.g. `create_collection(agent, nt, "%Map.prototype%", MapData::default())`.
pub(crate) fn create_collection<T: std::fmt::Debug + Send + kahvi_core::gc::Trace + 'static>(
    agent: &mut Agent,
    new_target: Object,
    default_proto: &str,
    data: T,
) -> JsResult<Object> {
    ops::ordinary_create_from_constructor(
        agent,
        new_target,
        default_proto,
        ObjectKind::Native(Box::new(data)),
    )
}

/// AddEntriesFromIterable(target, iterable, adder) (§24.1.1.2)
pub(crate) fn add_entries_from_iterable(
    agent: &mut Agent,
    target: Object,
    iterable: Value,
    adder: Value,
) -> JsResult<Value> {
    // 1. Let iteratorRecord be ? GetIterator(iterable, sync).
    let mut record = ops::get_iterator(agent, iterable, ops::IteratorKind::Sync)?;
    // 2. Repeat,
    loop {
        // a. Let next be ? IteratorStepValue(iteratorRecord).
        // b. If next is done, return target.
        let Some(next) = ops::iterator_step_value(agent, &mut record)? else {
            return Ok(Value::Object(target));
        };
        // c. If next is not an Object, then
        let Value::Object(item) = next else {
            // i. Let error be ThrowCompletion(a newly created TypeError object).
            let error = agent.throw_type_error("iterator value is not an entry object");
            // ii. Return ? IteratorClose(iteratorRecord, error).
            return ops::iterator_close(agent, &record, Err(error));
        };
        // d-g. k = Get(next, "0"), v = Get(next, "1"), Call(adder, target, « k, v »),
        //      each IfAbruptCloseIterator.
        let step = (|| -> JsResult<()> {
            let k = ops::get(agent, item, PropertyKey::Index(0))?;
            let v = ops::get(agent, item, PropertyKey::Index(1))?;
            ops::call(agent, adder, Value::Object(target), &[k, v])?;
            Ok(())
        })();
        if let Err(e) = step {
            return ops::iterator_close(agent, &record, Err(e));
        }
    }
}

/// CanBeHeldWeakly(v) (§9.13)
pub(crate) fn can_be_held_weakly(agent: &Agent, v: Value) -> bool {
    match v {
        // 1. If v is an Object, return true.
        Value::Object(_) => true,
        // 2. If v is a Symbol and KeyForSymbol(v) is undefined, return true.
        Value::Symbol(s) => !agent.symbol_registry.iter().any(|(_, sym)| *sym == s),
        // 3. Return false.
        _ => false,
    }
}

/// What a collection iterator yields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CollectionIterKind {
    Key,
    Value,
    KeyValue,
}

/// The state of a Map Iterator or Set Iterator (CreateMapIterator,
/// CreateSetIterator): the collection (None once exhausted), the next
/// position, and what to yield.
#[derive(Debug)]
pub(crate) struct CollectionIterator {
    pub(crate) collection: Option<Object>,
    pub(crate) next_index: usize,
    pub(crate) kind: CollectionIterKind,
    /// A Set iterator (entries are keys only).
    pub(crate) is_set: bool,
}

/// CreateMapIterator(map, kind) / CreateSetIterator(set, kind): the iterator
/// object, with %MapIteratorPrototype% or %SetIteratorPrototype%.
pub(crate) fn create_collection_iterator(
    agent: &mut Agent,
    collection: Object,
    kind: CollectionIterKind,
    is_set: bool,
) -> Object {
    let proto = agent.intrinsic(if is_set {
        "%SetIteratorPrototype%"
    } else {
        "%MapIteratorPrototype%"
    });
    let state = CollectionIterator {
        collection: Some(collection),
        next_index: 0,
        kind,
        is_set,
    };
    agent.object_with_kind(Some(proto), ObjectKind::Native(Box::new(state)))
}

/// The next entry of a Map or Set iterator, advancing it: `None` when done.
pub(crate) fn collection_iterator_step(agent: &mut Agent, it: Object) -> Option<(Value, Value)> {
    loop {
        let state = agent.native::<CollectionIterator>(it)?;
        let (collection, i, is_set) = (state.collection?, state.next_index, state.is_set);
        let (slots, entry) = {
            let table = if is_set {
                &agent.native::<SetData>(collection)?.0
            } else {
                &agent.native::<MapData>(collection)?.0
            };
            (table.slots(), table.entry(i))
        };
        let state = agent.native_mut::<CollectionIterator>(it)?;
        if i >= slots {
            state.collection = None;
            return None;
        }
        state.next_index = i + 1;
        if let Some(e) = entry {
            return Some(e);
        }
    }
}

// ---- garbage collection ----------------------------------------------------

use kahvi_core::gc::{Liveness, Trace, Tracer};

impl OrderedTable {
    /// Removes the entries whose key died (the keys of a WeakMap or WeakSet
    /// are objects or symbols).
    fn remove_dead_keys(&mut self, live: &Liveness) {
        for i in 0..self.entries.len() {
            if let Some((k, _)) = self.entries[i]
                && !live.is_live(k)
            {
                let repr = match k {
                    Value::Object(o) => KeyRepr::Object(o),
                    Value::Symbol(s) => KeyRepr::Symbol(s),
                    _ => unreachable!("a weakly held key that is not an object or symbol"),
                };
                self.index.remove(&repr);
                self.entries[i] = None;
                self.size -= 1;
            }
        }
    }
}

impl Trace for OrderedTable {
    fn trace(&self, t: &mut Tracer) {
        for (k, v) in self.entries.iter().flatten() {
            t.value(*k);
            t.value(*v);
        }
    }
}

impl Trace for MapData {
    fn trace(&self, t: &mut Tracer) {
        self.0.trace(t);
    }
}

impl Trace for SetData {
    fn trace(&self, t: &mut Tracer) {
        self.0.trace(t);
    }
}

impl Trace for WeakMapData {
    fn trace(&self, t: &mut Tracer) {
        for (k, v) in self.0.entries.iter().flatten() {
            t.ephemeron(*k, *v);
        }
    }
    fn sweep_weak(&mut self, live: &Liveness) {
        self.0.remove_dead_keys(live);
    }
}

impl Trace for WeakSetData {
    fn trace(&self, _t: &mut Tracer) {}
    fn sweep_weak(&mut self, live: &Liveness) {
        self.0.remove_dead_keys(live);
    }
}

impl Trace for CollectionIterator {
    fn trace(&self, t: &mut Tracer) {
        self.collection.trace(t);
    }
}
