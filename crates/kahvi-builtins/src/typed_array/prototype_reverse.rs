//! %TypedArray%.prototype.reverse ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.reverse

use kahvi_core::prelude::*;

/// TypedArray.prototype.reverse
pub(crate) fn prototype_reverse(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.reverse` as defined in [sec-array.prototype.reverse].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    // 4. Let _middle_ be floor(_length_ / 2).
    // 5. Let _lower_ be 0.
    // 6. Repeat, while _lower_ ≠ _middle_,
    //    a. Let _upper_ be _length_ - _lower_ - 1.
    //    b. Let _lowerValue_ be TypedArrayGetElement(_obj_, 𝔽(_lower_)).
    //    c. Let _upperValue_ be TypedArrayGetElement(_obj_, 𝔽(_upper_)).
    //    d. Perform ! TypedArraySetElement(_obj_, 𝔽(_lower_), _upperValue_).
    //    e. Perform ! TypedArraySetElement(_obj_, 𝔽(_upper_), _lowerValue_).
    //    f. Set _lower_ to _lower_ + 1.
    // 7. Return _obj_.
    //
    // This method is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.reverse"))
}
