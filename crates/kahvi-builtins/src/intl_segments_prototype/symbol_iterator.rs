//! %IntlSegmentsPrototype% [ %Symbol.iterator% ] ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-%intlsegmentsprototype%-%symbol.iterator%

use kahvi_core::prelude::*;

/// IntlSegmentsPrototype[Symbol.iterator]
pub(crate) fn symbol_iterator(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The `%Symbol.iterator%` method is called on a Segments instance to create a Segment Iterator over its string using the locale and options of its constructing Intl.Segmenter instance. The following steps are taken:
    //
    // 1. Let _segments_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_segments_, [[SegmentsSegmenter]]).
    // 3. Let _segmenter_ be _segments_.[[SegmentsSegmenter]].
    // 4. Let _string_ be _segments_.[[SegmentsString]].
    // 5. Return CreateSegmentIterator(_segmenter_, _string_).
    //
    // The value of the *"name"* property of this function is *"[Symbol.iterator]"*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("IntlSegmentsPrototype[Symbol.iterator]"))
}
