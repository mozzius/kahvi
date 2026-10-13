//! Array.prototype.indexOf ( _searchElement_ [ , _fromIndex_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.indexof

use kahvi_core::prelude::*;

/// Array.prototype.indexOf
pub(crate) fn prototype_index_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method compares _searchElement_ to the elements of the array, in ascending order, using the IsStrictlyEqual algorithm, and if found at one or more indices, returns the smallest such index; otherwise, it returns *-1*_𝔽.
    // NOTE:
    // The optional second argument _fromIndex_ defaults to *+0*_𝔽 (i.e. the whole array is searched). If it is greater than or equal to the length of the array, *-1*_𝔽 is returned, i.e. the array will not be searched. If it is less than *-0*_𝔽, it is used to compute an offset from the end of the array. If the computed index is less than or equal to *+0*_𝔽, the whole array will be searched.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. If _length_ = 0, return *-1*_𝔽.
    // 4. Let _k_ be ? ToClampedIndex(_fromIndex_, _length_).
    // 5. Repeat, while _k_ < _length_,
    //    a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //    b. Let _kPresent_ be ? HasProperty(_obj_, _propertyKey_).
    //    c. If _kPresent_ is *true*, then
    //       i. Let _elementK_ be ? Get(_obj_, _propertyKey_).
    //       ii. If IsStrictlyEqual(_searchElement_, _elementK_) is *true*, return 𝔽(_k_).
    //    d. Set _k_ to _k_ + 1.
    // 6. Return *-1*_𝔽.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.indexOf"))
}
