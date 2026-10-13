//! Array iterators (§23.1.5): CreateArrayIterator and the state that
//! %ArrayIteratorPrototype%.next advances.

use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IterationKind {
    Key,
    Value,
    KeyValue,
}

/// The state of an Array Iterator: [[IteratedArrayLike]] (None once
/// exhausted), [[ArrayLikeNextIndex]] and [[ArrayLikeIterationKind]].
#[derive(Debug)]
pub(crate) struct ArrayIterator {
    pub(crate) iterated: Option<Object>,
    pub(crate) next_index: f64,
    pub(crate) kind: IterationKind,
}

/// CreateArrayIterator(array, kind) (§23.1.5.1)
pub(crate) fn create_array_iterator(
    agent: &mut Agent,
    array: Object,
    kind: IterationKind,
) -> Object {
    let proto = agent.intrinsic("%ArrayIteratorPrototype%");
    let state = ArrayIterator {
        iterated: Some(array),
        next_index: 0.0,
        kind,
    };
    agent.object_with_kind(Some(proto), ObjectKind::Native(Box::new(state)))
}
