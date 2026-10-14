//! Turning a test262 file into the exact files the engine is given.
//!
//! Each run gets its own fresh temp directory. Script tests are written under
//! a random file name (so the engine cannot recognise tests by path) as one
//! script: optional `"use strict";`, the harness includes, the test body and,
//! for positive tests, a random completion marker that the engine must print.
//! Module tests keep their file name (they may import themselves) and get the
//! harness as a `--prelude` script. Any `./name` files the test refers to are
//! copied next to it.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::meta::Meta;
use crate::util::Rng;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Sloppy,
    Strict,
    Module,
    Raw,
}

impl Mode {
    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Sloppy => "sloppy",
            Mode::Strict => "strict",
            Mode::Module => "module",
            Mode::Raw => "raw",
        }
    }
}

pub fn modes_for(meta: &Meta) -> Vec<Mode> {
    if meta.has_flag("module") {
        // A raw module test is still a module; `raw` just means no harness.
        vec![Mode::Module]
    } else if meta.has_flag("raw") {
        vec![Mode::Raw]
    } else if meta.has_flag("module") {
        vec![Mode::Module]
    } else if meta.has_flag("onlyStrict") {
        vec![Mode::Strict]
    } else if meta.has_flag("noStrict") {
        vec![Mode::Sloppy]
    } else {
        vec![Mode::Sloppy, Mode::Strict]
    }
}

pub struct Harness {
    dir: PathBuf,
    cache: Mutex<HashMap<String, Arc<Vec<u8>>>>,
}

impl Harness {
    pub fn new(dir: PathBuf) -> Self {
        Harness {
            dir,
            cache: Mutex::new(HashMap::new()),
        }
    }

    fn get(&self, name: &str) -> Result<Arc<Vec<u8>>, String> {
        if let Some(src) = self.cache.lock().unwrap().get(name) {
            return Ok(src.clone());
        }
        let src =
            std::fs::read(self.dir.join(name)).map_err(|e| format!("harness file {name}: {e}"))?;
        let src = Arc::new(src);
        self.cache
            .lock()
            .unwrap()
            .insert(name.to_string(), src.clone());
        Ok(src)
    }
}

pub struct Prepared {
    pub dir: PathBuf,
    pub args: Vec<String>,
    pub nonce: Option<String>,
}

impl Drop for Prepared {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

/// `./name` specifiers mentioned anywhere in `src` (a superset of the real
/// imports, which is fine: only files that exist get copied).
fn relative_refs(src: &[u8]) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i + 3 < src.len() {
        let q = src[i];
        if (q == b'\'' || q == b'"') && src[i + 1] == b'.' && src[i + 2] == b'/' {
            let start = i + 3;
            if let Some(len) = src[start..]
                .iter()
                .take(256)
                .position(|&c| c == q || c == b'\n')
            {
                if src[start + len] == q
                    && let Ok(name) = std::str::from_utf8(&src[start..start + len])
                    && !name.is_empty()
                    && !name.contains('/')
                    && !name.contains("..")
                {
                    out.push(name.to_string());
                }
                i = start + len;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn copy_fixtures(test_dir: &Path, own_name: &str, src: &[u8], dest: &Path) -> Result<(), String> {
    let mut seen = HashSet::new();
    let mut queue = relative_refs(src);
    while let Some(name) = queue.pop() {
        if name == own_name || !seen.insert(name.clone()) {
            continue;
        }
        let from = test_dir.join(&name);
        if !from.is_file() {
            continue;
        }
        let data = std::fs::read(&from).map_err(|e| format!("fixture {name}: {e}"))?;
        queue.extend(relative_refs(&data));
        std::fs::write(dest.join(&name), &data).map_err(|e| format!("fixture {name}: {e}"))?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn prepare(
    test_path: &Path,
    src: &[u8],
    meta: &Meta,
    mode: Mode,
    harness: &Harness,
    tmp_root: &Path,
    rng: &mut Rng,
    parse_only: bool,
) -> Result<Prepared, String> {
    if parse_only {
        return prepare_parse_only(test_path, src, mode, tmp_root, rng);
    }
    let dir = tmp_root.join(rng.hex());
    std::fs::create_dir_all(&dir).map_err(|e| format!("tmp dir: {e}"))?;
    let mut prepared = Prepared {
        dir: dir.clone(),
        args: Vec::new(),
        nonce: None,
    };

    let own_name = test_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("test.js");
    let self_ref = relative_refs(src).iter().any(|n| n == own_name);
    let file_name = if mode == Mode::Module || self_ref {
        own_name.to_string()
    } else {
        format!("{}.js", rng.hex())
    };
    if mode != Mode::Raw {
        copy_fixtures(
            test_path.parent().unwrap_or(Path::new(".")),
            own_name,
            src,
            &dir,
        )?;
    }

    let mut body = src.to_vec();
    if mode != Mode::Raw && !meta.has_flag("raw") && meta.negative.is_none() {
        let nonce = format!("KAHVI-T262-DONE-{}", rng.hex());
        body.extend_from_slice(format!("\n;print('{nonce}');\n").as_bytes());
        prepared.nonce = Some(nonce);
    }

    let mut includes: Vec<&str> = Vec::new();
    if mode != Mode::Raw && !meta.has_flag("raw") {
        includes.extend(["assert.js", "sta.js"]);
        if meta.has_flag("async") {
            includes.push("doneprintHandle.js");
        }
        for inc in &meta.includes {
            if !includes.contains(&inc.as_str()) {
                includes.push(inc);
            }
        }
    }
    let mut prelude = Vec::new();
    for inc in &includes {
        prelude.extend_from_slice(&harness.get(inc)?);
        prelude.push(b'\n');
    }

    let test_file = dir.join(&file_name);
    let write = |path: &Path, data: &[u8]| {
        std::fs::write(path, data).map_err(|e| format!("write {}: {e}", path.display()))
    };
    match mode {
        Mode::Raw => write(&test_file, &body)?,
        Mode::Sloppy | Mode::Strict => {
            let mut script = Vec::with_capacity(prelude.len() + body.len() + 16);
            if mode == Mode::Strict {
                script.extend_from_slice(b"\"use strict\";\n");
            }
            script.extend_from_slice(&prelude);
            script.extend_from_slice(&body);
            write(&test_file, &script)?;
        }
        Mode::Module => {
            let prelude_file = dir.join(format!("prelude-{}.js", rng.hex()));
            write(&prelude_file, &prelude)?;
            write(&test_file, &body)?;
            prepared.args.push("--prelude".into());
            prepared
                .args
                .push(prelude_file.to_string_lossy().into_owned());
            prepared.args.push("--module".into());
        }
    }
    prepared.args.push(test_file.to_string_lossy().into_owned());
    Ok(prepared)
}

/// Parse-only mode: the test file alone (no harness, no marker), as the
/// goal and strictness the test runs under.
fn prepare_parse_only(
    test_path: &Path,
    src: &[u8],
    mode: Mode,
    tmp_root: &Path,
    rng: &mut Rng,
) -> Result<Prepared, String> {
    let dir = tmp_root.join(rng.hex());
    std::fs::create_dir_all(&dir).map_err(|e| format!("tmp dir: {e}"))?;
    let name = test_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("test.js")
        .to_string();
    let file = dir.join(name);
    let mut body = Vec::with_capacity(src.len() + 16);
    if mode == Mode::Strict {
        body.extend_from_slice(b"\"use strict\";\n");
    }
    body.extend_from_slice(src);
    std::fs::write(&file, &body).map_err(|e| format!("write {}: {e}", file.display()))?;
    let mut args = vec!["--parse-only".to_string()];
    if mode == Mode::Module {
        args.push("--module".into());
    }
    args.push(file.to_string_lossy().into_owned());
    Ok(Prepared {
        dir,
        args,
        nonce: None,
    })
}
