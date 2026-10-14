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
