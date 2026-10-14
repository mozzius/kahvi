//! String iterators (§22.1.5): the state CreateStringIterator's closure keeps.
#![allow(dead_code)] // used by the String iterator tickets

use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;

/// The iterated string (None once exhausted) and the next code unit position.
#[derive(Debug)]
pub(crate) struct StringIterator {
    pub(crate) string: Option<JsString>,
    pub(crate) position: usize,
}

/// A new String Iterator object for `s`, with %StringIteratorPrototype%.
pub(crate) fn create_string_iterator(agent: &mut Agent, s: JsString) -> Object {
    let proto = agent.intrinsic("%StringIteratorPrototype%");
    let state = StringIterator {
        string: Some(s),
        position: 0,
    };
    agent.object_with_kind(Some(proto), ObjectKind::Native(Box::new(state)))
}

impl kahvi_core::gc::Trace for StringIterator {
    fn trace(&self, t: &mut kahvi_core::gc::Tracer) {
        self.string.trace(t);
    }
}
