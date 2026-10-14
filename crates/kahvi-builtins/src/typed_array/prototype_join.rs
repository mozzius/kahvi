//! %TypedArray%.prototype.join ( _separator_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.join

use kahvi_core::prelude::*;

/// TypedArray.prototype.join
pub(crate) fn prototype_join(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.join` as defined in [sec-array.prototype.join].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    // 4. If _separator_ is *undefined*, let _separatorString_ be *","*.
    // 5. Else, let _separatorString_ be ? ToString(_separator_).
    // 6. Let _result_ be the empty String.
    // 7. Let _k_ be 0.
    // 8. Repeat, while _k_ < _length_,
    //    a. If _k_ > 0, set _result_ to the string-concatenation of _result_ and _separatorString_.
    //    b. Let _element_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
    //    c. If _element_ is not *undefined*, then
    //       i. Let _elementString_ be ! ToString(_element_).
    //       ii. Set _result_ to the string-concatenation of _result_ and _elementString_.
    //    d. Set _k_ to _k_ + 1.
    // 9. Return _result_.
    //
    // This method is not generic. The *this* value must be an object with a [[TypedArrayName]] internal slot.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.join"))
}
