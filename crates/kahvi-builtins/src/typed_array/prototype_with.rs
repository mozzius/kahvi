//! %TypedArray%.prototype.with ( _index_, _value_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.with

use kahvi_core::prelude::*;

/// TypedArray.prototype.with
pub(crate) fn prototype_with(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    // 4. Let _actualIndex_ be ? ToAbsoluteIndex(_index_, _length_).
    // 5. If _obj_.[[ContentType]] is ~bigint~, let _numericValue_ be ? ToBigInt(_value_).
    // 6. Else, let _numericValue_ be ? ToNumber(_value_).
    // 7. If IsValidIntegerIndex(_obj_, 𝔽(_actualIndex_)) is *false*, throw a *RangeError* exception.
    // 8. Let _resultArray_ be ? TypedArrayCreateSameType(_obj_, _length_).
    // 9. Let _k_ be 0.
    // 10. Repeat, while _k_ < _length_,
    //    a. If _k_ = _actualIndex_, let _fromValue_ be _numericValue_.
    //    b. Else, let _fromValue_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
    //    c. Perform ! TypedArraySetElement(_resultArray_, 𝔽(_k_), _fromValue_).
    //    d. Set _k_ to _k_ + 1.
    // 11. Return _resultArray_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.with"))
}
