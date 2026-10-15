//! Intl.Segmenter, Segments and Segment Iterators: the [[InitializedSegmenter]] internal slots and the
//! abstract operations shared by its built-ins.
//!
//! The ticket for this service owns this file: define the internal-slot
//! struct here (`ObjectKind::Native`, see docs/INTL.md) and the abstract
//! operations more than one of its built-ins needs.
#![allow(dead_code)] // helpers for the Intl tickets

use std::sync::Arc;

use icu::locale::Locale;
use icu::segmenter::options::{SentenceBreakOptions, WordBreakOptions};
use icu::segmenter::{GraphemeClusterSegmenter, SentenceSegmenter, WordSegmenter};
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
    /// The boundaries of [[SegmentsString]], computed once (see `boundaries`).
    pub bounds: Arc<Vec<(usize, bool)>>,
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
    /// The boundaries of [[IteratedString]], shared with the Segments instance.
    pub bounds: Arc<Vec<(usize, bool)>>,
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
) -> JsResult<Object> {
    let bounds = Arc::new(boundaries(agent, segmenter, string)?);
    // 1. Let _internalSlotsList_ be « [[SegmentsSegmenter]], [[SegmentsString]] ».
    // 2. Let _segments_ be OrdinaryObjectCreate(%IntlSegmentsPrototype%, _internalSlotsList_).
    // 3. Set _segments_.[[SegmentsSegmenter]] to _segmenter_.
    // 4. Set _segments_.[[SegmentsString]] to _string_.
    let proto = agent.intrinsic("%IntlSegmentsPrototype%");
    // 5. Return _segments_.
    Ok(agent.object_with_kind(
        Some(proto),
        ObjectKind::Native(Box::new(SegmentsSlots {
            segmenter,
            string,
            bounds,
        })),
    ))
}

/// The segmentation boundaries of `string` for `granularity`, ascending, as
/// code unit indices paired with whether the segment they end is word-like
/// (only meaningful for "word"). The first entry is the boundary at 0.
///
/// The word and sentence segmenters are built for [[Locale]]. ICU4X has no
/// locale-specific grapheme cluster data, so that segmenter is locale-independent.
/// For "word", the flag is that of the segment ending at the boundary.
fn boundaries(
    agent: &mut Agent,
    segmenter: Object,
    string: JsString,
) -> JsResult<Vec<(usize, bool)>> {
    let Some((locale, granularity)) = agent
        .native::<SegmenterSlots>(segmenter)
        .map(|s| (s.locale.clone(), s.granularity.clone()))
    else {
        return Err(agent.not_implemented("Intl.Segmenter: segmenter has no internal slots"));
    };
    let Ok(locale) = locale.parse::<Locale>() else {
        return Err(agent.not_implemented("Intl.Segmenter: unparseable [[Locale]]"));
    };
    let units = agent.str(string);
    match granularity.as_str() {
        "word" => {
            let mut options = WordBreakOptions::default();
            options.content_locale = Some(&locale.id);
            match WordSegmenter::try_new_auto(options) {
                Ok(s) => Ok(s
                    .as_borrowed()
                    .segment_utf16(units)
                    .iter_with_word_type()
                    .map(|(i, t)| (i, t.is_word_like()))
                    .collect()),
                Err(_) => Err(agent.not_implemented("Intl.Segmenter: word break data")),
            }
        }
        "sentence" => {
            let mut options = SentenceBreakOptions::default();
            options.content_locale = Some(&locale.id);
            match SentenceSegmenter::try_new(options) {
                Ok(s) => Ok(s
                    .as_borrowed()
                    .segment_utf16(units)
                    .map(|i| (i, false))
                    .collect()),
                Err(_) => Err(agent.not_implemented("Intl.Segmenter: sentence break data")),
            }
        }
        "grapheme" => Ok(GraphemeClusterSegmenter::new()
            .segment_utf16(units)
            .map(|i| (i, false))
            .collect()),
        _ => Err(agent.not_implemented("Intl.Segmenter: unknown [[SegmenterGranularity]]")),
    }
}

/// FindBoundary ( _segmenter_, _string_, _startIndex_, _direction_ )
///
/// `bounds` is the boundary list of _string_ for _segmenter_ (see `boundaries`).
pub(crate) fn find_boundary(
    agent: &Agent,
    bounds: &[(usize, bool)],
    string: JsString,
    start_index: usize,
    after: bool,
) -> usize {
    // 1. Let _length_ be the length of _string_.
    let length = agent.str_len(string);
    // 2. Assert: _startIndex_ < _length_.
    // 3. Let _locale_ be _segmenter_.[[Locale]].
    // 4. Let _granularity_ be _segmenter_.[[SegmenterGranularity]].
    if !after {
        // 5. If _direction_ is ~before~, then
        //    a. Search _string_ for the last segmentation boundary that is preceded by at most _startIndex_ code units from the beginning, using locale _locale_ and text element granularity _granularity_.
        //    b. If a boundary is found, return the count of code units in _string_ preceding it.
        if let Some((i, _)) = bounds.iter().rev().find(|(i, _)| *i <= start_index) {
            return *i;
        }
        //    c. Return 0.
        return 0;
    }
    // 6. Assert: _direction_ is ~after~.
    // 7. Search _string_ for the first segmentation boundary that follows the code unit at index _startIndex_, using locale _locale_ and text element granularity _granularity_.
    // 8. If a boundary is found, return the count of code units in _string_ preceding it.
    if let Some((i, _)) = bounds.iter().find(|(i, _)| *i > start_index) {
        return *i;
    }
    // 9. Return _length_.
    length
}

/// CreateSegmentDataObject ( _segmenter_, _string_, _startIndex_, _endIndex_ )
pub(crate) fn create_segment_data_object(
    agent: &mut Agent,
    segmenter: Object,
    bounds: &[(usize, bool)],
    string: JsString,
    start_index: usize,
    end_index: usize,
) -> JsResult<Object> {
    // 1. Let _length_ be the length of _string_.
    // 2. Assert: _endIndex_ ≤ _length_.
    // 3. Assert: _startIndex_ < _endIndex_.
    // 4. Let _result_ be OrdinaryObjectCreate(%Object.prototype%).
    let proto = agent.intrinsic("%Object.prototype%");
    let result = agent.ordinary_object_create(Some(proto));
    // 5. Let _segment_ be the substring of _string_ from _startIndex_ to _endIndex_.
    let segment = agent.substring(string, start_index, end_index);
    // 6. Perform ! CreateDataPropertyOrThrow(_result_, *"segment"*, _segment_).
    let key = agent.key("segment");
    ops::create_data_property_or_throw(agent, result, key, Value::String(segment))?;
    // 7. Perform ! CreateDataPropertyOrThrow(_result_, *"index"*, 𝔽(_startIndex_)).
    let key = agent.key("index");
    ops::create_data_property_or_throw(agent, result, key, Value::Number(start_index as f64))?;
    // 8. Perform ! CreateDataPropertyOrThrow(_result_, *"input"*, _string_).
    let key = agent.key("input");
    ops::create_data_property_or_throw(agent, result, key, Value::String(string))?;
    // 9. Let _granularity_ be _segmenter_.[[SegmenterGranularity]].
    let Some(granularity) = agent
        .native::<SegmenterSlots>(segmenter)
        .map(|s| s.granularity.clone())
    else {
        return Err(agent.not_implemented("Intl.Segmenter: segmenter has no internal slots"));
    };
    // 10. If _granularity_ is *"word"*, then
    if granularity == "word" {
        //    a. Let _isWordLike_ be a Boolean indicating whether the _segment_ in _string_ is "word-like" according to locale _segmenter_.[[Locale]].
        let is_word_like = bounds.iter().any(|(i, w)| *i == end_index && *w);
        //    b. Perform ! CreateDataPropertyOrThrow(_result_, *"isWordLike"*, _isWordLike_).
        let key = agent.key("isWordLike");
        ops::create_data_property_or_throw(agent, result, key, Value::Boolean(is_word_like))?;
    }
    // 11. Return _result_.
    Ok(result)
}

/// CreateSegmentIterator ( _segmenter_, _string_ )
pub(crate) fn create_segment_iterator(
    agent: &mut Agent,
    segmenter: Object,
    string: JsString,
    bounds: Arc<Vec<(usize, bool)>>,
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
            bounds,
        })),
    )
}
