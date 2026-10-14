//! Bare specifiers: packages in `node_modules` directories, found the way
//! Node's ESM resolver finds them.
//!
//! - `name` and `@scope/name` (optionally followed by `/subpath`) are looked
//!   up in `node_modules` in the importing module's directory and each
//!   directory above it.
//! - A package's `package.json` `"exports"` decides what it exposes: a
//!   string, an array (first entry that resolves), conditions (`"import"`,
//!   `"kahvi"`, `"default"`, in the object's order), and subpath keys,
//!   including one `*` pattern per key. A package with `"exports"` exposes
//!   nothing else.
//! - Without `"exports"`, `name` resolves to `"main"` (as a file, then with
//!   `.js`, then as a directory's `index.js`) or `index.js`, and
//!   `name/subpath` to that file in the package.
//!
//! `"imports"` (`#name` specifiers) and self-references are not supported.

use std::path::{Path, PathBuf};

/// The conditions this runtime matches in `"exports"`: an ES module host.
const CONDITIONS: &[&str] = &["kahvi", "import", "default"];

/// The file a bare specifier names, from the directory of the importing module.
pub fn resolve_package(from_dir: &Path, specifier: &str) -> Result<PathBuf, String> {
    let (name, subpath) = split_specifier(specifier)
        .ok_or_else(|| format!("cannot resolve '{specifier}': invalid package specifier"))?;
    let mut dir = Some(from_dir);
    while let Some(d) = dir {
        let package_dir = d.join("node_modules").join(name);
        if package_dir.is_dir() {
            return resolve_in_package(&package_dir, specifier, &subpath);
        }
        dir = d.parent();
    }
    Err(format!(
        "cannot resolve '{specifier}': package '{name}' not found in any node_modules directory"
    ))
}

/// The package name and the subpath (`.` or `./rest`) of a bare specifier.
fn split_specifier(specifier: &str) -> Option<(&str, String)> {
    let name_len = if specifier.starts_with('@') {
        let slash = specifier.find('/')?;
        match specifier[slash + 1..].find('/') {
            Some(i) => slash + 1 + i,
            None => specifier.len(),
        }
    } else {
        specifier.find('/').unwrap_or(specifier.len())
    };
    let name = &specifier[..name_len];
    if name.is_empty() || name.ends_with('/') || name.starts_with('.') || name.contains('\\') {
        return None;
    }
    let subpath = format!(".{}", &specifier[name_len..]);
    Some((name, subpath))
}

fn resolve_in_package(
    package_dir: &Path,
    specifier: &str,
    subpath: &str,
) -> Result<PathBuf, String> {
    let manifest = match std::fs::read_to_string(package_dir.join("package.json")) {
        Ok(text) => Some(
            Json::parse(&text)
                .map_err(|e| format!("cannot resolve '{specifier}': bad package.json ({e})"))?,
        ),
        Err(_) => None,
    };
    let field = |name: &str| match &manifest {
        Some(Json::Object(entries)) => entries.iter().find(|(k, _)| k == name).map(|(_, v)| v),
        _ => None,
    };
    if let Some(exports) = field("exports") {
        return match package_exports_resolve(exports, subpath)? {
            Some(target) => existing_file(&package_dir.join(target), specifier),
            None => Err(format!(
                "cannot resolve '{specifier}': '{subpath}' is not exported by the package"
            )),
        };
    }
    if subpath != "." {
        return existing_file(&package_dir.join(subpath), specifier);
    }
    if let Some(Json::String(main)) = field("main") {
        let main = package_dir.join(main);
        for candidate in [main.clone(), with_js(&main), main.join("index.js")] {
            if candidate.is_file() {
                return Ok(candidate);
            }
        }
    }
    existing_file(&package_dir.join("index.js"), specifier)
}

fn with_js(path: &Path) -> PathBuf {
    let mut s = path.as_os_str().to_owned();
    s.push(".js");
    PathBuf::from(s)
}

fn existing_file(path: &Path, specifier: &str) -> Result<PathBuf, String> {
    if path.is_file() {
        Ok(path.to_path_buf())
    } else {
        Err(format!(
            "cannot resolve '{specifier}': {} does not exist",
            path.display()
        ))
    }
}

/// The target `"exports"` maps `subpath` to, relative to the package (`./...`),
/// or `None` when the subpath is not exported.
fn package_exports_resolve(exports: &Json, subpath: &str) -> Result<Option<String>, String> {
    // A string, an array or conditions alone are the "." export.
    let has_subpath_keys = match exports {
        Json::Object(entries) => entries.iter().any(|(k, _)| k.starts_with('.')),
        _ => false,
    };
    if !has_subpath_keys {
        return if subpath == "." {
            resolve_target(exports, None)
        } else {
            Ok(None)
        };
    }
    let Json::Object(entries) = exports else {
        unreachable!("subpath keys are in an object")
    };
    if entries.iter().any(|(k, _)| !k.starts_with('.')) {
        return Err("package.json \"exports\" mixes subpaths and conditions".into());
    }
    if let Some((_, target)) = entries
        .iter()
        .find(|(k, _)| k == subpath && !k.contains('*'))
    {
        return resolve_target(target, None);
    }
    // The longest matching `*` pattern wins.
    let mut best: Option<(&str, &Json, &str)> = None;
    for (key, target) in entries {
        let Some(star) = key.find('*') else { continue };
        let (prefix, suffix) = (&key[..star], &key[star + 1..]);
        if subpath.len() >= prefix.len() + suffix.len()
            && subpath.starts_with(prefix)
            && subpath.ends_with(suffix)
            && best.is_none_or(|(k, _, _)| prefix.len() > k.find('*').unwrap_or(0))
        {
            let matched = &subpath[prefix.len()..subpath.len() - suffix.len()];
            best = Some((key, target, matched));
        }
    }
    match best {
        Some((_, target, matched)) => resolve_target(target, Some(matched)),
        None => Ok(None),
    }
}

/// One `"exports"` target: a `./` path (with `*` replaced by `pattern`), an
/// array of fallbacks, conditions, or null (not exported).
fn resolve_target(target: &Json, pattern: Option<&str>) -> Result<Option<String>, String> {
    match target {
        Json::String(s) => {
            if !s.starts_with("./") || s.split('/').any(|seg| seg == ".." || seg == "node_modules")
            {
                return Err(format!("invalid package.json \"exports\" target '{s}'"));
            }
            Ok(Some(match pattern {
                Some(p) => s.replace('*', p),
                None => s.clone(),
            }))
        }
        Json::Array(items) => {
            for item in items {
                if let Ok(Some(t)) = resolve_target(item, pattern) {
                    return Ok(Some(t));
                }
            }
            Ok(None)
        }
        Json::Object(entries) => {
            for (condition, value) in entries {
                if CONDITIONS.contains(&condition.as_str())
                    && let Some(t) = resolve_target(value, pattern)?
                {
                    return Ok(Some(t));
                }
            }
            Ok(None)
        }
        Json::Null => Ok(None),
        _ => Err("invalid package.json \"exports\" target".into()),
    }
}

/// The JSON values a `package.json` holds. Objects keep their key order,
/// which decides which condition matches first.
#[derive(Debug)]
enum Json {
    Null,
    Bool,
    Number,
    String(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl Json {
    fn parse(text: &str) -> Result<Json, String> {
        let mut p = JsonParser {
            s: text.as_bytes(),
            i: 0,
        };
        let v = p.value()?;
        p.ws();
        if p.i != p.s.len() {
            return Err(format!("unexpected text at byte {}", p.i));
        }
        Ok(v)
    }
}

struct JsonParser<'a> {
    s: &'a [u8],
    i: usize,
}

impl JsonParser<'_> {
    fn ws(&mut self) {
        while matches!(self.s.get(self.i), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.i += 1;
        }
    }

    fn eat(&mut self, b: u8) -> bool {
        self.ws();
        if self.s.get(self.i) == Some(&b) {
            self.i += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, b: u8) -> Result<(), String> {
        if self.eat(b) {
            Ok(())
        } else {
            Err(format!("expected '{}' at byte {}", b as char, self.i))
        }
    }

    fn literal(&mut self, word: &str, v: Json) -> Result<Json, String> {
        if self.s[self.i..].starts_with(word.as_bytes()) {
            self.i += word.len();
            Ok(v)
        } else {
            Err(format!("unexpected text at byte {}", self.i))
        }
    }

    fn value(&mut self) -> Result<Json, String> {
        self.ws();
        match self.s.get(self.i) {
            Some(b'n') => self.literal("null", Json::Null),
            Some(b't') => self.literal("true", Json::Bool),
            Some(b'f') => self.literal("false", Json::Bool),
            Some(b'"') => Ok(Json::String(self.string()?)),
            Some(b'[') => {
                self.i += 1;
                let mut items = Vec::new();
                if !self.eat(b']') {
                    loop {
                        items.push(self.value()?);
                        if self.eat(b']') {
                            break;
                        }
                        self.expect(b',')?;
                    }
                }
                Ok(Json::Array(items))
            }
            Some(b'{') => {
                self.i += 1;
                let mut entries = Vec::new();
                if !self.eat(b'}') {
                    loop {
                        self.ws();
                        let key = self.string()?;
                        self.expect(b':')?;
                        entries.push((key, self.value()?));
                        if self.eat(b'}') {
                            break;
                        }
                        self.expect(b',')?;
                    }
                }
                Ok(Json::Object(entries))
            }
            Some(b'-' | b'0'..=b'9') => {
                while matches!(
                    self.s.get(self.i),
                    Some(b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9')
                ) {
                    self.i += 1;
                }
                Ok(Json::Number)
            }
            _ => Err(format!("unexpected text at byte {}", self.i)),
        }
    }

    fn string(&mut self) -> Result<String, String> {
        if self.s.get(self.i) != Some(&b'"') {
            return Err(format!("expected a string at byte {}", self.i));
        }
        self.i += 1;
        let mut units: Vec<u16> = Vec::new();
        loop {
            let start = self.i;
            while let Some(&b) = self.s.get(self.i) {
                if b == b'"' || b == b'\\' {
                    break;
                }
                self.i += 1;
            }
            let run = std::str::from_utf8(&self.s[start..self.i])
                .map_err(|_| "invalid UTF-8".to_string())?;
            units.extend(run.encode_utf16());
            match self.s.get(self.i) {
                Some(b'"') => {
                    self.i += 1;
                    return Ok(String::from_utf16_lossy(&units));
                }
                Some(b'\\') => {
                    let esc = *self.s.get(self.i + 1).ok_or("unterminated string")?;
                    self.i += 2;
                    let unit = match esc {
                        b'"' => b'"' as u16,
                        b'\\' => b'\\' as u16,
                        b'/' => b'/' as u16,
                        b'b' => 0x08,
                        b'f' => 0x0c,
                        b'n' => b'\n' as u16,
                        b'r' => b'\r' as u16,
                        b't' => b'\t' as u16,
                        b'u' => {
                            let hex = self
                                .s
                                .get(self.i..self.i + 4)
                                .and_then(|h| std::str::from_utf8(h).ok())
                                .and_then(|h| u16::from_str_radix(h, 16).ok())
                                .ok_or("bad \\u escape")?;
                            self.i += 4;
                            hex
                        }
                        _ => return Err(format!("bad escape at byte {}", self.i)),
                    };
                    units.push(unit);
                }
                _ => return Err("unterminated string".into()),
            }
        }
    }
}
