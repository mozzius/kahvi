//! Abstract operations (ECMA-262 §7) as free functions taking `&mut Agent`.

pub mod bigint;
pub mod conversion;
pub mod iteration;
pub mod object;
pub mod promise;
pub mod testing;

pub use conversion::*;
pub use iteration::*;
pub use object::*;
pub use testing::*;

use crate::agent::Agent;
use crate::value::Object;

/// Marker for the [[RegExpMatcher]] internal slot. The RegExp builtin stores
/// its compiled matcher in a type that registers itself here.
pub type RegExpMatcherCheck = fn(&Agent, Object) -> bool;

/// Whether `o` has a [[RegExpMatcher]] internal slot.
pub fn has_regexp_matcher(agent: &Agent, o: Object) -> bool {
    match agent.regexp_matcher_check {
        Some(check) => check(agent, o),
        None => false,
    }
}
