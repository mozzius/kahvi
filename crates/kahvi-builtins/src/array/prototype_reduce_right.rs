//! Array.prototype.reduceRight ( _callback_ [ , _initialValue_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.reduceright

use kahvi_core::prelude::*;

/// Array.prototype.reduceRight
pub(crate) fn prototype_reduce_right(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // _callback_ should be a function that takes four arguments. `reduceRight` calls _callback_ once for each element present in the array, in descending order, skipping the first call unless _initialValue_ is provided.
    // _callback_ is called with four arguments: the _previousValue_ (value from the previous call to _callback_), the _currentValue_ (value of the current element), the _currentIndex_, and the object being traversed. The first time the function is called, the _previousValue_ and _currentValue_ can be one of two values. If an _initialValue_ was supplied in the call to `reduceRight`, then _previousValue_ will be _initialValue_ and _currentValue_ will be the last value in the array. If no _initialValue_ was supplied, then _previousValue_ will be the last value in the array and _currentValue_ will be the second-to-last value. It is a *TypeError* if the array contains no elements and _initialValue_ is not provided.
    // `reduceRight` does not directly mutate the object on which it is called but the object may be mutated by the calls to _callback_.
    // The range of elements processed by `reduceRight` is set before the first call to _callback_. Elements that are appended to the array after the call to `reduceRight` begins will not be visited by _callback_. If existing elements of the array are changed by _callback_, their value as passed to _callback_ will be the value at the time `reduceRight` visits them; elements that are deleted after the call to `reduceRight` begins and before being visited are not visited.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    // 4. If _length_ = 0 and _initialValue_ is not present, throw a *TypeError* exception.
    // 5. Let _k_ be _length_ - 1.
    // 6. Let _accumulator_ be *undefined*.
    // 7. If _initialValue_ is present, then
    //    a. Set _accumulator_ to _initialValue_.
    // 8. Else,
    //    a. Let _kPresent_ be *false*.
    //    b. Repeat, while _kPresent_ is *false* and _k_ ≥ 0,
    //       i. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //       ii. Set _kPresent_ to ? HasProperty(_obj_, _propertyKey_).
    //       iii. If _kPresent_ is *true*, then
    //          1. Set _accumulator_ to ? Get(_obj_, _propertyKey_).
    //       iv. Set _k_ to _k_ - 1.
    //    c. If _kPresent_ is *false*, throw a *TypeError* exception.
    // 9. Repeat, while _k_ ≥ 0,
    //    a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //    b. Let _kPresent_ be ? HasProperty(_obj_, _propertyKey_).
    //    c. If _kPresent_ is *true*, then
    //       i. Let _kValue_ be ? Get(_obj_, _propertyKey_).
    //       ii. Set _accumulator_ to ? Call(_callback_, *undefined*, « _accumulator_, _kValue_, 𝔽(_k_), _obj_ »).
    //    d. Set _k_ to _k_ - 1.
    // 10. Return _accumulator_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.reduceRight"))
}
