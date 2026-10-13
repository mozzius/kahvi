//! Array.prototype.includes ( _searchElement_ [ , _fromIndex_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.includes

use kahvi_core::prelude::*;

/// Array.prototype.includes
pub(crate) fn prototype_includes(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method compares _searchElement_ to the elements of the array, in ascending order, using the SameValueZero algorithm, and if found at any position, returns *true*; otherwise, it returns *false*.
    // The optional second argument _fromIndex_ defaults to *+0*_𝔽 (i.e. the whole array is searched). If it is greater than or equal to the length of the array, *false* is returned, i.e. the array will not be searched. If it is less than *-0*_𝔽, it is used as the offset from the end of the array to compute _fromIndex_. If the computed index is less than or equal to *+0*_𝔽, the whole array will be searched.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. If _length_ = 0, return *false*.
    // 4. Let _k_ be ? ToClampedIndex(_fromIndex_, _length_).
    // 5. Repeat, while _k_ < _length_,
    //    a. Let _elementK_ be ? Get(_obj_, ! ToString(𝔽(_k_))).
    //    b. If SameValueZero(_searchElement_, _elementK_) is *true*, return *true*.
    //    c. Set _k_ to _k_ + 1.
    // 6. Return *false*.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    // NOTE:
    // This method intentionally differs from the similar `indexOf` method in two ways. First, it uses the SameValueZero algorithm, instead of IsStrictlyEqual, allowing it to detect *NaN* array elements. Second, it does not skip missing array elements, instead treating them as *undefined*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.includes"))
}
