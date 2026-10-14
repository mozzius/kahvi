//! Array.prototype.includes ( _searchElement_ [ , _fromIndex_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.includes

use kahvi_core::prelude::*;

/// Array.prototype.includes
pub(crate) fn prototype_includes(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method compares _searchElement_ to the elements of the array, in ascending order, using the SameValueZero algorithm, and if found at any position, returns *true*; otherwise, it returns *false*.
    // The optional second argument _fromIndex_ defaults to *+0*_𝔽 (i.e. the whole array is searched). If it is greater than or equal to the length of the array, *false* is returned, i.e. the array will not be searched. If it is less than *-0*_𝔽, it is used as the offset from the end of the array to compute _fromIndex_. If the computed index is less than or equal to *+0*_𝔽, the whole array will be searched.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    let obj = ops::to_object(agent, this)?;
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    let length = ops::length_of_array_like(agent, obj)?;
    // 3. If _length_ = 0, return *false*.
    if length == 0.0 {
        return Ok(Value::Boolean(false));
    }
    // 4. Let _k_ be ? ToClampedIndex(_fromIndex_, _length_).
    let mut k = to_clamped_index(agent, args.get_or_undefined(1), length)?;
    // 5. Repeat, while _k_ < _length_,
    while k < length {
        // a. Let _elementK_ be ? Get(_obj_, ! ToString(𝔽(_k_))).
        let property_key = agent.key_from_index(k);
        let element_k = ops::get(agent, obj, property_key)?;
        // b. If SameValueZero(_searchElement_, _elementK_) is *true*, return *true*.
        if ops::same_value_zero(agent, args.get_or_undefined(0), element_k) {
            return Ok(Value::Boolean(true));
        }
        // c. Set _k_ to _k_ + 1.
        k += 1.0;
    }
    // 6. Return *false*.
    Ok(Value::Boolean(false))
}

/// ToClampedIndex ( _value_, _length_ ), which is not yet in `kahvi_core::ops`.
fn to_clamped_index(agent: &mut Agent, value: Value, length: f64) -> JsResult<f64> {
    // 1. Let _index_ be ? ToAbsoluteIndex(_value_, _length_).
    // ToAbsoluteIndex ( _relative_, _length_ ) is inlined here:
    // 1. Let relativeIndex be ? ToIntegerOrInfinity(relative).
    let relative_index = ops::to_integer_or_infinity(agent, value)?;
    // 2. If relativeIndex = -∞, return +0𝔽.
    // 3. If relativeIndex < 0, return max(length + relativeIndex, 0).
    // 4. Return min(relativeIndex, length).
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
