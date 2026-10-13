//! %TypedArray%.prototype.sort ( _comparator_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.sort

use kahvi_core::prelude::*;

/// TypedArray.prototype.sort
pub(crate) fn prototype_sort(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This is a distinct method that, except as described below, implements the same requirements as those of `Array.prototype.sort` as defined in [sec-array.prototype.sort]. The implementation of this method may be optimized with the knowledge that the *this* value is an object that has a fixed length and whose integer-indexed properties are not sparse.
    // This method is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.
    // It performs the following steps when called:
    //
    // 1. If _comparator_ is not *undefined* and IsCallable(_comparator_) is *false*, throw a *TypeError* exception.
    // 2. Let _obj_ be the *this* value.
    // 3. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 4. Let _length_ be TypedArrayLength(_taRecord_).
    // 5. NOTE: The following closure performs a numeric comparison rather than the string comparison used in [sec-array.prototype.sort].
    // 6. Let _sortCompare_ be a new Abstract Closure with parameters (_x_, _y_) that captures _comparator_ and performs the following steps when called:
    //    a. Return ? CompareTypedArrayElements(_x_, _y_, _comparator_).
    // 7. Let _sortedList_ be ? SortIndexedProperties(_obj_, _length_, _sortCompare_, ~read-through-holes~).
    // 8. Let _j_ be 0.
    // 9. Repeat, while _j_ < _length_,
    //    a. Perform ! TypedArraySetElement(_obj_, 𝔽(_j_), _sortedList_[_j_]).
    //    b. Set _j_ to _j_ + 1.
    // 10. Return _obj_.
    //
    // NOTE:
    // Because *NaN* always compares greater than any other value (see CompareTypedArrayElements), *NaN* property values always sort to the end of the result when _comparator_ is not provided.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.sort"))
}
