//! Regular expressions (ECMA-262 §22.2): pattern parsing with early errors,
//! and a backtracking matcher over UTF-16 input.
//!
//! The engine-facing RegExp objects live in the engine; this crate knows
//! only patterns, flags and strings of code units.

mod charset;
mod compiler;
mod parser;
mod vm;

use charset::Rer;

/// A pattern SyntaxError.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

/// The flags that affect parsing and matching (`d`, `g` and `y` only
/// affect RegExpBuiltinExec).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Flags {
    pub ignore_case: bool,
    pub multiline: bool,
    pub dot_all: bool,
    pub unicode: bool,
    pub unicode_sets: bool,
}

/// A compiled pattern: the [[RegExpMatcher]] and the parts of the
/// [[RegExpRecord]] that RegExpBuiltinExec needs.
pub struct Regex {
    program: compiler::Program,
    group_count: usize,
    group_names: Vec<(String, Vec<usize>)>,
}

/// The pattern's characters: code points with either Unicode flag (a lone
/// surrogate stays a code point of its own), code units otherwise.
fn pattern_chars(pattern: &[u16], unicode: bool) -> Vec<u32> {
    if !unicode {
        return pattern.iter().map(|&u| u as u32).collect();
    }
    char::decode_utf16(pattern.iter().copied())
        .map(|r| match r {
            Ok(c) => c as u32,
            Err(e) => e.unpaired_surrogate() as u32,
        })
        .collect()
}

/// ParsePattern and CompilePattern: a SyntaxError if the pattern is invalid.
pub fn compile(pattern: &[u16], flags: Flags) -> Result<Regex, Error> {
    let rer = Rer {
        ignore_case: flags.ignore_case,
        multiline: flags.multiline,
        dot_all: flags.dot_all,
        unicode: flags.unicode,
        unicode_sets: flags.unicode_sets,
    };
    let chars = pattern_chars(pattern, rer.either_unicode());
    let parsed = parser::parse_pattern(&chars, rer)?;
    let program = compiler::compile(&parsed.node, rer, parsed.group_count);
    Ok(Regex {
        program,
        group_count: parsed.group_count,
        group_names: parsed.group_names,
    })
}

/// Only the early errors: whether `pattern` parses with these flags.
pub fn validate(pattern: &[u16], flags: Flags) -> Result<(), Error> {
    let rer = Rer {
        ignore_case: flags.ignore_case,
        multiline: flags.multiline,
        dot_all: flags.dot_all,
        unicode: flags.unicode,
        unicode_sets: flags.unicode_sets,
    };
    parser::parse_pattern(&pattern_chars(pattern, rer.either_unicode()), rer).map(|_| ())
}

/// The span of the whole match and of each group (`None` for a group that
/// did not participate).
pub type Captures = Vec<Option<(usize, usize)>>;

impl Regex {
    /// [[RegExpRecord]].[[CapturingGroupsCount]]
    pub fn group_count(&self) -> usize {
        self.group_count
    }

    /// Each distinct group name with the indices of the groups that have it
    /// (several only for duplicate named groups), in order of appearance.
    pub fn group_names(&self) -> &[(String, Vec<usize>)] {
        &self.group_names
    }

    /// The matcher applied at `index`: on success, the span of the whole
    /// match and of each group (`None` for a group that did not participate).
    pub fn match_at(&self, input: &[u16], index: usize) -> Option<Vec<Option<(usize, usize)>>> {
        if index > input.len() {
            return None;
        }
        self.search(input, index, None).map(|(_, caps)| caps)
    }

    /// The matcher applied at `index` and then, until it succeeds, at each
    /// position after it that RegExpBuiltinExec's loop would try (the next
    /// code unit, or the next code point when `full_unicode`), up to the
    /// end of the input: the position it succeeded at and the spans.
    pub fn search_from(
        &self,
        input: &[u16],
        index: usize,
        full_unicode: bool,
    ) -> Option<(usize, Captures)> {
        self.search(input, index, Some(full_unicode))
    }

    /// One start position (`advance` is `None`), or a search.
    fn search(
        &self,
        input: &[u16],
        mut index: usize,
        advance: Option<bool>,
    ) -> Option<(usize, Captures)> {
        thread_local! {
            static SCRATCH: std::cell::Cell<Option<vm::Scratch>> = const { std::cell::Cell::new(None) };
        }
        let scratch = SCRATCH.take().unwrap_or_default();
        let mut m = vm::Matcher::with_scratch(&self.program, input, scratch);
        let found = loop {
            if index > input.len() {
                break None;
            }
            if m.match_at(index) {
                break Some(index);
            }
            match advance {
                None => break None,
                // AdvanceStringIndex: a surrogate pair is one step in Unicode mode.
                Some(full_unicode) => {
                    let pair = full_unicode
                        && index + 1 < input.len()
                        && (0xD800..=0xDBFF).contains(&input[index])
                        && (0xDC00..=0xDFFF).contains(&input[index + 1]);
                    index += if pair { 2 } else { 1 };
                }
            }
        };
        let result = found.map(|at| {
            let caps = m
                .caps
                .chunks(2)
                .map(|c| (c[0] != usize::MAX && c[1] != usize::MAX).then_some((c[0], c[1])))
                .collect();
            (at, caps)
        });
        SCRATCH.set(Some(m.into_scratch()));
        result
    }
}
