//! A fast hasher for maps keyed by engine handles (`JsString`, `Symbol`,
//! `PropertyKey`): small integers the engine hands out itself, so there is
//! nothing for an attacker to collide. Maps keyed by string contents keep
//! std's DoS-resistant hasher.

use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};

/// The rustc "Fx" hash: one multiply per word.
#[derive(Default, Clone, Copy)]
pub struct FxHasher {
    hash: u64,
}

const SEED: u64 = 0x51_7c_c1_b7_27_22_0a_95;

impl FxHasher {
    #[inline]
    fn add(&mut self, word: u64) {
        self.hash = (self.hash.rotate_left(5) ^ word).wrapping_mul(SEED);
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let mut chunks = bytes.chunks_exact(8);
        for c in &mut chunks {
            self.add(u64::from_le_bytes(c.try_into().expect("8 bytes")));
        }
        for &b in chunks.remainder() {
            self.add(b as u64);
        }
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add(i as u64);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.add(i as u64);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add(i);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.add(i as u64);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}

pub type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;
pub type FxHashSet<K> = HashSet<K, BuildHasherDefault<FxHasher>>;

/// A hasher for string contents: Fx over whole words, started from a seed
/// drawn once per process, so scripts cannot precompute colliding strings.
#[derive(Clone, Copy)]
pub struct SeededFxBuilder {
    seed: u64,
}

impl Default for SeededFxBuilder {
    fn default() -> Self {
        use std::hash::BuildHasher;
        static SEED: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
        let seed = *SEED.get_or_init(|| {
            std::collections::hash_map::RandomState::new().hash_one(0x006b_6168_7669_u64)
        });
        SeededFxBuilder { seed }
    }
}

impl std::hash::BuildHasher for SeededFxBuilder {
    type Hasher = FxHasher;
    fn build_hasher(&self) -> FxHasher {
        FxHasher { hash: self.seed }
    }
}

pub type ContentHashMap<K, V> = HashMap<K, V, SeededFxBuilder>;

/// The string intern table: a set of string handles hashed by the contents
/// they name in the agent's string arena. It stores only the `u32` handles,
/// so every interned string's code units are kept once (in the arena), and
/// lookups probe with the caller's slice without allocating.
///
/// Open addressing with linear probing over a power-of-two table. Removed
/// entries leave a tombstone; the table is rebuilt when live entries plus
/// tombstones pass three quarters of its capacity.
pub struct StringTable {
    slots: Vec<u32>,
    live: usize,
    tombstones: usize,
    seed: u64,
}

const EMPTY: u32 = u32::MAX;
const TOMBSTONE: u32 = u32::MAX - 1;

impl Default for StringTable {
    fn default() -> Self {
        use std::hash::BuildHasher;
        let seed = SeededFxBuilder::default().build_hasher().hash;
        StringTable {
            slots: Vec::new(),
            live: 0,
            tombstones: 0,
            seed,
        }
    }
}

impl StringTable {
    fn hash(&self, units: &[u16]) -> u64 {
        let mut h = FxHasher { hash: self.seed };
        let mut chunks = units.chunks_exact(4);
        for c in &mut chunks {
            h.add(c[0] as u64 | (c[1] as u64) << 16 | (c[2] as u64) << 32 | (c[3] as u64) << 48);
        }
        let mut last = 0u64;
        for (i, &u) in chunks.remainder().iter().enumerate() {
            last |= (u as u64) << (16 * i);
        }
        h.add(last);
        h.add(units.len() as u64);
        // Fx leaves the low bits weak; the probe index takes the high bits.
        h.hash
    }

    fn start(&self, hash: u64) -> usize {
        let bits = self.slots.len().trailing_zeros();
        (hash >> (64 - bits)) as usize
    }

    /// The handle whose contents equal `units`, if one is interned.
    pub fn find(&self, strings: &[Box<[u16]>], units: &[u16]) -> Option<u32> {
        if self.slots.is_empty() {
            return None;
        }
        let mask = self.slots.len() - 1;
        let mut i = self.start(self.hash(units));
        loop {
            match self.slots[i] {
                EMPTY => return None,
                TOMBSTONE => {}
                id => {
                    if &*strings[id as usize] == units {
                        return Some(id);
                    }
                }
            }
            i = (i + 1) & mask;
        }
    }

    /// Adds handle `id`, whose contents are `strings[id]` and not yet in
    /// the table.
    pub fn insert(&mut self, strings: &[Box<[u16]>], id: u32) {
        if (self.live + self.tombstones + 1) * 4 > self.slots.len() * 3 {
            self.rebuild(strings);
        }
        let mask = self.slots.len() - 1;
        let mut i = self.start(self.hash(&strings[id as usize]));
        loop {
            match self.slots[i] {
                EMPTY => break,
                TOMBSTONE => {
                    self.tombstones -= 1;
                    break;
                }
                _ => i = (i + 1) & mask,
            }
        }
        self.slots[i] = id;
        self.live += 1;
    }

    /// Removes handle `id`; `strings[id]` must still hold its contents.
    pub fn remove(&mut self, strings: &[Box<[u16]>], id: u32) {
        if self.slots.is_empty() {
            return;
        }
        let mask = self.slots.len() - 1;
        let mut i = self.start(self.hash(&strings[id as usize]));
        loop {
            match self.slots[i] {
                EMPTY => return,
                s if s == id => {
                    self.slots[i] = TOMBSTONE;
                    self.live -= 1;
                    self.tombstones += 1;
                    return;
                }
                _ => i = (i + 1) & mask,
            }
        }
    }

    /// Re-inserts the live handles into a table sized for twice as many.
    fn rebuild(&mut self, strings: &[Box<[u16]>]) {
        let capacity = ((self.live + 1) * 2).next_power_of_two().max(64);
        let old = std::mem::replace(&mut self.slots, vec![EMPTY; capacity]);
        self.live = 0;
        self.tombstones = 0;
        let mask = capacity - 1;
        for id in old.into_iter().filter(|&s| s != EMPTY && s != TOMBSTONE) {
            let mut i = self.start(self.hash(&strings[id as usize]));
            while self.slots[i] != EMPTY {
                i = (i + 1) & mask;
            }
            self.slots[i] = id;
            self.live += 1;
        }
    }
}
