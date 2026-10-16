//! Sets `KAHVI_CODE_FORMAT`: a hash of the parser and compiler sources, so
//! that bytecode cached on disk by one build (`code_cache`) is never loaded
//! by a build that would compile the same source differently.

use std::path::Path;

fn hash_dir(dir: &Path, h: &mut u64) {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("reading {}: {e}", dir.display()))
        .map(|e| e.expect("directory entry").path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            hash_dir(&path, h);
        } else if path.extension().is_some_and(|x| x == "rs") {
            let name = path.file_name().expect("file name").as_encoded_bytes();
            let text = std::fs::read(&path).expect("reading source");
            for &b in name.iter().chain(&text) {
                *h ^= u64::from(b);
                *h = h.wrapping_mul(0x0000_0100_0000_01b3);
            }
        }
    }
}

fn main() {
    let core = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dirs = [core.join("src/compiler"), core.join("../kahvi-parser/src")];
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for dir in &dirs {
        println!("cargo:rerun-if-changed={}", dir.display());
        hash_dir(dir, &mut h);
    }
    println!("cargo:rustc-env=KAHVI_CODE_FORMAT={h:016x}");
}
