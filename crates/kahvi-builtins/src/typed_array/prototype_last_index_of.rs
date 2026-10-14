//! %TypedArray%.prototype.lastIndexOf ( _searchElement_ [ , _fromIndex_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.lastindexof

use kahvi_core::prelude::*;

/// TypedArray.prototype.lastIndexOf
pub(crate) fn prototype_last_index_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.lastIndexOf` as defined in [sec-array.prototype.lastindexof].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    // 4. If _length_ = 0, return *-1*_𝔽.
    // 5. If _fromIndex_ is not present, let _k_ be _length_ - 1; else let _k_ be min(? ToAbsoluteIndex(_fromIndex_, _length_), _length_ - 1).
    // 6. Repeat, while _k_ ≥ 0,
    //    a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //    b. Let _kPresent_ be ! HasProperty(_obj_, _propertyKey_).
    //    c. If _kPresent_ is *true*, then
    //       i. Let _elementK_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
    //       ii. If IsStrictlyEqual(_searchElement_, _elementK_) is *true*, return 𝔽(_k_).
    //    d. Set _k_ to _k_ - 1.
    // 7. Return *-1*_𝔽.
    //
    // This method is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.lastIndexOf"))
}
