//! %IntlSegmentIteratorPrototype%.next ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-%intlsegmentiteratorprototype%.next

use kahvi_core::prelude::*;

/// IntlSegmentIteratorPrototype.next
pub(crate) fn next(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The `next` method is called on a Segment Iterator instance to advance it forward one segment and return an IteratorResult object either describing the new segment or declaring iteration done. The following steps are taken:
    //
    // 1. Let _iterator_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_iterator_, [[IteratingSegmenter]]).
    // 3. Let _segmenter_ be _iterator_.[[IteratingSegmenter]].
    // 4. Let _string_ be _iterator_.[[IteratedString]].
    // 5. Let _startIndex_ be _iterator_.[[IteratedStringNextSegmentCodeUnitIndex]].
    // 6. Let _length_ be the length of _string_.
    // 7. If _startIndex_ ≥ _length_, then
    //    a. Return CreateIteratorResultObject(*undefined*, *true*).
    // 8. Let _endIndex_ be FindBoundary(_segmenter_, _string_, _startIndex_, ~after~).
    // 9. Set _iterator_.[[IteratedStringNextSegmentCodeUnitIndex]] to _endIndex_.
    // 10. Let _segmentData_ be CreateSegmentDataObject(_segmenter_, _string_, _startIndex_, _endIndex_).
    // 11. Return CreateIteratorResultObject(_segmentData_, *false*).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("IntlSegmentIteratorPrototype.next"))
}
