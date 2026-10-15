//! %IntlSegmentsPrototype% [ %Symbol.iterator% ] ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-%intlsegmentsprototype%-%symbol.iterator%

use kahvi_core::prelude::*;

use crate::support::intl_segmenter::{SegmentsSlots, create_segment_iterator};

/// IntlSegmentsPrototype[Symbol.iterator]
pub(crate) fn symbol_iterator(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The `%Symbol.iterator%` method is called on a Segments instance to create a Segment Iterator over its string using the locale and options of its constructing Intl.Segmenter instance. The following steps are taken:
    //
    // The value of the *"name"* property of this function is *"[Symbol.iterator]"*.
    let _ = (args, new_target);
    // 1. Let _segments_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_segments_, [[SegmentsSegmenter]]).
    let Some(slots) = (match this {
        Value::Object(o) => agent.native::<SegmentsSlots>(o),
        _ => None,
    }) else {
        return Err(agent
            .throw_type_error("%Segments%[Symbol.iterator] called on an incompatible receiver"));
    };
    // 3. Let _segmenter_ be _segments_.[[SegmentsSegmenter]].
    let segmenter = slots.segmenter;
    // 4. Let _string_ be _segments_.[[SegmentsString]].
    let string = slots.string;
    let bounds = slots.bounds.clone();
    // 5. Return CreateSegmentIterator(_segmenter_, _string_).
    Ok(Value::Object(create_segment_iterator(
        agent, segmenter, string, bounds,
    )))
}
