//! Parsing of test262 frontmatter (`/*--- ... ---*/`).
//!
//! The frontmatter is YAML, but test262 only uses a small, regular subset for
//! the keys we care about: flow lists (`flags: [a, b]`), block lists
//! (`flags:` followed by `  - a` lines) and the two-level `negative:` map.
//! Everything else (description, info, ...) is skipped.

#[derive(Debug, Default, Clone)]
pub struct Meta {
    pub flags: Vec<String>,
    pub includes: Vec<String>,
    pub features: Vec<String>,
    pub negative: Option<Negative>,
    /// Canary-only key: whether the runner expects the test to pass.
    pub expect: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct Negative {
    pub phase: String,
    pub error_type: String,
}

impl Meta {
    pub fn has_flag(&self, flag: &str) -> bool {
        self.flags.iter().any(|f| f == flag)
    }
}

fn parse_flow_list(rest: &str) -> Vec<String> {
    let inner = match (rest.find('['), rest.rfind(']')) {
        (Some(a), Some(b)) if a < b => &rest[a + 1..b],
        _ => return Vec::new(),
    };
    inner
        .split(',')
        .map(clean_scalar)
        .filter(|s| !s.is_empty())
        .collect()
}

fn clean_scalar(s: &str) -> String {
    let s = s.trim();
    let s = match s.find(" #") {
        Some(i) => s[..i].trim(),
        None => s,
    };
    s.trim_matches(|c| c == '"' || c == '\'').to_string()
}

pub fn parse(src: &[u8]) -> Result<Meta, String> {
    let text = String::from_utf8_lossy(src);
    let start = text.find("/*---").ok_or("no frontmatter")? + 5;
    let end = start
        + text[start..]
            .find("---*/")
            .ok_or("unterminated frontmatter")?;
    let mut meta = Meta::default();
    let mut current = String::new();
    for line in text[start..end].lines() {
        if line.trim().is_empty() {
            continue;
        }
        let indented = line.starts_with(' ') || line.starts_with('\t');
        if !indented {
            let Some((key, rest)) = line.split_once(':') else {
                current.clear();
                continue;
            };
            current = key.trim().to_string();
            let rest = rest.trim();
            match current.as_str() {
                "flags" => meta.flags = parse_flow_list(rest),
                "includes" => meta.includes = parse_flow_list(rest),
                "features" => meta.features = parse_flow_list(rest),
                "negative" => meta.negative = Some(Negative::default()),
                "expect" => meta.expect = Some(clean_scalar(rest)),
                _ => {}
            }
            continue;
        }
        let item = line.trim();
        match current.as_str() {
            "flags" | "includes" | "features" => {
                if let Some(value) = item.strip_prefix('-') {
                    let value = clean_scalar(value);
                    let list = match current.as_str() {
                        "flags" => &mut meta.flags,
                        "includes" => &mut meta.includes,
                        _ => &mut meta.features,
                    };
                    list.push(value);
                }
            }
            "negative" => {
                let neg = meta.negative.get_or_insert_with(Negative::default);
                if let Some(v) = item.strip_prefix("phase:") {
                    neg.phase = clean_scalar(v);
                } else if let Some(v) = item.strip_prefix("type:") {
                    neg.error_type = clean_scalar(v);
                }
            }
            _ => {}
        }
    }
    if let Some(neg) = &meta.negative
        && (neg.phase.is_empty() || neg.error_type.is_empty())
    {
        return Err("negative: needs phase and type".into());
    }
    Ok(meta)
}

#[cfg(test)]
mod tests {
    use super::*;

    // The runner is the judge of the whole project, so its parsing of the
    // frontmatter shapes test262 actually uses is worth pinning down.
    #[test]
    fn parses_flow_block_and_negative_forms() {
        let src = b"/*---\ndescription: |\n  - not a flag\nflags:\n  - noStrict\nincludes: [a.js, 'b.js']\nfeatures: [x] # trailing\nnegative:\n    phase: parse\n    type: SyntaxError\n---*/\nfoo();";
        let m = parse(src).unwrap();
        assert_eq!(m.flags, vec!["noStrict"]);
        assert_eq!(m.includes, vec!["a.js", "b.js"]);
        assert_eq!(m.features, vec!["x"]);
        let neg = m.negative.unwrap();
        assert_eq!(
            (neg.phase.as_str(), neg.error_type.as_str()),
            ("parse", "SyntaxError")
        );
    }
}
