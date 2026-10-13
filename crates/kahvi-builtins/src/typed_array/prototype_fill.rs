//! %TypedArray%.prototype.fill ( _value_ [ , _start_ [ , _end_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.fill

use kahvi_core::prelude::*;

/// TypedArray.prototype.fill
pub(crate) fn prototype_fill(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.fill` as defined in [sec-array.prototype.fill].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    // 4. If _obj_.[[ContentType]] is ~bigint~, set _value_ to ? ToBigInt(_value_).
    // 5. Else, set _value_ to ? ToNumber(_value_).
    // 6. Let _startIndex_ be ? ToClampedIndex(_start_, _length_).
    // 7. If _end_ is *undefined*, let _endIndex_ be _length_; else let _endIndex_ be ? ToClampedIndex(_end_, _length_).
    // 8. Set _taRecord_ to ? ValidateTypedArrayBounds(_obj_, ~seq-cst~).
    // 9. Set _length_ to TypedArrayLength(_taRecord_).
    // 10. Set _endIndex_ to min(_endIndex_, _length_).
    // 11. Let _k_ be _startIndex_.
    // 12. Repeat, while _k_ < _endIndex_,
    //    a. Perform ! TypedArraySetElement(_obj_, 𝔽(_k_), _value_).
    //    b. Set _k_ to _k_ + 1.
    // 13. Return _obj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.fill"))
}
