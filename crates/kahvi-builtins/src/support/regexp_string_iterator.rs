//! RegExp String Iterator objects (§22.2.9): CreateRegExpStringIterator and
//! the internal slots of its instances.

use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;

/// The [[IteratingRegExp]], [[IteratedString]], [[Global]], [[Unicode]] and
/// [[Done]] internal slots (§22.2.9.3).
#[derive(Debug)]
pub(crate) struct RegExpStringIterator {
    pub(crate) iterating_regexp: Object,
    pub(crate) iterated_string: JsString,
    pub(crate) global: bool,
    pub(crate) unicode: bool,
    pub(crate) done: bool,
}

impl kahvi_core::gc::Trace for RegExpStringIterator {
    fn trace(&self, t: &mut kahvi_core::gc::Tracer) {
        self.iterating_regexp.trace(t);
        self.iterated_string.trace(t);
    }
}

/// CreateRegExpStringIterator ( _regexp_, _string_, _global_, _fullUnicode_ ) (§22.2.9.1)
pub(crate) fn create_regexp_string_iterator(
    agent: &mut Agent,
    regexp: Object,
    string: JsString,
    global: bool,
    full_unicode: bool,
) -> Object {
    // 1. Let iterator be OrdinaryObjectCreate(%RegExpStringIteratorPrototype%, « [[IteratingRegExp]], [[IteratedString]], [[Global]], [[Unicode]], [[Done]] »).
    let proto = agent.intrinsic("%RegExpStringIteratorPrototype%");
    let slots = RegExpStringIterator {
        // 2. Set iterator.[[IteratingRegExp]] to regexp.
        iterating_regexp: regexp,
        // 3. Set iterator.[[IteratedString]] to string.
        iterated_string: string,
        // 4. Set iterator.[[Global]] to global.
        global,
        // 5. Set iterator.[[Unicode]] to fullUnicode.
        unicode: full_unicode,
        // 6. Set iterator.[[Done]] to false.
        done: false,
    };
    // 7. Return iterator.
    agent.object_with_kind(Some(proto), ObjectKind::Native(Box::new(slots)))
}
