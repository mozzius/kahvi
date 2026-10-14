//! Unicode character data, generated from the Unicode Character Database by
//! `harness/gen/unicode.py`.

mod tables;

pub use tables::UNICODE_VERSION;

/// The largest code point.
pub const MAX_CODE_POINT: u32 = 0x10FFFF;

fn find_range<T>(ranges: &[(u32, u32, T)], c: u32) -> Option<&T> {
    ranges
        .binary_search_by(|&(lo, hi, _)| {
            if hi < c {
                std::cmp::Ordering::Less
            } else if lo > c {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .ok()
        .map(|i| &ranges[i].2)
}

/// Whether `c` is in a sorted list of disjoint inclusive ranges.
pub fn in_ranges(ranges: &[(u32, u32)], c: u32) -> bool {
    ranges
        .binary_search_by(|&(lo, hi)| {
            if hi < c {
                std::cmp::Ordering::Less
            } else if lo > c {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .is_ok()
}

/// The ID_Start property (UAX #31).
pub fn is_id_start(c: char) -> bool {
    if c.is_ascii() {
        return c.is_ascii_alphabetic();
    }
    in_ranges(tables::ID_START, c as u32)
}

/// The ID_Continue property (UAX #31).
pub fn is_id_continue(c: char) -> bool {
    if c.is_ascii() {
        return c.is_ascii_alphanumeric() || c == '_';
    }
    in_ranges(tables::ID_CONTINUE, c as u32)
}

// ---- range lists -----------------------------------------------------------

fn push_range(out: &mut Vec<(u32, u32)>, lo: u32, hi: u32) {
    match out.last_mut() {
        Some(last) if last.1 + 1 >= lo => last.1 = last.1.max(hi),
        _ => out.push((lo, hi)),
    }
}

/// Sorts and merges ranges.
pub fn normalize(mut ranges: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    ranges.sort_unstable();
    let mut out = Vec::with_capacity(ranges.len());
    for (lo, hi) in ranges {
        push_range(&mut out, lo, hi);
    }
    out
}

/// `a` minus `b`, both sorted and disjoint.
pub fn subtract(a: &[(u32, u32)], b: &[(u32, u32)]) -> Vec<(u32, u32)> {
    let mut out = Vec::new();
    let mut j = 0;
    for &(lo, hi) in a {
        let mut lo = lo;
        while j < b.len() && b[j].1 < lo {
            j += 1;
        }
        let mut k = j;
        while k < b.len() && b[k].0 <= hi {
            if b[k].0 > lo {
                out.push((lo, b[k].0 - 1));
            }
            if b[k].1 >= hi {
                lo = hi + 1;
                break;
            }
            lo = b[k].1 + 1;
            k += 1;
        }
        if lo <= hi {
            out.push((lo, hi));
        }
    }
    out
}

// ---- General_Category ------------------------------------------------------

/// The General_Category values a value name or alias of PropertyValueAliases.txt
/// stands for ("Lu", "Letter", "L", "punct", ...), as a mask over the leaf values.
pub fn general_category_mask(name: &str) -> Option<u32> {
    tables::GC_VALUE_ALIASES
        .binary_search_by(|(n, _)| (*n).cmp(name))
        .ok()
        .map(|i| tables::GC_VALUE_ALIASES[i].1)
}

/// The short name of `c`'s General_Category ("Lu", "Cn", ...).
pub fn general_category(c: u32) -> &'static str {
    match find_range(tables::GENERAL_CATEGORY, c) {
        Some(&i) => tables::GC_VALUES[i as usize],
        None => "Cn",
    }
}

/// The code points whose General_Category is in `mask`, as sorted ranges.
pub fn general_category_ranges(mask: u32) -> Vec<(u32, u32)> {
    let mut out = Vec::new();
    for &(lo, hi, i) in tables::GENERAL_CATEGORY {
        if mask & (1 << i) != 0 {
            push_range(&mut out, lo, hi);
        }
    }
    out
}

// ---- Script and Script_Extensions -------------------------------------------

/// A Script value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Script(u16);

/// A Script value by any of its names or aliases in PropertyValueAliases.txt.
pub fn script_by_name(name: &str) -> Option<Script> {
    tables::SCRIPT_VALUE_ALIASES
        .binary_search_by(|(n, _)| (*n).cmp(name))
        .ok()
        .map(|i| Script(tables::SCRIPT_VALUE_ALIASES[i].1))
}

/// The code points whose Script is `s` (code points Scripts.txt does not
/// list are Unknown, "Zzzz").
pub fn script_ranges(s: Script) -> Vec<(u32, u32)> {
    let mut out = Vec::new();
    for &(lo, hi, i) in tables::SCRIPT {
        if i == s.0 {
            push_range(&mut out, lo, hi);
        }
    }
    if tables::SCRIPT_VALUES[s.0 as usize] == "Zzzz" {
        let listed: Vec<(u32, u32)> = tables::SCRIPT.iter().map(|&(lo, hi, _)| (lo, hi)).collect();
        out.extend(subtract(&[(0, MAX_CODE_POINT)], &normalize(listed)));
        return normalize(out);
    }
    out
}

/// The code points whose Script_Extensions contain `s`. A code point that
/// ScriptExtensions.txt does not list has its Script as its only extension.
pub fn script_extensions_ranges(s: Script) -> Vec<(u32, u32)> {
    let explicit: Vec<(u32, u32)> = tables::SCRIPT_EXTENSIONS
        .iter()
        .map(|&(lo, hi, _)| (lo, hi))
        .collect();
    let mut out = subtract(&script_ranges(s), &normalize(explicit));
    for &(lo, hi, list) in tables::SCRIPT_EXTENSIONS {
        if list.contains(&s.0) {
            out.push((lo, hi));
        }
    }
    normalize(out)
}

// ---- Binary properties -----------------------------------------------------

/// A binary property (spec table-binary-unicode-properties), by name or alias.
pub fn binary_property(name: &str) -> Option<&'static [(u32, u32)]> {
    tables::BINARY_PROPERTIES
        .binary_search_by(|(n, _)| (*n).cmp(name))
        .ok()
        .map(|i| tables::BINARY_PROPERTIES[i].1)
}

/// The single code points of a binary property of strings.
pub type CodePoints = &'static [(u32, u32)];
/// The strings of two or more code points of a binary property of strings.
pub type Strings = &'static [&'static [u32]];

/// A binary property of strings (spec table-binary-unicode-properties-of-strings).
pub fn string_property(name: &str) -> Option<(CodePoints, Strings)> {
    tables::STRING_PROPERTIES
        .iter()
        .find(|(n, _, _)| *n == name)
        .map(|&(_, cps, strings)| (cps, strings))
}

// ---- Case ------------------------------------------------------------------

/// scf(c): the simple case folding of CaseFolding.txt (statuses C and S).
pub fn simple_case_fold(c: u32) -> u32 {
    match tables::SIMPLE_CASE_FOLDING.binary_search_by_key(&c, |&(from, _)| from) {
        Ok(i) => tables::SIMPLE_CASE_FOLDING[i].1,
        Err(_) => c,
    }
}

/// Every (c, scf(c)) pair with scf(c) ≠ c.
pub fn simple_case_foldings() -> impl Iterator<Item = (u32, u32)> {
    tables::SIMPLE_CASE_FOLDING.iter().copied()
}

fn full_mapping(table: &'static [(u32, [u32; 3])], c: u32) -> Option<&'static [u32]> {
    let i = table.binary_search_by_key(&c, |&(from, _)| from).ok()?;
    Some(trim(&table[i].1))
}

fn trim(m: &[u32; 3]) -> &[u32] {
    let len = m.iter().position(|&x| x == 0).unwrap_or(3);
    &m[..len]
}

/// The unconditional full uppercase mapping of `c` (UnicodeData.txt and
/// SpecialCasing.txt); `None` if `c` maps to itself.
pub fn to_uppercase(c: u32) -> Option<&'static [u32]> {
    full_mapping(tables::TO_UPPERCASE, c)
}

/// The unconditional full lowercase mapping of `c`; `None` if `c` maps to itself.
pub fn to_lowercase(c: u32) -> Option<&'static [u32]> {
    full_mapping(tables::TO_LOWERCASE, c)
}

/// Every code point with an uppercase mapping, with that mapping.
pub fn uppercase_mappings() -> impl Iterator<Item = (u32, &'static [u32])> {
    tables::TO_UPPERCASE.iter().map(|(c, m)| (*c, trim(m)))
}
