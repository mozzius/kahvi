//! %TypedArray%.prototype.reverse ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.reverse

use crate::support::typed_array::{
    length, typed_array_get_element, typed_array_set_element, validate_typed_array,
};
use kahvi_core::prelude::*;

/// TypedArray.prototype.reverse
pub(crate) fn prototype_reverse(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.reverse` as defined in [sec-array.prototype.reverse].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    let obj = this;
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    let ta_record = validate_typed_array(agent, obj)?;
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    let len = length(agent, ta_record) as f64;
    // 4. Let _middle_ be floor(_length_ / 2).
    let middle = (len / 2.0).floor();
    // 5. Let _lower_ be 0.
    let mut lower = 0.0;
    // 6. Repeat, while _lower_ ≠ _middle_,
    while lower != middle {
        // a. Let _upper_ be _length_ - _lower_ - 1.
        let upper = len - lower - 1.0;
        // b. Let _lowerValue_ be TypedArrayGetElement(_obj_, 𝔽(_lower_)).
        let lower_value = typed_array_get_element(agent, ta_record.object, lower);
        // c. Let _upperValue_ be TypedArrayGetElement(_obj_, 𝔽(_upper_)).
        let upper_value = typed_array_get_element(agent, ta_record.object, upper);
        // d. Perform ! TypedArraySetElement(_obj_, 𝔽(_lower_), _upperValue_).
        typed_array_set_element(agent, ta_record.object, lower, upper_value)?;
        // e. Perform ! TypedArraySetElement(_obj_, 𝔽(_upper_), _lowerValue_).
        typed_array_set_element(agent, ta_record.object, upper, lower_value)?;
        // f. Set _lower_ to _lower_ + 1.
        lower += 1.0;
    }
    // 7. Return _obj_.
    Ok(obj)
}
