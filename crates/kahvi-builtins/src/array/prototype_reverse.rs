//! Array.prototype.reverse ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.reverse

use kahvi_core::prelude::*;

/// Array.prototype.reverse
pub(crate) fn prototype_reverse(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method rearranges the elements of the array so as to reverse their order. It returns the reversed array.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. Let _middle_ be floor(_length_ / 2).
    // 4. Let _lower_ be 0.
    // 5. Repeat, while _lower_ ≠ _middle_,
    //    a. Let _upper_ be _length_ - _lower_ - 1.
    //    b. Let _upperP_ be ! ToString(𝔽(_upper_)).
    //    c. Let _lowerP_ be ! ToString(𝔽(_lower_)).
    //    d. Let _lowerExists_ be ? HasProperty(_obj_, _lowerP_).
    //    e. If _lowerExists_ is *true*, then
    //       i. Let _lowerValue_ be ? Get(_obj_, _lowerP_).
    //    f. Let _upperExists_ be ? HasProperty(_obj_, _upperP_).
    //    g. If _upperExists_ is *true*, then
    //       i. Let _upperValue_ be ? Get(_obj_, _upperP_).
    //    h. If _lowerExists_ is *true* and _upperExists_ is *true*, then
    //       i. Perform ? Set(_obj_, _lowerP_, _upperValue_, *true*).
    //       ii. Perform ? Set(_obj_, _upperP_, _lowerValue_, *true*).
    //    i. Else if _lowerExists_ is *false* and _upperExists_ is *true*, then
    //       i. Perform ? Set(_obj_, _lowerP_, _upperValue_, *true*).
    //       ii. Perform ? DeletePropertyOrThrow(_obj_, _upperP_).
    //    j. Else if _lowerExists_ is *true* and _upperExists_ is *false*, then
    //       i. Perform ? DeletePropertyOrThrow(_obj_, _lowerP_).
    //       ii. Perform ? Set(_obj_, _upperP_, _lowerValue_, *true*).
    //    k. Else,
    //       i. Assert: _lowerExists_ and _upperExists_ are both *false*.
    //       ii. NOTE: No action is required.
    //    l. Set _lower_ to _lower_ + 1.
    // 6. Return _obj_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore, it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.reverse"))
}
