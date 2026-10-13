//! Array.prototype.slice ( _start_, _end_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.slice

use kahvi_core::prelude::*;

/// Array.prototype.slice
pub(crate) fn prototype_slice(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method returns an array containing the elements of the array from element _start_ up to, but not including, element _end_ (or through the end of the array if _end_ is *undefined*). If _start_ is negative, it is treated as _length_ + _start_ where _length_ is the length of the array. If _end_ is negative, it is treated as _length_ + _end_ where _length_ is the length of the array.
    // It performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. Let _k_ be ? ToClampedIndex(_start_, _length_).
    // 4. If _end_ is *undefined*, let _final_ be _length_; else let _final_ be ? ToClampedIndex(_end_, _length_).
    // 5. Let _count_ be max(_final_ - _k_, 0).
    // 6. Let _array_ be ? ArraySpeciesCreate(_obj_, _count_).
    // 7. Let _resultIndex_ be 0.
    // 8. Repeat, while _k_ < _final_,
    //    a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //    b. Let _kPresent_ be ? HasProperty(_obj_, _propertyKey_).
    //    c. If _kPresent_ is *true*, then
    //       i. Let _kValue_ be ? Get(_obj_, _propertyKey_).
    //       ii. Perform ? CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_resultIndex_)), _kValue_).
    //    d. Set _k_ to _k_ + 1.
    //    e. Set _resultIndex_ to _resultIndex_ + 1.
    // 9. Perform ? Set(_array_, *"length"*, 𝔽(_resultIndex_), *true*).
    // 10. Return _array_.
    //
    // NOTE:
    // The explicit setting of the *"length"* property in step [step-array-proto-slice-set-length] is intended to ensure the length is correct even when _array_ is not a built-in Array.
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.slice"))
}
