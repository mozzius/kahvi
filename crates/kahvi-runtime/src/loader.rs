//! ES module loading from the file system. Specifiers are paths relative to
//! the importing module (`./x.js`, `../x.js`), absolute paths, `file:` URLs,
//! or bare package specifiers resolved from `node_modules` (`package.rs`).

use std::path::{Path, PathBuf};

use kahvi::{Agent, LoadedModuleSource, Object, Value};

/// HostLoadImportedModule: `referrer` is the importing module's key (its
/// absolute path) or a script's name.
pub fn load_module(
    _agent: &mut Agent,
    referrer: &str,
    specifier: &str,
    attributes: &[(String, String)],
) -> Result<LoadedModuleSource, String> {
    if let Some((_, ty)) = attributes.iter().find(|(k, _)| k == "type")
        && ty != "json"
    {
        return Err(format!(
            "cannot load '{specifier}': modules of type '{ty}' are not supported"
        ));
    }
    let path = resolve(referrer, specifier)?;
    let source = std::fs::read(&path)
        .map_err(|e| format!("cannot load module '{specifier}' ({}): {e}", path.display()))?;
    Ok(LoadedModuleSource {
        key: path.to_string_lossy().into_owned(),
        source: String::from_utf8(source)
            .unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned()),
    })
}

/// The absolute, canonical path a specifier names.
pub fn resolve(referrer: &str, specifier: &str) -> Result<PathBuf, String> {
    let path = if let Some(p) = specifier.strip_prefix("file://") {
        PathBuf::from(percent_decode(p))
    } else if specifier.starts_with('/') {
        PathBuf::from(specifier)
    } else if specifier.starts_with("./") || specifier.starts_with("../") {
        let base = Path::new(referrer)
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_default();
        base.join(specifier)
    } else if specifier.starts_with('#') {
        return Err(format!(
            "cannot resolve '{specifier}': package.json \"imports\" are not supported"
        ));
    } else {
        let base = Path::new(referrer)
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_default();
        crate::package::resolve_package(&base, specifier)?
    };
    path.canonicalize()
        .map_err(|e| format!("cannot load module '{specifier}': {e}"))
}

/// HostGetImportMetaProperties: `import.meta.url`, `filename` and `dirname`.
pub fn import_meta(agent: &mut Agent, module: u32, meta: Object) {
    let key = agent.modules[module as usize].host_defined.to_string();
    let path = Path::new(&key);
    let url = file_url(path);
    let dirname = path
        .parent()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    for (name, value) in [
        ("url", url),
        ("filename", key.clone()),
        ("dirname", dirname),
    ] {
        let s = agent.string(&value);
        let k = agent.key(name);
        agent.obj_mut(meta).properties.insert(
            k,
            kahvi_core::object::Property::Data {
                value: Value::String(s),
                writable: true,
                enumerable: true,
                configurable: true,
            },
        );
    }
}

/// A `file://` URL for an absolute path.
pub fn file_url(path: &Path) -> String {
    let mut url = String::from("file://");
    for b in path.to_string_lossy().bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'/' | b'-' | b'_' | b'.' | b'~' => {
                url.push(b as char)
            }
            _ => url.push_str(&format!("%{b:02X}")),
        }
    }
    url
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let hex = |b: u8| (b as char).to_digit(16);
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let (Some(h), Some(l)) = (hex(bytes[i + 1]), hex(bytes[i + 2]))
        {
            let b = (h * 16 + l) as u8;
            out.push(b);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}
