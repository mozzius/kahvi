//! ECMAScript parser: source text to [`ast`], with early errors.
//!
//! The grammar and early errors follow ECMA-262 §12–16. The lexer is driven
//! by the parser because tokenisation depends on grammatical context (regular
//! expressions vs division, template continuations).

pub mod ast;
pub mod early;
pub mod lexer;
pub mod number;
mod parser;

pub use parser::{ParseOptions, parse_dynamic_function, parse_module, parse_script};

/// An early (parse-time) error. `pos` is a byte offset into the source.
#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxError {
    pub message: String,
    pub pos: usize,
}

impl SyntaxError {
    pub fn new(message: impl Into<String>, pos: usize) -> SyntaxError {
        SyntaxError {
            message: message.into(),
            pos,
        }
    }

    /// 1-based line and column of the error in `source`.
    pub fn line_col(&self, source: &str) -> (usize, usize) {
        let upto = &source[..self.pos.min(source.len())];
        let line = upto.matches('\n').count() + 1;
        let col = upto.rsplit('\n').next().map_or(0, |l| l.chars().count()) + 1;
        (line, col)
    }
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

/// Words that are always reserved (ReservedWord minus the contextual ones).
pub const RESERVED_WORDS: &[&str] = &[
    "await",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "enum",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "function",
    "if",
    "import",
    "in",
    "instanceof",
    "new",
    "null",
    "return",
    "super",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "var",
    "void",
    "while",
    "with",
    "yield",
];

/// Additionally reserved in strict mode code.
pub const STRICT_RESERVED_WORDS: &[&str] = &[
    "implements",
    "interface",
    "let",
    "package",
    "private",
    "protected",
    "public",
    "static",
    "yield",
];
