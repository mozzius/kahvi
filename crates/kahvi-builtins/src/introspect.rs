//! Read-only views of built-in objects' internal slots for hosts (a
//! console's value formatter). Nothing here runs JavaScript.

use crate::support::collections::{MapData, SetData};
use crate::support::date::DateValue;
use crate::support::regexp::RegExpData;
use kahvi_core::prelude::*;

/// What kind of built-in object `o` is, by its internal slots.
#[derive(Debug, Clone)]
pub enum Kind {
    /// [[MapData]]: the live entries, in order.
    Map(Vec<(Value, Value)>),
    /// [[SetData]]: the live values, in order.
    Set(Vec<Value>),
    /// [[DateValue]]: the time value.
    Date(f64),
    /// [[RegExpMatcher]]: [[OriginalSource]] and [[OriginalFlags]].
    RegExp(JsString, JsString),
    Other,
}

pub fn kind(agent: &Agent, o: Object) -> Kind {
    if let Some(m) = agent.native::<MapData>(o) {
        let t = &m.0;
        return Kind::Map((0..t.slots()).filter_map(|i| t.entry(i)).collect());
    }
    if let Some(s) = agent.native::<SetData>(o) {
        let t = &s.0;
        return Kind::Set(
            (0..t.slots())
                .filter_map(|i| t.entry(i).map(|(k, _)| k))
                .collect(),
        );
    }
    if let Some(d) = agent.native::<DateValue>(o) {
        return Kind::Date(d.0);
    }
    if let Some(r) = agent.native::<RegExpData>(o) {
        return Kind::RegExp(r.original_source, r.original_flags);
    }
    Kind::Other
}
