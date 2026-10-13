//! Unicode character data, generated from the Unicode Character Database by
//! `harness/gen/unicode.py`.

mod tables;

pub use tables::UNICODE_VERSION;

fn in_ranges(ranges: &[(u32, u32)], c: u32) -> bool {
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
