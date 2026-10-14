//! %TypedArray%.prototype.includes ( _searchElement_ [ , _fromIndex_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.includes

use kahvi_core::object::typed_array::typed_array_length;
use kahvi_core::prelude::*;

use crate::support::typed_array::{typed_array_get_element, validate_typed_array};

/// ToClampedIndex(_value_, _length_), which is not yet in `ops`.
///
/// Spec: https://tc39.es/ecma262/#sec-toclampedindex
fn to_clamped_index(agent: &mut Agent, value: Value, length: f64) -> JsResult<f64> {
    // 1. Let _index_ be ? ToAbsoluteIndex(_value_, _length_).
    //    ToAbsoluteIndex ( _start_, _length_ ):
    //    1. Let _relativeIndex_ be ? ToIntegerOrInfinity(_start_).
    let relative_index = ops::to_integer_or_infinity(agent, value)?;
    //    2. If _relativeIndex_ is -∞, return +0𝔽.
    //    3. Else if _relativeIndex_ < 0, return max(_length_ + _relativeIndex_, 0).
    //    4. Else, return min(_relativeIndex_, _length_).
    let index = if relative_index == f64::NEG_INFINITY {
        0.0
    } else if relative_index < 0.0 {
        (length + relative_index).max(0.0)
    } else {
        relative_index.min(length)
    };
    // 2. Return the result of clamping _index_ between 0 and _length_.
    Ok(index.clamp(0.0, length))
}

/// TypedArray.prototype.includes
pub(crate) fn prototype_includes(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // The interpretation and use of the arguments of this method are the same as for `Array.prototype.includes` as defined in [sec-array.prototype.includes].
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    let obj = this;
    // 2. Let _taRecord_ be ? ValidateTypedArray(_obj_, ~seq-cst~).
    let ta_record = validate_typed_array(agent, obj)?;
    let Value::Object(obj) = obj else {
        unreachable!("ValidateTypedArray accepted a non-object")
    };
    // 3. Let _length_ be TypedArrayLength(_taRecord_).
    let length = typed_array_length(agent, ta_record) as f64;
    // 4. If _length_ = 0, return *false*.
    if length == 0.0 {
        return Ok(Value::Boolean(false));
    }
    // 5. Let _k_ be ? ToClampedIndex(_fromIndex_, _length_).
    let mut k = to_clamped_index(agent, args.get_or_undefined(1), length)?;
    // 6. Repeat, while _k_ < _length_,
    while k < length {
        // a. Let _elementK_ be TypedArrayGetElement(_obj_, 𝔽(_k_)).
        let element_k = typed_array_get_element(agent, obj, k);
        // b. If SameValueZero(_searchElement_, _elementK_) is *true*, return *true*.
        if ops::same_value_zero(agent, args.get_or_undefined(0), element_k) {
            return Ok(Value::Boolean(true));
        }
        // c. Set _k_ to _k_ + 1.
        k += 1.0;
    }
    // 7. Return *false*.
    Ok(Value::Boolean(false))
}
