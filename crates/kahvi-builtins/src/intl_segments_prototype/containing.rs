//! %IntlSegmentsPrototype%.containing ( _index_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-%intlsegmentsprototype%.containing

use kahvi_core::prelude::*;

/// IntlSegmentsPrototype.containing
pub(crate) fn containing(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The `containing` method is called on a Segments instance with argument _index_ to return a Segment Data object describing the segment in the string including the code unit at the specified index according to the locale and options of the Segments instance's constructing Intl.Segmenter instance. The following steps are taken:
    //
    // 1. Let _segments_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_segments_, [[SegmentsSegmenter]]).
    // 3. Let _segmenter_ be _segments_.[[SegmentsSegmenter]].
    // 4. Let _string_ be _segments_.[[SegmentsString]].
    // 5. Let _length_ be the length of _string_.
    // 6. Let _n_ be ? ToIntegerOrInfinity(_index_).
    // 7. If _n_ < 0 or _n_ ≥ _length_, return *undefined*.
    // 8. Let _startIndex_ be FindBoundary(_segmenter_, _string_, _n_, ~before~).
    // 9. Let _endIndex_ be FindBoundary(_segmenter_, _string_, _n_, ~after~).
    // 10. Return CreateSegmentDataObject(_segmenter_, _string_, _startIndex_, _endIndex_).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("IntlSegmentsPrototype.containing"))
}
