//! A cache of what parsing large source texts learned about their lazy
//! function bodies ([`Preparse`]), shared by every agent in the process and,
//! when the host names a directory, kept on disk between runs.
//!
//! An app bundle is parsed by each runtime that evaluates it (React Native
//! evaluates its bundle in the JavaScript runtime and again in the worklets
//! UI runtime) and on every launch. With the cache only the first of those
//! parses checks the bodies of the bundle's functions; the others skip
//! them, and each body is parsed when its function is first called.
//!
//! Entries are keyed by the source text's length and a 128-bit hash of it,
//! and by the goal (script or module); the parser also checks that each
//! skipped body starts with `{` and ends with `}` in the text it is given.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

use kahvi_parser::preparse::Preparse;

/// Sources shorter than this are parsed in full: hashing and storing them
/// costs about what it saves.
const MIN_SOURCE_LEN: usize = 256 << 10;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Key {
    hash: [u64; 2],
    len: usize,
    module: bool,
}

impl Key {
    fn file_name(&self) -> String {
        format!(
            "{:016x}{:016x}-{:x}-{}.kvpp",
            self.hash[0],
            self.hash[1],
            self.len,
            if self.module { "m" } else { "s" }
        )
    }
}

struct Cache {
    dir: Option<PathBuf>,
    entries: HashMap<Key, Arc<Preparse>>,
}

fn cache() -> &'static Mutex<Cache> {
    static CACHE: OnceLock<Mutex<Cache>> = OnceLock::new();
    CACHE.get_or_init(|| {
        Mutex::new(Cache {
            dir: None,
            entries: HashMap::new(),
        })
    })
}

/// Keeps the cache in `dir` (created if missing) as well as in memory, or
/// only in memory for `None`. Hosts call this before evaluating code.
pub fn set_dir(dir: Option<PathBuf>) {
    if let Some(d) = &dir {
        let _ = std::fs::create_dir_all(d);
    }
    cache().lock().unwrap_or_else(|e| e.into_inner()).dir = dir;
}

/// Two independent 64-bit multiply-xorshift hashes over the bytes of `s`.
fn hash(s: &str) -> [u64; 2] {
    const K0: u64 = 0x9e37_79b9_7f4a_7c15;
    const K1: u64 = 0xc2b2_ae3d_27d4_eb4f;
    let mix = |h: u64, w: u64, k: u64| (h ^ w).wrapping_mul(k).rotate_left(29);
    let (mut a, mut b) = (K1, K0);
    let bytes = s.as_bytes();
    let mut chunks = bytes.chunks_exact(8);
    for c in &mut chunks {
        let w = u64::from_le_bytes(c.try_into().expect("8 bytes"));
        a = mix(a, w, K0);
        b = mix(b, w.rotate_left(32), K1);
    }
    let mut tail = [0u8; 8];
    tail[..chunks.remainder().len()].copy_from_slice(chunks.remainder());
    let w = u64::from_le_bytes(tail);
    a = mix(a, w, K0);
    b = mix(b, w.rotate_left(32), K1);
    let fin = |mut h: u64| {
        h ^= h >> 33;
        h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
        h ^= h >> 33;
        h = h.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
        h ^ (h >> 33)
    };
    [fin(a ^ bytes.len() as u64), fin(b)]
}

fn key(source: &str, module: bool) -> Option<Key> {
    (source.len() >= MIN_SOURCE_LEN).then(|| Key {
        hash: hash(source),
        len: source.len(),
        module,
    })
}

/// The [`Preparse`] of an earlier parse of `source` with the given goal
/// (and lazy compilation), from memory or the cache directory.
pub(crate) fn lookup(source: &str, module: bool) -> Option<Arc<Preparse>> {
    let key = key(source, module)?;
    let mut c = cache().lock().unwrap_or_else(|e| e.into_inner());
    if let Some(p) = c.entries.get(&key) {
        return Some(p.clone());
    }
    let path = c.dir.as_ref()?.join(key.file_name());
    let p = Arc::new(Preparse::from_bytes(&std::fs::read(path).ok()?)?);
    c.entries.insert(key, p.clone());
    Some(p)
}

/// Records what a parse of `source` learned, for the next parse of it.
pub(crate) fn store(source: &str, module: bool, preparse: &Arc<Preparse>) {
    let Some(key) = key(source, module) else {
        return;
    };
    let mut c = cache().lock().unwrap_or_else(|e| e.into_inner());
    if c.entries.insert(key, preparse.clone()).is_some() {
        return;
    }
    if let Some(dir) = &c.dir {
        // Written whole then renamed, so a reader never sees part of it. A
        // cache that cannot be written is only a slower next start.
        let path = dir.join(key.file_name());
        let tmp = dir.join(format!("{}.{}.tmp", key.file_name(), std::process::id()));
        if std::fs::write(&tmp, preparse.to_bytes()).is_ok()
            && std::fs::rename(&tmp, &path).is_err()
        {
            let _ = std::fs::remove_file(&tmp);
        }
    }
}
