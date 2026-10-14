//! Array.prototype.with ( _index_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.with

use kahvi_core::prelude::*;

/// Array.prototype.with
pub(crate) fn prototype_with(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. Let _actualIndex_ be ? ToAbsoluteIndex(_index_, _length_).
    // 4. If _actualIndex_ < 0 or _actualIndex_ ≥ _length_, throw a *RangeError* exception.
    // 5. Let _array_ be ? ArrayCreate(_length_).
    // 6. Let _k_ be 0.
    // 7. Repeat, while _k_ < _length_,
    //    a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //    b. If _k_ = _actualIndex_, let _fromValue_ be _value_.
    //    c. Else, let _fromValue_ be ? Get(_obj_, _propertyKey_).
    //    d. Perform ! CreateDataPropertyOrThrow(_array_, _propertyKey_, _fromValue_).
    //    e. Set _k_ to _k_ + 1.
    // 8. Return _array_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.with"))
}
