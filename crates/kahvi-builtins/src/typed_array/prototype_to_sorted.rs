//! %TypedArray%.prototype.toSorted ( _comparator_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.tosorted

use kahvi_core::prelude::*;

/// TypedArray.prototype.toSorted
pub(crate) fn prototype_to_sorted(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. If _comparator_ is not *undefined* and IsCallable(_comparator_) is *false*, throw a *TypeError* exception.
    // 2. Let _obj_ be the *this* value.
    // 3. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 4. Let _length_ be TypedArrayLength(_taRecord_).
    // 5. Let _resultArray_ be ? TypedArrayCreateSameType(_obj_, _length_).
    // 6. NOTE: The following closure performs a numeric comparison rather than the string comparison used in [sec-array.prototype.tosorted].
    // 7. Let _sortCompare_ be a new Abstract Closure with parameters (_x_, _y_) that captures _comparator_ and performs the following steps when called:
    //    a. Return ? CompareTypedArrayElements(_x_, _y_, _comparator_).
    // 8. Let _sortedList_ be ? SortIndexedProperties(_obj_, _length_, _sortCompare_, ~read-through-holes~).
    // 9. Let _j_ be 0.
    // 10. Repeat, while _j_ < _length_,
    //    a. Perform ! TypedArraySetElement(_resultArray_, 𝔽(_j_), _sortedList_[_j_]).
    //    b. Set _j_ to _j_ + 1.
    // 11. Return _resultArray_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.toSorted"))
}
