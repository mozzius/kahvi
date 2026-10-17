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
//! It also keeps the compiled code of the lazy functions that were called
//! (`compiler::codec`), so the next runtime or run that calls one skips
//! parsing and compiling it. On disk that code is appended to a file per
//! source as functions are compiled, under a header naming the compiler
//! build, so a different build ignores and replaces it.
//!
//! Entries are keyed by the source text's length and a 128-bit hash of it,
//! and by the goal (script or module); the parser also checks that each
//! skipped body starts with `{` and ends with `}` in the text it is given.

use std::io::{Read, Write};
use std::ops::{Deref, Range};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

use kahvi_parser::preparse::Preparse;

use crate::source::SourceText;

/// Sources shorter than this are parsed in full: hashing and storing them
/// costs about what it saves.
const MIN_SOURCE_LEN: usize = 256 << 10;

/// Identifies a source text and goal in the cache.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Key {
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
    entries: crate::hash::FxHashMap<Key, Arc<Preparse>>,
    functions: crate::hash::FxHashMap<Key, Functions>,
    /// The source texts that runtimes hold, so another runtime evaluating
    /// the same text shares the copy instead of making its own.
    sources: crate::hash::FxHashMap<Key, std::sync::Weak<SourceText>>,
}

/// The cached code of one source's functions, by the offset of each.
#[derive(Default)]
struct Functions {
    code: crate::hash::FxHashMap<u32, CodeBytes>,
    /// The file new code is appended to.
    file: Option<std::fs::File>,
}

/// One function's encoded code, in a buffer it may share with others.
#[derive(Clone)]
pub(crate) struct CodeBytes {
    blob: Arc<Blob>,
    range: Range<usize>,
}

/// The bytes [`CodeBytes`] are part of: code compiled in this process, or a
/// code file. A code file is kept as a read-only mapping (as large source
/// texts are, see [`SourceText`]): the system loads the pages of a
/// function's code when it is decoded and can drop them again, where a
/// copy would hold all of it for as long as the process runs.
enum Blob {
    Heap(Box<[u8]>),
    #[cfg(all(unix, target_pointer_width = "64"))]
    Mapped(crate::source::map::Mapping),
}

impl Blob {
    fn release_pages(&self) {
        #[cfg(all(unix, target_pointer_width = "64"))]
        if let Blob::Mapped(m) = self {
            m.release_pages();
        }
    }
}

impl Deref for Blob {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        match self {
            Blob::Heap(b) => b,
            #[cfg(all(unix, target_pointer_width = "64"))]
            Blob::Mapped(m) => m.bytes(),
        }
    }
}

impl Deref for CodeBytes {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.blob[self.range.clone()]
    }
}

/// Starts the code file: the format of the code in it.
fn code_file_header() -> Vec<u8> {
    let mut h = b"kvbc\0\0\0\x01".to_vec();
    h.extend_from_slice(crate::compiler::codec::FORMAT.as_bytes());
    h
}

fn checksum(bytes: &[u8]) -> u32 {
    bytes.iter().fold(0x811c_9dc5u32, |h, &b| {
        (h ^ u32::from(b)).wrapping_mul(0x0100_0193)
    })
}

/// The bytes of the code file at `path`, mapped where the system allows
/// it.
fn read_code_file(path: &Path) -> Option<Blob> {
    let mut file = std::fs::File::open(path).ok()?;
    #[cfg(all(unix, target_pointer_width = "64"))]
    if let Some(m) = crate::source::map::Mapping::file(&file) {
        return Some(Blob::Mapped(m));
    }
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).ok()?;
    Some(Blob::Heap(bytes.into()))
}

/// The records of a code file's `bytes` (each function's offset and where
/// its code is), up to the first that is cut short or damaged, and the
/// length of the bytes they take; 0 if the file is from another build.
fn code_records(bytes: &[u8], header: &[u8]) -> (usize, Vec<(u32, Range<usize>)>) {
    let mut records = Vec::new();
    if !bytes.starts_with(header) {
        return (0, records);
    }
    let mut pos = header.len();
    while let Some(head) = bytes.get(pos..pos + 8) {
        let start = u32::from_le_bytes(head[..4].try_into().expect("4 bytes"));
        let len = u32::from_le_bytes(head[4..].try_into().expect("4 bytes")) as usize;
        let body = pos + 8..pos + 8 + len;
        let Some(sum) = bytes.get(body.end..body.end + 4) else {
            break;
        };
        if checksum(&bytes[body.clone()]).to_le_bytes() != sum {
            break;
        }
        pos = body.end + 4;
        records.push((start, body));
    }
    (pos, records)
}

/// Reads the code file at `path`, keeping its valid records in `f`, and
/// opens it for appending: a file from another build is started afresh,
/// and a record cut short (the process ended while writing it) is dropped.
///
/// The file is mapped, so it is only ever appended to or replaced whole by
/// rename, never changed in place: another process may have it mapped.
fn open_code_file(path: &Path, f: &mut Functions) -> Option<()> {
    let header = code_file_header();
    let mut blob = read_code_file(path);
    let (mut valid, mut records) = blob
        .as_deref()
        .map_or((0, Vec::new()), |b| code_records(b, &header));
    if valid == 0 || blob.as_ref().is_some_and(|b| b.len() != valid) {
        let keep = match &blob {
            Some(b) if valid > 0 => &b[..valid],
            _ => &header[..],
        };
        let tmp = path.with_extension(format!("tmp{}", std::process::id()));
        if std::fs::write(&tmp, keep)
            .and_then(|()| std::fs::rename(&tmp, path))
            .is_err()
        {
            let _ = std::fs::remove_file(&tmp);
            return None;
        }
        blob = read_code_file(path);
        (valid, records) = blob
            .as_deref()
            .map_or((0, Vec::new()), |b| code_records(b, &header));
    }
    let blob = Arc::new(blob?);
    // Checking the records read every page; the system reads them back
    // from the file as functions' code is decoded.
    blob.release_pages();
    for (start, range) in records {
        f.code.insert(
            start,
            CodeBytes {
                blob: blob.clone(),
                range,
            },
        );
    }
    if valid == 0 {
        return None;
    }
    f.file = Some(std::fs::OpenOptions::new().append(true).open(path).ok()?);
    Some(())
}

fn cache() -> &'static Mutex<Cache> {
    static CACHE: OnceLock<Mutex<Cache>> = OnceLock::new();
    CACHE.get_or_init(|| {
        Mutex::new(Cache {
            dir: None,
            entries: Default::default(),
            functions: Default::default(),
            sources: Default::default(),
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

/// `KAHVI_CODE_CACHE_STRESS` set: every lazily compiled function runs the
/// code its encoding decodes to, cached or not (finds codec bugs, the way
/// `KAHVI_GC_STRESS` finds rooting bugs).
pub(crate) fn stress() -> bool {
    static STRESS: OnceLock<bool> = OnceLock::new();
    *STRESS.get_or_init(|| std::env::var_os("KAHVI_CODE_CACHE_STRESS").is_some())
}

/// The cache's key for `source` with the given goal, if it is large enough
/// to cache.
pub(crate) fn key(source: &str, module: bool) -> Option<Key> {
    (source.len() >= MIN_SOURCE_LEN).then(|| Key {
        hash: hash(source),
        len: source.len(),
        module,
    })
}

/// The [`Preparse`] of an earlier parse of the source `key` names (with
/// lazy compilation), from memory or the cache directory.
pub(crate) fn lookup(key: &Key) -> Option<Arc<Preparse>> {
    let mut c = cache().lock().unwrap_or_else(|e| e.into_inner());
    if let Some(p) = c.entries.get(key) {
        return Some(p.clone());
    }
    let path = c.dir.as_ref()?.join(key.file_name());
    let p = Arc::new(Preparse::from_bytes(&std::fs::read(path).ok()?)?);
    c.entries.insert(*key, p.clone());
    Some(p)
}

/// Records what a parse of the source `key` names learned, for the next
/// parse of it.
pub(crate) fn store(key: &Key, preparse: &Arc<Preparse>) {
    let mut c = cache().lock().unwrap_or_else(|e| e.into_inner());
    if c.entries.insert(*key, preparse.clone()).is_some() {
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

/// The source's functions, read from the cache directory the first time.
fn functions<'a>(c: &'a mut Cache, key: &Key) -> &'a mut Functions {
    if !c.functions.contains_key(key) {
        c.functions
            .insert(*key, read_functions(c.dir.as_deref(), key));
    }
    c.functions.get_mut(key).expect("inserted above")
}

fn read_functions(dir: Option<&Path>, key: &Key) -> Functions {
    {
        let mut f = Functions::default();
        if let Some(dir) = dir {
            let path = dir.join(key.file_name().replace(".kvpp", ".kvbc"));
            if open_code_file(&path, &mut f).is_none() {
                f.file = None;
            }
        }
        f
    }
}

/// `source` as a shared string: the copy another runtime holds of the same
/// text when there is one, else a new copy that later ones share.
pub(crate) fn shared_source(key: Option<&Key>, source: &str) -> Arc<SourceText> {
    let Some(key) = key else {
        return SourceText::new(source);
    };
    let mut c = cache().lock().unwrap_or_else(|e| e.into_inner());
    if let Some(s) = c.sources.get(key).and_then(std::sync::Weak::upgrade)
        && **s == *source
    {
        return s;
    }
    // In the cache directory, the text is kept as a mapping of a copy there
    // (see `SourceText`).
    let mapped = c.dir.as_ref().and_then(|d| {
        SourceText::mapped(&d.join(key.file_name().replace(".kvpp", ".src")), source)
    });
    let s = mapped.map_or_else(|| SourceText::new(source), Arc::new);
    c.sources.retain(|_, w| w.strong_count() > 0);
    c.sources.insert(*key, Arc::downgrade(&s));
    s
}

/// The encoded code of the function at offset `start` of the source `key`
/// names, if it was cached.
pub(crate) fn function(key: &Key, start: u32) -> Option<CodeBytes> {
    let mut c = cache().lock().unwrap_or_else(|e| e.into_inner());
    functions(&mut c, key).code.get(&start).cloned()
}

/// Caches `code`, the encoded code of the function at offset `start` of the
/// source `key` names.
pub(crate) fn store_function(key: &Key, start: u32, code: Vec<u8>) {
    let mut c = cache().lock().unwrap_or_else(|e| e.into_inner());
    let f = functions(&mut c, key);
    if f.code.contains_key(&start) {
        return;
    }
    if let Some(file) = &mut f.file {
        let mut record = Vec::with_capacity(code.len() + 12);
        record.extend_from_slice(&start.to_le_bytes());
        record.extend_from_slice(&(code.len() as u32).to_le_bytes());
        record.extend_from_slice(&code);
        record.extend_from_slice(&checksum(&code).to_le_bytes());
        // A failed write leaves at most a cut-short record, dropped when the
        // file is next read; stop writing to it.
        if file.write_all(&record).is_err() {
            f.file = None;
        }
    }
    let len = code.len();
    f.code.insert(
        start,
        CodeBytes {
            blob: Arc::new(Blob::Heap(code.into())),
            range: 0..len,
        },
    );
}
