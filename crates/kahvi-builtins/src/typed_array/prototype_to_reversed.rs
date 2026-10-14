//! %TypedArray%.prototype.toReversed ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.toreversed

use kahvi_core::prelude::*;

/// TypedArray.prototype.toReversed
pub(crate) fn prototype_to_reversed(
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
    // 4. Let _resultArray_ be ? TypedArrayCreateSameType(_obj_, _length_).
    // 5. Let _k_ be 0.
    // 6. Repeat, while _k_ < _length_,
    //    a. Let _from_ be _length_ - _k_ - 1.
    //    b. Let _fromValue_ be TypedArrayGetElement(_obj_, 𝔽(_from_)).
    //    c. Perform ! TypedArraySetElement(_resultArray_, 𝔽(_k_), _fromValue_).
    //    d. Set _k_ to _k_ + 1.
    // 7. Return _resultArray_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.toReversed"))
}
