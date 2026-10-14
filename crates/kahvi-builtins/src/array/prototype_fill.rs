//! Array.prototype.fill ( _value_ [ , _start_ [ , _end_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.fill

use kahvi_core::prelude::*;

/// Array.prototype.fill
pub(crate) fn prototype_fill(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // The _start_ argument is optional. If it is not provided, *+0*_𝔽 is used.
    // The _end_ argument is optional. If it is not provided, the length of the *this* value is used.
    // NOTE:
    // If _start_ is negative, it is treated as _length_ + _start_ where _length_ is the length of the array. If _end_ is negative, it is treated as _length_ + _end_.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    let obj = ops::to_object(agent, this)?;
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    let length = ops::length_of_array_like(agent, obj)?;
    let value = args.get_or_undefined(0);
    let start = args.get_or_undefined(1);
    let end = args.get_or_undefined(2);
    // The setters called by Set may collect garbage: _obj_ and _value_ stay rooted.
    agent.with_roots(&[Value::Object(obj), value], |agent| {
        // 3. Let _k_ be ? ToClampedIndex(_start_, _length_).
        let mut k = to_clamped_index(agent, start, length)?;
        // 4. If _end_ is *undefined*, let _final_ be _length_; else let _final_ be ? ToClampedIndex(_end_, _length_).
        let final_index = if matches!(end, Value::Undefined) {
            length
        } else {
            to_clamped_index(agent, end, length)?
        };
        // 5. Repeat, while _k_ < _final_,
        while k < final_index {
            // a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
            let property_key = agent.key_from_index(k);
            // b. Perform ? Set(_obj_, _propertyKey_, _value_, *true*).
            ops::set(agent, obj, property_key, value, true)?;
            // c. Set _k_ to _k_ + 1.
            k += 1.0;
        }
        // 6. Return _obj_.
        Ok(Value::Object(obj))
    })
}

/// ToClampedIndex ( _value_, _length_ )
///
/// Spec: https://tc39.es/ecma262/#sec-toclampedindex
///
/// Not yet in `ops`; only this built-in needs it so far.
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
    Ok(index.max(0.0).min(length))
}
