//! Abrupt completions that cross Rust function boundaries.
//!
//! `break`/`continue`/`return` never leave the bytecode interpreter, so the
//! only abrupt completion Rust code sees is a throw. `NotImplemented` is the
//! engine admitting something is missing: it is *not* catchable by
//! JavaScript, so a missing feature can never be mistaken for a passing test.

use crate::value::Value;

#[derive(Debug, Clone)]
pub enum JsError {
    /// A throw completion carrying the thrown value.
    Throw(Value),
    /// The engine does not implement what was asked. Uncatchable.
    NotImplemented(String),
}

/// The result of anything that can throw: Rust's `?` is the spec's `?`.
pub type JsResult<T> = Result<T, JsError>;
