//! Array.prototype.filter ( _callback_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.filter

use kahvi_core::prelude::*;

/// Array.prototype.filter
pub(crate) fn prototype_filter(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // _callback_ should be a function that accepts three arguments and returns a value that is coercible to a Boolean value. `filter` calls _callback_ once for each element in the array, in ascending order, and constructs a new array of all the values for which _callback_ returns *true*. _callback_ is called only for elements of the array which actually exist; it is not called for missing elements of the array.
    // If a _thisArg_ parameter is provided, it will be used as the *this* value for each invocation of _callback_. If it is not provided, *undefined* is used instead.
    // _callback_ is called with three arguments: the value of the element, the index of the element, and the object being traversed.
    // `filter` does not directly mutate the object on which it is called but the object may be mutated by the calls to _callback_.
    // The range of elements processed by `filter` is set before the first call to _callback_. Elements which are appended to the array after the call to `filter` begins will not be visited by _callback_. If existing elements of the array are changed their value as passed to _callback_ will be the value at the time `filter` visits them; elements that are deleted after the call to `filter` begins and before being visited are not visited.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    // 4. Let _array_ be ? ArraySpeciesCreate(_obj_, 0).
    // 5. Let _k_ be 0.
    // 6. Let _to_ be 0.
    // 7. Repeat, while _k_ < _length_,
    //    a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //    b. Let _kPresent_ be ? HasProperty(_obj_, _propertyKey_).
    //    c. If _kPresent_ is *true*, then
    //       i. Let _kValue_ be ? Get(_obj_, _propertyKey_).
    //       ii. Let _selected_ be ToBoolean(? Call(_callback_, _thisArg_, « _kValue_, 𝔽(_k_), _obj_ »)).
    //       iii. If _selected_ is *true*, then
    //          1. Perform ? CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_to_)), _kValue_).
    //          2. Set _to_ to _to_ + 1.
    //    d. Set _k_ to _k_ + 1.
    // 8. Return _array_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.filter"))
}
