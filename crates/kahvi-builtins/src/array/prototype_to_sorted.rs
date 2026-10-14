//! Array.prototype.toSorted ( _comparator_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.tosorted

use kahvi_core::prelude::*;

/// Array.prototype.toSorted
pub(crate) fn prototype_to_sorted(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. If _comparator_ is not *undefined* and IsCallable(_comparator_) is *false*, throw a *TypeError* exception.
    // 2. Let _obj_ be ? ToObject(*this* value).
    // 3. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 4. Let _array_ be ? ArrayCreate(_length_).
    // 5. Let _sortCompare_ be a new Abstract Closure with parameters (_x_, _y_) that captures _comparator_ and performs the following steps when called:
    //    a. Return ? CompareArrayElements(_x_, _y_, _comparator_).
    // 6. Let _sortedList_ be ? SortIndexedProperties(_obj_, _length_, _sortCompare_, ~read-through-holes~).
    // 7. Let _j_ be 0.
    // 8. Repeat, while _j_ < _length_,
    //    a. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_j_)), _sortedList_[_j_]).
    //    b. Set _j_ to _j_ + 1.
    // 9. Return _array_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.toSorted"))
}
