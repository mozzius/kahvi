//! Array.prototype.forEach ( _callback_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.foreach

use kahvi_core::prelude::*;

/// Array.prototype.forEach
pub(crate) fn prototype_for_each(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // _callback_ should be a function that accepts three arguments. `forEach` calls _callback_ once for each element present in the array, in ascending order. _callback_ is called only for elements of the array which actually exist; it is not called for missing elements of the array.
    // If a _thisArg_ parameter is provided, it will be used as the *this* value for each invocation of _callback_. If it is not provided, *undefined* is used instead.
    // _callback_ is called with three arguments: the value of the element, the index of the element, and the object being traversed.
    // `forEach` does not directly mutate the object on which it is called but the object may be mutated by the calls to _callback_.
    // The range of elements processed by `forEach` is set before the first call to _callback_. Elements which are appended to the array after the call to `forEach` begins will not be visited by _callback_. If existing elements of the array are changed, their value as passed to _callback_ will be the value at the time `forEach` visits them; elements that are deleted after the call to `forEach` begins and before being visited are not visited.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    // 4. Let _k_ be 0.
    // 5. Repeat, while _k_ < _length_,
    //    a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //    b. Let _kPresent_ be ? HasProperty(_obj_, _propertyKey_).
    //    c. If _kPresent_ is *true*, then
    //       i. Let _kValue_ be ? Get(_obj_, _propertyKey_).
    //       ii. Perform ? Call(_callback_, _thisArg_, « _kValue_, 𝔽(_k_), _obj_ »).
    //    d. Set _k_ to _k_ + 1.
    // 6. Return *undefined*.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.forEach"))
}
