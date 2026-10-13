//! %TypedArray%.prototype.includes ( _searchElement_ [ , _fromIndex_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.includes

use kahvi_core::prelude::*;

/// TypedArray.prototype.includes
pub(crate) fn prototype_includes(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.includes` as defined in [sec-array.prototype.includes].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    // 4. If _length_ = 0, return *false*.
    // 5. Let _k_ be ? ToClampedIndex(_fromIndex_, _length_).
    // 6. Repeat, while _k_ < _length_,
    //    a. Let _elementK_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
    //    b. If SameValueZero(_searchElement_, _elementK_) is *true*, return *true*.
    //    c. Set _k_ to _k_ + 1.
    // 7. Return *false*.
    //
    // This method is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.includes"))
}
