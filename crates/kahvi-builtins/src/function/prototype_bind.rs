//! Function.prototype.bind ( _thisArg_, ..._args_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-function.prototype.bind

use kahvi_core::prelude::*;

/// Function.prototype.bind
pub(crate) fn prototype_bind(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _target_ be the *this* value.
    // 2. If IsCallable(_target_) is *false*, throw a *TypeError* exception.
    // 3. Let _boundFunc_ be ? BoundFunctionCreate(_target_, _thisArg_, _args_).
    // 4. Let _length_ be 0.
    // 5. Let _targetHasLength_ be ? HasOwnProperty(_target_, *"length"*).
    // 6. If _targetHasLength_ is *true*, then
    //    a. Let _targetLength_ be ? Get(_target_, *"length"*).
    //    b. If _targetLength_ is a Number, then
    //       i. Let _targetLengthAsInt_ be ! ToIntegerOrInfinity(_targetLength_).
    //       ii. If _targetLengthAsInt_ = +∞, then
    //          1. Set _length_ to +∞.
    //       iii. Else if _targetLengthAsInt_ = -∞, then
    //          1. Set _length_ to 0.
    //       iv. Else,
    //          1. Let _argCount_ be the number of elements in _args_.
    //          2. Set _length_ to max(_targetLengthAsInt_ - _argCount_, 0).
    // 7. Perform SetFunctionLength(_boundFunc_, _length_).
    // 8. Let _targetName_ be ? Get(_target_, *"name"*).
    // 9. If _targetName_ is not a String, set _targetName_ to the empty String.
    // 10. Perform SetFunctionName(_boundFunc_, _targetName_, *"bound"*).
    // 11. Return _boundFunc_.
    //
    // NOTE:
    // Function objects created using `Function.prototype.bind` are exotic objects. They also do not have a *"prototype"* property.
    // NOTE:
    // If _target_ is either an arrow function or a bound function exotic object, then the _thisArg_ passed to this method will not be used by subsequent calls to _func_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Function.prototype.bind"))
}
