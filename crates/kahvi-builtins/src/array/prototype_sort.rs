//! Array.prototype.sort ( _comparator_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.sort

use kahvi_core::prelude::*;

/// Array.prototype.sort
pub(crate) fn prototype_sort(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method sorts the elements of this array. If _comparator_ is not *undefined*, it should be a function that accepts two arguments _x_ and _y_ and returns a negative Number if _x_ < _y_, a positive Number if _x_ > _y_, or a zero otherwise.
    // It performs the following steps when called:
    //
    // 1. If _comparator_ is not *undefined* and IsCallable(_comparator_) is *false*, throw a *TypeError* exception.
    // 2. Let _obj_ be ? ToObject(*this* value).
    // 3. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 4. Let _sortCompare_ be a new Abstract Closure with parameters (_x_, _y_) that captures _comparator_ and performs the following steps when called:
    //    a. Return ? CompareArrayElements(_x_, _y_, _comparator_).
    // 5. Let _sortedList_ be ? SortIndexedProperties(_obj_, _length_, _sortCompare_, ~skip-holes~).
    // 6. Let _itemCount_ be the number of elements in _sortedList_.
    // 7. Let _j_ be 0.
    // 8. Repeat, while _j_ < _itemCount_,
    //    a. Perform ? Set(_obj_, ! ToString(𝔽(_j_)), _sortedList_[_j_], *true*).
    //    b. Set _j_ to _j_ + 1.
    // 9. NOTE: The call to SortIndexedProperties in step [step-array-sortindexedproperties] uses ~skip-holes~. The remaining indices are deleted to preserve the number of holes that were detected and excluded from the sort.
    // 10. Repeat, while _j_ < _length_,
    //    a. Perform ? DeletePropertyOrThrow(_obj_, ! ToString(𝔽(_j_))).
    //    b. Set _j_ to _j_ + 1.
    // 11. Return _obj_.
    //
    // NOTE:
    // Because non-existent property values always compare greater than *undefined* property values, and *undefined* always compares greater than any other value (see CompareArrayElements), *undefined* property values always sort to the end of the result, followed by non-existent property values.
    // NOTE:
    // Method calls performed by the ToString abstract operations in steps [step-sortcompare-tostring-x] and [step-sortcompare-tostring-y] have the potential to cause _sortCompare_ to not behave as a consistent comparator.
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore, it can be transferred to other kinds of objects for use as a method.
    // [subclause sec-sortindexedproperties: SortIndexedProperties (
    // _obj_: an Object,
    // _length_: a non-negative integer,
    // _sortCompare_: an Abstract Closure with two parameters,
    // _holes_: ~skip-holes~ or ~read-through-holes~,
    // ): either a normal completion containing a List of ECMAScript language values or a throw completion]
    // [subclause sec-comparearrayelements: CompareArrayElements (
    // _x_: an ECMAScript language value,
    // _y_: an ECMAScript language value,
    // _comparator_: a function object or *undefined*,
    // ): either a normal completion containing a Number or an abrupt completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.sort"))
}
