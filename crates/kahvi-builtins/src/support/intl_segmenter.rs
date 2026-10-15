//! Intl.Segmenter, Segments and Segment Iterators: the [[InitializedSegmenter]] internal slots and the
//! abstract operations shared by its built-ins.
//!
//! The ticket for this service owns this file: define the internal-slot
//! struct here (`ObjectKind::Native`, see docs/INTL.md) and the abstract
//! operations more than one of its built-ins needs.
#![allow(dead_code)] // helpers for the Intl tickets

use kahvi_core::gc::{Trace, Tracer};
use kahvi_core::prelude::*;

/// The internal slots of an Intl.Segmenter: [[InitializedSegmenter]],
/// [[Locale]], [[SegmenterGranularity]].
#[derive(Debug, Clone, Default)]
pub(crate) struct SegmenterSlots {
    /// [[Locale]]
    pub locale: String,
    /// [[SegmenterGranularity]]: "grapheme", "word" or "sentence".
    pub granularity: String,
}

impl Trace for SegmenterSlots {
    fn trace(&self, _: &mut Tracer) {}
}

/// The internal slots of a Segments instance.
#[derive(Debug)]
pub(crate) struct SegmentsSlots {
    /// [[SegmentsSegmenter]]
    pub segmenter: Object,
    /// [[SegmentsString]]
    pub string: JsString,
}

impl Trace for SegmentsSlots {
    fn trace(&self, t: &mut Tracer) {
        self.segmenter.trace(t);
        self.string.trace(t);
    }
}

/// The internal slots of a Segment Iterator.
#[derive(Debug)]
pub(crate) struct SegmentIteratorSlots {
    /// [[IteratingSegmenter]]
    pub segmenter: Object,
    /// [[IteratedString]]
    pub string: JsString,
    /// [[IteratedStringNextSegmentCodeUnitIndex]]
    pub next_index: usize,
}

impl Trace for SegmentIteratorSlots {
    fn trace(&self, t: &mut Tracer) {
        self.segmenter.trace(t);
        self.string.trace(t);
    }
}

/// CreateSegmentsObject ( _segmenter_, _string_ )
pub(crate) fn create_segments_object(
    agent: &mut Agent,
    segmenter: Object,
    string: JsString,
) -> Object {
    // 1. Let _internalSlotsList_ be « [[SegmentsSegmenter]], [[SegmentsString]] ».
    // 2. Let _segments_ be OrdinaryObjectCreate(%IntlSegmentsPrototype%, _internalSlotsList_).
    // 3. Set _segments_.[[SegmentsSegmenter]] to _segmenter_.
    // 4. Set _segments_.[[SegmentsString]] to _string_.
    let proto = agent.intrinsic("%IntlSegmentsPrototype%");
    // 5. Return _segments_.
    agent.object_with_kind(
        Some(proto),
        ObjectKind::Native(Box::new(SegmentsSlots { segmenter, string })),
    )
}

/// CreateSegmentIterator ( _segmenter_, _string_ )
pub(crate) fn create_segment_iterator(
    agent: &mut Agent,
    segmenter: Object,
    string: JsString,
) -> Object {
    // 1. Let _internalSlotsList_ be « [[IteratingSegmenter]], [[IteratedString]], [[IteratedStringNextSegmentCodeUnitIndex]] ».
    // 2. Let _iterator_ be OrdinaryObjectCreate(%IntlSegmentIteratorPrototype%, _internalSlotsList_).
    // 3. Set _iterator_.[[IteratingSegmenter]] to _segmenter_.
    // 4. Set _iterator_.[[IteratedString]] to _string_.
    // 5. Set _iterator_.[[IteratedStringNextSegmentCodeUnitIndex]] to 0.
    let proto = agent.intrinsic("%IntlSegmentIteratorPrototype%");
    // 6. Return _iterator_.
    agent.object_with_kind(
        Some(proto),
        ObjectKind::Native(Box::new(SegmentIteratorSlots {
            segmenter,
            string,
            next_index: 0,
        })),
    )
}
