//! %TypedArray%.prototype.reduceRight ( _callback_ [ , _initialValue_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.reduceright

use kahvi_core::prelude::*;

/// TypedArray.prototype.reduceRight
pub(crate) fn prototype_reduce_right(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.reduceRight` as defined in [sec-array.prototype.reduceright].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    // 4. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    // 5. If _length_ = 0 and _initialValue_ is not present, throw a *TypeError* exception.
    // 6. Let _k_ be _length_ - 1.
    // 7. Let _accumulator_ be *undefined*.
    // 8. If _initialValue_ is present, then
    //    a. Set _accumulator_ to _initialValue_.
    // 9. Else,
    //    a. Set _accumulator_ to TypedArrayGetElement(_obj_, 𝔽(_k_)).
    //    b. Set _k_ to _k_ - 1.
    // 10. Repeat, while _k_ ≥ 0,
    //    a. Let _kValue_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
    //    b. Set _accumulator_ to ? Call(_callback_, *undefined*, « _accumulator_, _kValue_, 𝔽(_k_), _obj_ »).
    //    c. Set _k_ to _k_ - 1.
    // 11. Return _accumulator_.
    //
    // This method is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.reduceRight"))
}
