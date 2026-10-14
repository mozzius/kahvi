//! %TypedArray%.prototype.map ( _callback_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.map

use kahvi_core::prelude::*;

/// TypedArray.prototype.map
pub(crate) fn prototype_map(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.map` as defined in [sec-array.prototype.map].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    // 4. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    // 5. Let _result_ be ? TypedArraySpeciesCreate(_obj_, « 𝔽(_length_) »).
    // 6. Let _k_ be 0.
    // 7. Repeat, while _k_ < _length_,
    //    a. Let _kValue_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
    //    b. Let _mappedValue_ be ? Call(_callback_, _thisArg_, « _kValue_, 𝔽(_k_), _obj_ »).
    //    c. Perform ? TypedArraySetElement(_result_, 𝔽(_k_), _mappedValue_).
    //    d. Set _k_ to _k_ + 1.
    // 8. Return _result_.
    //
    // This method is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.map"))
}
