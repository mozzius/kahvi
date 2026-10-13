//! %TypedArray%.prototype.filter ( _callback_ [ , _thisArg_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.filter

use kahvi_core::prelude::*;

/// TypedArray.prototype.filter
pub(crate) fn prototype_filter(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.filter` as defined in [sec-array.prototype.filter].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    // 4. If IsCallable(_callback_) is *false*, throw a *TypeError* exception.
    // 5. Let _kept_ be a new empty List.
    // 6. Let _captured_ be 0.
    // 7. Let _k_ be 0.
    // 8. Repeat, while _k_ < _length_,
    //    a. Let _kValue_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
    //    b. Let _selected_ be ToBoolean(? Call(_callback_, _thisArg_, « _kValue_, 𝔽(_k_), _obj_ »)).
    //    c. If _selected_ is *true*, then
    //       i. Append _kValue_ to _kept_.
    //       ii. Set _captured_ to _captured_ + 1.
    //    d. Set _k_ to _k_ + 1.
    // 9. Let _result_ be ? TypedArraySpeciesCreate(_obj_, « 𝔽(_captured_) »).
    // 10. Let _n_ be 0.
    // 11. For each element _element_ of _kept_, do
    //    a. Perform ! TypedArraySetElement(_result_, 𝔽(_n_), _element_).
    //    b. Set _n_ to _n_ + 1.
    // 12. Return _result_.
    //
    // This method is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.filter"))
}
